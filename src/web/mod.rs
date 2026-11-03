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
//!
//! # The offscreen alternative (experimental)
//!
//! With `PYROQUAD_WEB_OFFSCREEN=1` in the environment at `activate_engine()`,
//! the split goes away: every command except `NextFrame` is dispatched at push
//! time, in order, on one path. That is possible because nothing is drawn to
//! the screen during Python's turn any more - "the screen" is a pyroquad-owned
//! render target (see [`OffscreenFrame`]), and macroquad's `begin_frame()` can
//! clear and reset all it likes:
//!
//! * [`CameraManager`](crate::engine::CameraManager) points the default camera,
//!   and every user camera without a render target of its own, at it.
//! * `next_frame()` flushes the pending draws into it *before* the frame
//!   ([`offscreen_flush`]), copies it to the canvas inside the frame
//!   ([`offscreen_present`]), and clears it again afterwards
//!   ([`offscreen_after_frame`]).
//!
//! `PYROQUAD_WEB_OFFSCREEN_MSAA` sets its sample count (default 4, which is what
//! the canvas' `antialias: true` gives the split path).

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use macroquad::color::BLACK;
use macroquad::miniquad;
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

    fn pq_js_is_touch_enabled() -> i32;

    fn pq_js_is_mobile() -> i32;
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

/// Whether the device supports touch input. A touchscreen laptop counts too, so
/// this says touch is available, not that it is the primary input.
pub fn is_touch_enabled() -> bool {
    unsafe { pq_js_is_touch_enabled() != 0 }
}


