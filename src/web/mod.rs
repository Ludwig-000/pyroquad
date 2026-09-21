//! Emscripten / Pyodide runtime glue.
//!
//! This module only exists on `wasm32-unknown-emscripten`. It is the Rust half
//! of [`shim.c`](shim.c) plus the single-threaded replacement for pyroquad's
//! engine thread.
//!
//! # Why the threading model has to change here
//!
//! Natively, [`activate_engine`](crate::py_abstractions::py_functions::activate_engine)
//! spawns an OS thread that owns the macroquad window and drains
//! [`COMMAND_QUEUE`](crate::engine::CoreLoop::COMMAND_QUEUE), while the Python
//! thread blocks on a [`PChannel`](crate::engine::PChannel) whenever it needs an
//! answer. Pyodide has no threads: CPython, the engine and the browser's event
//! loop all share one.
//!
//! The replacement has two halves:
//!
//! * **Frames are pulled, not pushed.** `pq_set_frame_driver(1)` stops
//!   miniquad's `requestAnimationFrame` pump, and Python's `next_frame()` calls
//!   [`frame_now`] to run exactly one miniquad frame on its own stack. So the
//!   engine can never advance while Python is halfway through a frame.
//! * **Blocking becomes stack switching.** [`frame_yield`] hands control back to
//!   the browser from the middle of synchronous Python via JSPI
//!   (`pyodide.ffi.run_sync`), so the page paints and delivers input, and then
//!   resumes exactly where it left off.
//!
//! # Why commands are split in two
//!
//! macroquad brackets every frame with `begin_frame()` / `end_frame()`, and
//! `begin_frame()` resets the draw list. Anything that draws must therefore run
//! *inside* [`frame_now`]. But a command that owes Python an answer cannot wait
//! for the next frame - Python is blocked on it right now, so the frame would
//! never come.
//!
//! So [`Command::is_frame_local`] splits them: drawing and render state stay in
//! `COMMAND_QUEUE` and are executed by [`engine_loop`] inside a frame; everything
//! else (queries, resource creation, object/physics mutation, audio, file loads)
//! is executed immediately by [`run_query_now`] at push time, off the Python
//! stack. Ordering is preserved for everything Python can observe, because an
//! immediate command runs strictly before any command Python queues after it.

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use macroquad::color::BLACK;
use macroquad::prelude as mq;
use pyo3::prelude::*;

use crate::engine::CoreLoop::{dispatch, COMMAND_QUEUE, Command, EngineState};

// ---------------------------------------------------------------------------
// shim.c
// ---------------------------------------------------------------------------

unsafe extern "C" {
    /// Publishes miniquad's entry points to JS and builds `globalThis.__PQ`.
    fn pq_platform_init();
    /// `1` = stop the rAF pump; frames only happen when [`frame_now`] asks.
    fn pq_set_frame_driver(manual: i32);
    /// `0` = let the Python loop run flat out instead of at the display refresh.
    fn pq_set_vsync(on: i32);
    /// Runs one miniquad frame synchronously on the caller's stack.
    fn pq_frame_now();
    /// `1` when the page looks usable (has `#glcanvas`, has Emscripten's `GL`).
    fn pq_probe() -> i32;
}

pub fn platform_init() {
    unsafe { pq_platform_init() }
}

pub fn set_frame_driver_manual(manual: bool) {
    unsafe { pq_set_frame_driver(manual as i32) }
}

/// Paces [`frame_yield`]: `true` waits for an animation frame, `false` only for
/// the task queue.
///
/// This is where `Config.swap_interval == Some(0)` lands in the browser. The
/// native backends hand the swap interval to the driver; WebGL has no such knob,
/// so on the web the loop's own yield is the only place a frame cap exists.
pub fn set_vsync(on: bool) {
    unsafe { pq_set_vsync(on as i32) }
}

/// One miniquad frame: `begin_frame` -> poll [`engine_loop`] -> `end_frame`.
pub fn frame_now() {
    unsafe { pq_frame_now() }
}

/// Whether the page can host the engine at all.
pub fn page_is_ready() -> bool {
    unsafe { pq_probe() != 0 }
}

// ---------------------------------------------------------------------------
// Yielding to the browser from synchronous Python (JSPI)
// ---------------------------------------------------------------------------

const YIELD_HELPERS: &std::ffi::CStr = cr#"
import js
from pyodide.ffi import run_sync

_PQ = getattr(js, "__PQ")

def frame_yield():
    run_sync(_PQ.frame_yield())

def task_yield():
    run_sync(_PQ.task_yield())
"#;

thread_local! {
    static YIELD_MODULE: RefCell<Option<Py<PyAny>>> = const { RefCell::new(None) };
}

fn yield_helper(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    YIELD_MODULE.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            let module = PyModule::from_code(
                py,
                YIELD_HELPERS,
                c"_pyroquad_yield.py",
                c"_pyroquad_yield",
            )?;
            *slot = Some(module.into_any().unbind());
        }
        slot.as_ref().unwrap().getattr(py, name)
    })
}

/// Hands the browser one animation frame, then resumes.
///
/// Needs JSPI (`WebAssembly.Suspending`), which is what `pyodide.ffi.run_sync`
/// is built on. Without it the browser can never paint while Python is running
/// and there is no way to fix that from inside the module, so the error is
/// reported as-is rather than papered over.
pub fn frame_yield(py: Python<'_>) -> PyResult<()> {
    yield_helper(py, "frame_yield")?.call0(py)?;
    Ok(())
}

/// Hands the browser the rest of the current task, then resumes.
pub fn task_yield(py: Python<'_>) -> PyResult<()> {
    yield_helper(py, "task_yield")?.call0(py)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// A minimal executor
// ---------------------------------------------------------------------------

fn noop_waker() -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }
    unsafe fn noop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

