//! macOS only: the engine shares the main thread with Python.
//!
//! AppKit only allows windows on the process's main thread: creating them and handling their
//! events. Everywhere else the engine gets a thread of its own (see `activate_engine`), but on
//! macOS that thread would abort the process with `NSInternalInconsistencyException`. So here the
//! engine runs on the main thread, on a stack of its own (a stackful coroutine), and takes turns
//! with Python: whenever Python waits for the engine (see `PChannel`), the engine gets the thread
//! and works through `COMMAND_QUEUE`. Once the queue is empty (`idle`), it hands the thread back.
//! AppKit only checks which thread it runs on, not which stack.
//!
//! The engine runs with the GIL released, like its thread elsewhere. That also keeps Python code
//! off the engine's stack: Python objects the engine drops are only released later, by pyo3,
//! once a thread holds the GIL again.

use std::cell::{Cell, RefCell};
use std::mem::ManuallyDrop;
use std::os::raw::c_int;
use std::ptr;

use corosensei::stack::DefaultStack;
use corosensei::{Coroutine, CoroutineResult, Yielder};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Stack of the engine coroutine: what a main thread gets. The engine thread elsewhere gets
/// Rust's default of 2 MiB.
const STACK_SIZE: usize = 8 * 1024 * 1024;

unsafe extern "C" {
    // libSystem
    fn pthread_main_np() -> c_int;
    // CPython; not in the limited API, but present in every CPython. On macOS, extension
    // modules look up interpreter symbols when they are loaded.
    fn PyGILState_Check() -> c_int;
}

type Engine = Coroutine<(), (), ()>;

thread_local! {
    /// The engine, on the main thread; empty everywhere else.
    /// Never dropped: dropping a suspended coroutine unwinds its stack, through frames of
    /// miniquad and AppKit that can't be unwound, and that would abort the process on exit.
    static ENGINE: ManuallyDrop<RefCell<Option<Engine>>> = const { ManuallyDrop::new(RefCell::new(None)) };
    /// Inside the engine: how `idle` hands the thread back.
    static YIELDER: Cell<*const Yielder<(), ()>> = const { Cell::new(ptr::null()) };
    /// True while the engine has the thread. It must not resume itself.
    static IN_ENGINE: Cell<bool> = const { Cell::new(false) };
}

/// Starts `engine` - the body of the engine thread elsewhere - on this thread, which has to be
/// the main thread. Returns once the engine is set up and waits for commands, or has stopped.
pub fn start(engine: impl FnOnce() + 'static) -> PyResult<()> {
    if unsafe { pthread_main_np() } == 0 {
        return Err(PyRuntimeError::new_err(
            "On macOS, activate_engine() has to be called from the main thread: \
             macOS only allows windows there.",
        ));
    }

    // AppKit reports misuse by throwing Objective-C exceptions. If one unwinds into a
    // catch_unwind, Rust aborts with "cannot catch foreign exceptions" and the exception's
    // name and reason are lost - so print every one the moment it is thrown.
    crate::engine::MacExceptionLog::install();
    // Macs without GPU access (VMs) only have a software OpenGL renderer, which miniquad won't take.
    crate::engine::MacSoftwareGL::install();

    // corosensei copies the closure onto the new stack and refuses anything over 1 KiB. The
    // engine carries the whole window config, icons included, so hand it over boxed.
    let engine: Box<dyn FnOnce()> = Box::new(engine);

    let stack = DefaultStack::new(STACK_SIZE).map_err(|e| {
        PyRuntimeError::new_err(format!("Could not allocate the engine's stack: {e}"))
    })?;
    let coroutine = Coroutine::with_stack(stack, move |yielder: &Yielder<(), ()>, ()| {
        YIELDER.set(yielder);
        engine();
        YIELDER.set(ptr::null());
    });
    ENGINE.with(|slot| *slot.borrow_mut() = Some(coroutine));

    // the first turn creates the window and sets the engine up
    run_engine();
    Ok(())
}

/// Gives the engine a turn: it runs until the command queue is empty, which includes drawing a
/// frame if `next_frame()` asked for one. Returns false - without doing anything - when there is
/// no engine to run here: on any thread but the main thread, inside the engine itself, before
/// `activate_engine()` and after the window was closed.
pub fn run_engine() -> bool {
    if IN_ENGINE.get() {
        return false;
    }
    ENGINE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let Some(engine) = slot.as_mut() else {
            return false;
        };
        IN_ENGINE.set(true);
        let result = without_gil(|| engine.resume(()));
        IN_ENGINE.set(false);
        if let CoroutineResult::Return(()) = result {
            // the window is gone, and the engine with it
            *slot = None;
        }
        true
    })
}

/// Called by the command loop whenever the queue is empty: hands the main thread back to
/// whoever gave the engine its turn.
pub fn idle() {
    let yielder = YIELDER.get();
    if !yielder.is_null() {
        unsafe { &*yielder }.suspend(());
    }
}

/// Runs `f` with the GIL released, if this thread holds it.
///
/// Waiting for the engine while holding the GIL would deadlock here: the main thread has to run
/// Python to give the engine a turn at all.
pub fn without_gil<R>(f: impl FnOnce() -> R) -> R {
    if unsafe { PyGILState_Check() } == 0 {
        return f();
    }
    let f = AssertSend(f);
    Python::attach(|py| py.detach(move || AssertSend(f.into_inner()()))).into_inner()
}

/// `Python::detach` wants a `Send` closure, so that it can't carry Python references. Ours carry
/// none, only references to the engine or to a channel, used on this very thread.
struct AssertSend<T>(T);
unsafe impl<T> Send for AssertSend<T> {}
impl<T> AssertSend<T> {
    // a method, so that closures capture the whole wrapper rather than its field
    fn into_inner(self) -> T {
        self.0
    }
}