/// Whether we are on a mobile device.
pub fn is_mobile() -> bool {
    unsafe { pq_js_is_mobile() != 0 }
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

                    let offscreen = offscreen_active();
                    if offscreen {
                        offscreen_present();
                    }

                    mq::next_frame().await;

                    // 3d rendering is bugged if we don't clear. Belongs to the
                    // frame we just started, so it goes after the yield. The
                    // offscreen path clears in `offscreen_after_frame` instead -
                    // here it would wipe what Python already flushed into it.
                    if !offscreen {
                        mq::clear_background(BLACK);
                    }
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
// The offscreen frame
// ---------------------------------------------------------------------------

/// The render target that stands in for the canvas while Python is running.
///
/// Built from raw miniquad objects rather than `mq::render_target_ex`, for two
/// reasons. macroquad gives *every* render target a resolve attachment (even at
/// `sample_count: 1`), and miniquad resolves in `end_render_pass` - which
/// `QuadGl::draw` calls once per draw call, so that would be a full-size blit
/// per draw call. And it keeps the copy to the canvas out of macroquad's draw
/// list, so presenting touches no camera, material, viewport or depth state.
struct OffscreenFrame {
    width: u32,
    height: u32,
    sample_count: i32,
    targets: Targets,
    blit_pipeline: miniquad::Pipeline,
    blit_bindings: miniquad::Bindings,
    /// The pass that was active when [`offscreen_present`] took it away so that
    /// `end_frame()`'s UI pass lands on the canvas.
    saved_pass: Option<miniquad::RenderPass>,
}

/// The size-dependent half of [`OffscreenFrame`], rebuilt when the canvas resizes.
struct Targets {
    /// What draws are redirected into. Multisampled or not.
    pass: miniquad::RenderPass,
    /// What gets copied to the canvas: `pass`'s own color texture, or with MSAA
    /// a single-sampled texture that [`OffscreenFrame::resolve`] blits into.
    texture: miniquad::TextureId,
    /// A raw GL depth-stencil renderbuffer attached to `pass`, because miniquad
    /// 0.4.8 can do neither of the two things needed here: its `Depth` format is
    /// 16 bit (the canvas has 24 bit + stencil, and pyroquad's default
    /// `z_near`/`z_far` of 0.01/10000 z-fights badly at 16), and
    /// `new_render_pass_mrt` unwraps a multisampled depth attachment as a
    /// texture and panics.
    depth_stencil: u32,
    /// MSAA only: `(read_fb, draw_fb, resolve_pass)`. The resolve is done by
    /// hand, once per frame, for the reason in the [`OffscreenFrame`] docs.
    msaa: Option<(u32, u32, miniquad::RenderPass)>,
}

const GL_DEPTH_STENCIL: u32 = 0x84F9;
const GL_DEPTH24_STENCIL8: u32 = 0x88F0;
const GL_DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;

const BLIT_VERTEX: &str = r#"#version 100
attribute vec2 in_pos;
varying highp vec2 uv;
void main() {
    uv = in_pos * 0.5 + 0.5;
    gl_Position = vec4(in_pos, 0.0, 1.0);
}
"#;

const BLIT_FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
uniform sampler2D tex;
void main() {
    gl_FragColor = texture2D(tex, uv);
}
"#;

impl OffscreenFrame {
    fn new(ctx: &mut dyn miniquad::RenderingBackend, sample_count: i32) -> Self {
        use miniquad::*;

        // One triangle that covers clip space. The canvas and the target share
        // GL's bottom-left origin, so uv = position needs no flip.
        let vertices: [f32; 6] = [-1.0, -1.0, 3.0, -1.0, -1.0, 3.0];
        let indices: [u16; 3] = [0, 1, 2];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );
        let shader = ctx
            .new_shader(
                ShaderSource::Glsl { vertex: BLIT_VERTEX, fragment: BLIT_FRAGMENT },
                ShaderMeta {
                    uniforms: UniformBlockLayout { uniforms: vec![] },
                    images: vec!["tex".to_string()],
                },
            )
            .expect("offscreen blit shader");
        let blit_pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[VertexAttribute::new("in_pos", VertexFormat::Float2)],
            shader,
            PipelineParams::default(),
        );

        let (width, height) = physical_screen_size();
        let targets = Targets::new(ctx, width, height, sample_count);
        let texture = targets.texture;

        Self {
            width,
            height,
            sample_count,
            targets,
            blit_pipeline,
            blit_bindings: Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![texture],
            },
            saved_pass: None,
        }
    }

    /// Follows the canvas. Returns the old pass if it was replaced.
    fn fit_to_screen(&mut self, ctx: &mut dyn miniquad::RenderingBackend) -> Option<miniquad::RenderPass> {
        let (width, height) = physical_screen_size();
        if (width, height) == (self.width, self.height) {
            return None;
        }
        let old = self.targets.pass;
        self.targets.delete(ctx);
        self.targets = Targets::new(ctx, width, height, self.sample_count);
        self.width = width;
        self.height = height;
        self.blit_bindings.images[0] = self.targets.texture;
        Some(old)
    }

    fn resolve(&self) {
        use macroquad::miniquad::gl::*;

        if let Some((read_fb, draw_fb, _)) = self.targets.msaa {
            let (w, h) = (self.width as i32, self.height as i32);
            unsafe {
                glBindFramebuffer(GL_READ_FRAMEBUFFER, read_fb);
                glBindFramebuffer(GL_DRAW_FRAMEBUFFER, draw_fb);
                glBlitFramebuffer(0, 0, w, h, 0, 0, w, h, GL_COLOR_BUFFER_BIT, GL_NEAREST);
                // miniquad leaves the default framebuffer bound between passes
                // and binds explicitly in begin_pass, so this is all it expects.
                glBindFramebuffer(GL_FRAMEBUFFER, 0);
            }
        }
    }

    fn clear(&self, ctx: &mut dyn miniquad::RenderingBackend) {
        let c = BLACK;
        ctx.begin_pass(Some(self.targets.pass), miniquad::PassAction::clear_color(c.r, c.g, c.b, c.a));
        ctx.end_render_pass();
    }
}