/// Polls `future` once. `None` means it is not finished yet.
fn poll_once<F: Future + ?Sized>(future: &mut Pin<Box<F>>) -> Option<F::Output> {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => Some(value),
        Poll::Pending => None,
    }
}

/// Drives `future` to completion, giving the browser a turn between polls.
///
/// The only commands that need this are the asset loads (`load_file`,
/// `load_sound`, `load_ttf_font`, `load_image`), which are waiting on a `fetch`
/// that can only make progress once we return to the event loop. Everything
/// else finishes on the first poll.
fn block_on<F: Future + ?Sized>(mut future: Pin<Box<F>>) -> F::Output {
    loop {
        if let Some(value) = poll_once(&mut future) {
            return value;
        }
        let _ = Python::attach(task_yield);
    }
}

// ---------------------------------------------------------------------------
// Engine state
// ---------------------------------------------------------------------------

thread_local! {
    /// Natively this lives on [`crate::engine::CoreLoop::proccess_commands_loop`]'s
    /// stack. Here two different call paths dispatch commands - [`engine_loop`]
    /// inside a frame, and [`run_query_now`] off the Python stack - so it has to
    /// be shared.
    ///
    /// The borrow is held for one command at a time. An asset load does hold it
    /// across browser turns (see [`block_on`]), which is safe because Python is
    /// blocked inside that very call and only DOM/fetch callbacks - which never
    /// touch this - can run meanwhile. If that ever stops being true the
    /// `borrow_mut` below panics, which is the intended failure mode.
    static ENGINE_STATE: RefCell<EngineState> = RefCell::new(EngineState::new());
}

fn with_state<R>(f: impl FnOnce(&mut EngineState) -> R) -> R {
    ENGINE_STATE.with(|cell| f(&mut cell.borrow_mut()))
}

// ---------------------------------------------------------------------------
// The two dispatch paths
// ---------------------------------------------------------------------------

/// The engine's main future. Replaces the native
/// [`proccess_commands_loop`](crate::engine::CoreLoop::proccess_commands_loop)
/// body when running in the browser.
///
/// It is polled by macroquad once per [`frame_now`], between `begin_frame()` and
/// `end_frame()`. Each poll executes everything Python queued since the last
/// frame and then suspends again on `Command::NextFrame`, which is what makes
/// macroquad flush and present.
pub async fn engine_loop() {
    loop {
        while let Some(command) = COMMAND_QUEUE.pop() {
            match command {
                Command::NextFrame { physics_step, sender } => {
                    // End-of-frame bookkeeping happens *before* the yield,
                    // because the yield is what ends the macroquad frame and
                    // returns control to Python - which must find its reply
                    // already waiting. (Natively the order is reversed, since
                    // there the engine thread keeps running afterwards.)
                    crate::engine::SHADERS::shader_manager::new_frame_shader_update();
                    crate::engine::FrameInfo::update_frame_info();

                    if let Some(step) = physics_step {
                        with_state(|state| state.object_storage.step_physics(step));
                    }

                    let _ = sender.send(());

                    mq::next_frame().await;

                    // 3d rendering is bugged if we don't clear. Belongs to the
                    // frame we just started, so it goes after the yield.
                    mq::clear_background(BLACK);
                }

                other => {
                    // Everything still in this queue is frame-local, and
                    // `NextFrame` is the only frame-local command that awaits.
                    with_state(|state| {
                        let mut future = Box::pin(dispatch(other, state));
                        assert!(
                            poll_once(&mut future).is_some(),
                            "a frame-local command tried to await; it belongs on                              the query path (Command::is_frame_local)",
                        );
                    });
                }
            }
        }

        // Nothing queued. Never spin here - this is the browser's only thread.
        mq::next_frame().await;
    }
}

/// Executes a command that owes Python an answer, immediately, off the Python
/// stack rather than inside a frame. See the module docs for why.
pub fn run_query_now(command: Command) {
    with_state(|state| block_on(Box::pin(dispatch(command, state))));
}

// ---------------------------------------------------------------------------
// HTTP
// ---------------------------------------------------------------------------

/// Starts a download and returns immediately; `on_done` runs later, from the
/// browser's event loop.
///
/// This is miniquad's own wasm file loader, which `src/web/shim.c` implements
/// with `fetch`. Going through it rather than `ehttp`/`reqwest` means downloads
/// are genuinely concurrent - a hundred of them can be in flight at once, which
/// is what the example game's asset loader expects - and that completion
/// arrives as a plain callback on this thread, with nothing to synchronise.
pub fn fetch<F: Fn(Result<Vec<u8>, String>) + 'static>(url: &str, on_done: F) {
    let url_for_error = url.to_string();
    macroquad::miniquad::fs::load_file(url, move |response| {
        on_done(response.map_err(|e| format!("failed to download {url_for_error}: {e}")));
    });
}

/// [`fetch`], but waits for the answer - by suspending this Python stack through
/// JSPI, not by blocking the thread, so the `fetch` can actually progress.
pub fn fetch_blocking(url: &str) -> Result<Vec<u8>, String> {
    use std::rc::Rc;

    let slot: Rc<RefCell<Option<Result<Vec<u8>, String>>>> = Rc::new(RefCell::new(None));
    let sink = Rc::clone(&slot);
    fetch(url, move |result| {
        *sink.borrow_mut() = Some(result);
    });

    loop {
        if let Some(result) = slot.borrow_mut().take() {
            return result;
        }
        Python::attach(task_yield)
            .map_err(|e| format!("could not yield to the browser while downloading: {e}"))?;
    }
}