impl Targets {
    fn new(ctx: &mut dyn miniquad::RenderingBackend, width: u32, height: u32, sample_count: i32) -> Self {
        use macroquad::miniquad::gl::*;
        use miniquad::*;

        // Nearest: the copy to the canvas is exactly 1:1.
        let color_params = TextureParams {
            width,
            height,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            ..Default::default()
        };
        let color = ctx.new_render_texture(TextureParams { sample_count, ..color_params });
        let pass = ctx.new_render_pass(color, None);
        let pass_fb = framebuffer_of(ctx, pass);

        let mut depth_stencil: u32 = 0;
        unsafe {
            glGenRenderbuffers(1, &mut depth_stencil);
            glBindRenderbuffer(GL_RENDERBUFFER, depth_stencil);
            if sample_count > 1 {
                glRenderbufferStorageMultisample(
                    GL_RENDERBUFFER, sample_count, GL_DEPTH24_STENCIL8, width as i32, height as i32,
                );
            } else {
                // The WebGL1 spelling; WebGL2 maps it to DEPTH24_STENCIL8.
                glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH_STENCIL, width as i32, height as i32);
            }
            glBindFramebuffer(GL_FRAMEBUFFER, pass_fb);
            glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_RENDERBUFFER, depth_stencil);
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }

        if sample_count <= 1 {
            return Self { pass, texture: color, depth_stencil, msaa: None };
        }

        let resolved = ctx.new_render_texture(color_params);
        let resolve_pass = ctx.new_render_pass(resolved, None);
        let draw_fb = framebuffer_of(ctx, resolve_pass);
        Self { pass, texture: resolved, depth_stencil, msaa: Some((pass_fb, draw_fb, resolve_pass)) }
    }

    fn delete(&mut self, ctx: &mut dyn miniquad::RenderingBackend) {
        // Deleting a pass deletes the attachments miniquad knows about.
        ctx.delete_render_pass(self.pass);
        if let Some((_, _, resolve_pass)) = self.msaa.take() {
            ctx.delete_render_pass(resolve_pass);
        }
        unsafe { macroquad::miniquad::gl::glDeleteRenderbuffers(1, &self.depth_stencil) };
    }
}

/// miniquad does not expose a pass' GL framebuffer, but binds it in
/// `begin_pass`.
fn framebuffer_of(ctx: &mut dyn miniquad::RenderingBackend, pass: miniquad::RenderPass) -> u32 {
    use macroquad::miniquad::gl::*;

    let mut fb: i32 = 0;
    ctx.begin_pass(Some(pass), miniquad::PassAction::Nothing);
    unsafe { glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut fb) };
    ctx.end_render_pass();
    fb as u32
}

/// The canvas' backing store in physical pixels - i.e. `high_dpi` already
/// applied, see `pq_js_setup_canvas_size` in `shim.c`.
fn physical_screen_size() -> (u32, u32) {
    let (w, h) = miniquad::window::screen_size();
    ((w as u32).max(1), (h as u32).max(1))
}

thread_local! {
    static OFFSCREEN: RefCell<Option<OffscreenFrame>> = const { RefCell::new(None) };
}

fn with_offscreen<R>(f: impl FnOnce(&mut OffscreenFrame) -> R) -> Option<R> {
    OFFSCREEN.with(|cell| cell.borrow_mut().as_mut().map(f))
}

/// Whether the offscreen path is running, i.e. whether every command is
/// dispatched at push time. Only ever turns on in [`offscreen_begin`].
pub fn offscreen_active() -> bool {
    OFFSCREEN.with(|cell| cell.borrow().is_some())
}

fn env_flag(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| v != "0" && !v.is_empty())
}

/// The sample count `PYROQUAD_WEB_OFFSCREEN` asks for, or `None` for the split
/// path.
fn offscreen_requested() -> Option<i32> {
    if !env_flag("PYROQUAD_WEB_OFFSCREEN") {
        return None;
    }
    let samples = std::env::var("PYROQUAD_WEB_OFFSCREEN_MSAA")
        .ok()
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(4);
    Some(samples.max(1))
}

/// Whether `activate_engine()` should ask for a WebGL2 context.
///
/// macroquad defaults to WebGL1, where the canvas gets its antialiasing from
/// `antialias: true` but a render target cannot be multisampled at all
/// (`renderbufferStorageMultisample` and `blitFramebuffer` are WebGL2-only - on
/// WebGL1 the call throws in JS and takes the module down with it). So an MSAA
/// offscreen target implies WebGL2. `PYROQUAD_WEB_WEBGL2` forces it for either
/// path, which is what makes the split path comparable.
pub fn wants_webgl2() -> bool {
    env_flag("PYROQUAD_WEB_WEBGL2") || offscreen_requested().is_some_and(|samples| samples > 1)
}

/// Called once by `activate_engine()`, after the first frame. Turns the
/// offscreen path on if `PYROQUAD_WEB_OFFSCREEN` asks for it.
pub fn offscreen_begin() {
    let Some(sample_count) = offscreen_requested() else {
        return;
    };

    let frame = {
        let ctx = unsafe { mq::get_internal_gl() }.quad_context;
        let frame = OffscreenFrame::new(ctx, sample_count);
        frame.clear(ctx);
        frame
    };
    // Nothing has set a camera yet, so the active pass is the canvas.
    unsafe { mq::get_internal_gl() }.quad_gl.render_pass(Some(frame.targets.pass));
    OFFSCREEN.with(|cell| *cell.borrow_mut() = Some(frame));
}

/// Where the screen is while Python is running. `CameraManager` calls this
/// after every camera change that targets the screen.
pub fn redirect_screen_pass() {
    if let Some(pass) = with_offscreen(|frame| frame.targets.pass) {
        unsafe { mq::get_internal_gl() }.quad_gl.render_pass(Some(pass));
    }
}

/// Before the frame: puts everything drawn this turn into the offscreen target.
/// `begin_frame()` would otherwise drop it, since `QuadGl::reset` forgets
/// unflushed draw calls.
pub fn offscreen_flush() {
    // push + pop is the public way to `perform_render_passes()` with the current
    // camera and no state change.
    crate::engine::CameraManager::push_camera_state();
    crate::engine::CameraManager::pop_camera_state();
}

/// Inside the frame, from `NextFrame`: copies the offscreen target onto the
/// canvas.
fn offscreen_present() {
    // Anything dispatched inside this frame (only commands queued before the
    // target existed) still has to reach it before it is copied.
    offscreen_flush();

    let gl = unsafe { mq::get_internal_gl() };
    let saved = gl.quad_gl.get_active_render_pass();
    // `end_frame()` draws macroquad's UI with the active pass; it belongs on the
    // canvas, as it would be natively with the default camera.
    gl.quad_gl.render_pass(None);

    with_offscreen(|frame| {
        frame.saved_pass = saved;
        frame.resolve();

        let ctx = gl.quad_context;
        ctx.begin_default_pass(miniquad::PassAction::Nothing);
        ctx.apply_pipeline(&frame.blit_pipeline);
        ctx.apply_bindings(&frame.blit_bindings);
        ctx.draw(0, 3, 1);
        ctx.end_render_pass();
    });
}

/// After the frame, before Python's next turn: puts the camera back on the
/// offscreen target (resized to the canvas if that changed) and clears it.
pub fn offscreen_after_frame() {
    let gl = unsafe { mq::get_internal_gl() };
    let ctx = gl.quad_context;

    let Some((saved, old_pass, pass)) = with_offscreen(|frame| {
        let old_pass = frame.fit_to_screen(&mut *ctx);
        (frame.saved_pass.take(), old_pass, frame.targets.pass)
    }) else {
        return;
    };

    // A camera state pushed before a resize still names the old pass; that one
    // cannot be fixed from here.
    let restored = match saved {
        Some(p) if Some(p) == old_pass => Some(pass),
        other => other,
    };
    gl.quad_gl.render_pass(restored);

    // Natively `begin_frame()` clears the canvas and the engine then clears the
    // active pass. Here the offscreen target plays the canvas.
    if restored != Some(pass) {
        with_offscreen(|frame| frame.clear(&mut *ctx));
    }
    mq::clear_background(BLACK);
}

/// `get_screen_data()` for the offscreen path: the canvas itself only holds a
/// finished frame inside `frame_now()`, so read the target instead. Rows come
/// out bottom-up, exactly like `mq::get_screen_data()`.
pub fn offscreen_screen_data() -> Option<mq::Image> {
    offscreen_flush();
    with_offscreen(|frame| {
        frame.resolve();
        let ctx = unsafe { mq::get_internal_gl() }.quad_context;
        let mut bytes = vec![0u8; frame.width as usize * frame.height as usize * 4];
        ctx.texture_read_pixels(frame.targets.texture, &mut bytes);
        mq::Image { bytes, width: frame.width as u16, height: frame.height as u16 }
    })
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
