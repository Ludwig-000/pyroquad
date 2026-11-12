use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use macroquad::{miniquad::conf::Icon, prelude as mq};

use crate::_pyroquad::FilterMode;



#[pyclass(from_py_object)]
#[derive(Clone, PartialEq, Debug)]
pub struct Config {

    /// Name of the window which the engine runs in.
    #[pyo3(get, set)]
    pub window_title: String,

    /// width in pixels. "fullscreen=True" will overwrite this.
    #[pyo3(get, set)] 
    pub window_width: i32,

    /// height in pixels. "fullscreen=True" will overwrite this.
    #[pyo3(get, set)] 
    pub window_height: i32,

    /// False -> creates a window with the before above selected width and heigt.
    /// True -> creates a "windowed fullscreen" window with a resultion equal to the monitor resolution.
    #[pyo3(get, set)] 
    pub fullscreen: bool,
    
    /// Optional swap interval (vertical sync).
    /// 
    /// Set to 0, the framerate will be uncapped.
    ///
    /// Note that this is highly platform- and driver-dependent.
    /// There is no guarantee the FPS will match the specified `swap_interval`.
    /// In other words, `swap_interval` is only a hint to the GPU driver and
    /// not a reliable way to limit the game's FPS.
    #[pyo3(get, set)] 
    pub swap_interval: Option<i32>,

    #[pyo3(get, set)]
    pub sample_count: i32,

    #[pyo3(get, set)] 
    pub window_resizable: bool,

    /// once the window gets closed ( not minimized ) the python script gets terminated.
    #[pyo3(get, set)] 
    pub stop_python_when_closing_window: bool,

    /// max number of verices a single mesh can have.
    #[pyo3(get, set)] 
    pub draw_call_vertex_capacity: usize,
    
    /// max number of indices a single mesh can have.
    #[pyo3(get, set)] 
    pub draw_call_index_capacity: usize,

    #[pyo3(get, set)] 
    pub default_filter_mode: FilterMode,

    #[pyo3(get, set)]
    pub high_dpi: bool,

    /// Linux-only window settings. Ignored on every other platform.
    /// Reading `config.linux` returns a copy, so assign a whole `LinuxConfig` to change it.
    #[pyo3(get, set)]
    pub linux: LinuxConfig,
}


/// Window settings that only matter on Linux. Ignored on every other platform.
#[pyclass(from_py_object)]
#[derive(Clone, PartialEq, Debug)]
pub struct LinuxConfig {
    /// Which window system to use (X11 or Wayland).
    /// The default prefers X11 (XWayland on Wayland desktops) and only falls back to
    /// native Wayland when no X server is available.
    #[pyo3(get, set)]
    pub backend: LinuxBackend,

    /// X11 only: how the OpenGL context is loaded (GLX or EGL).
    #[pyo3(get, set)]
    pub x11_gl: LinuxX11Gl,

    /// Wayland only: how to draw the window's title bar when the compositor
    /// doesn't draw one itself (e.g. GNOME).
    #[pyo3(get, set)]
    pub wayland_decorations: WaylandDecorations,

    /// Sets `WM_CLASS` on X11 and `app_id` on Wayland.
    /// Desktops use it (together with a `.desktop` file) to group windows and pick their icon.
    #[pyo3(get, set)]
    pub wm_class: String,
}

#[pymethods]
impl LinuxConfig {
    #[new]
    #[pyo3(signature = (
        backend = LinuxBackend::X11WithWaylandFallback,
        x11_gl = LinuxX11Gl::GLXWithEGLFallback,
        wayland_decorations = WaylandDecorations::ServerWithLibDecorFallback,
        wm_class = "pyroquad".to_string(),
    ))]
    pub const fn new(
        backend: LinuxBackend,
        x11_gl: LinuxX11Gl,
        wayland_decorations: WaylandDecorations,
        wm_class: String,
    ) -> Self {
        LinuxConfig { backend, x11_gl, wayland_decorations, wm_class }
    }
}

impl Default for LinuxConfig {
    fn default() -> Self {
        LinuxConfig::new(
            LinuxBackend::X11WithWaylandFallback,
            LinuxX11Gl::GLXWithEGLFallback,
            WaylandDecorations::ServerWithLibDecorFallback,
            "pyroquad".to_string(),
        )
    }
}


/// Linux only: which window system the engine uses.
#[pyclass(from_py_object, eq)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinuxBackend {
    /// Use only X11. Fails to start if no X server (or XWayland) is available.
    X11Only,
    /// Use only native Wayland. Fails to start if Wayland is unavailable.
    WaylandOnly,
    /// Prefer X11, fall back to native Wayland if X11 is unavailable. (default)
    X11WithWaylandFallback,
    /// Prefer native Wayland, fall back to X11 if Wayland is unavailable.
    WaylandWithX11Fallback,
}

/// Linux/X11 only: which library loads the OpenGL context.
#[pyclass(from_py_object, eq)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinuxX11Gl {
    /// Use only libGLX.
    GLXOnly,
    /// Use only libEGL.
    EGLOnly,
    /// Prefer libGLX, fall back to libEGL. (default)
    GLXWithEGLFallback,
    /// Prefer libEGL, fall back to libGLX.
    EGLWithGLXFallback,
}

/// Linux/Wayland only: how to draw the title bar when the compositor doesn't (e.g. GNOME).
#[pyclass(from_py_object, eq)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WaylandDecorations {
    /// Try to load libdecor to draw a proper title bar. (default)
    ServerWithLibDecorFallback,
    /// Draw a simple light gray border.
    ServerWithMiniquadFallback,
    /// Draw nothing.
    ServerOnly,
}

impl From<LinuxBackend> for mq::miniquad::conf::LinuxBackend {
    fn from(v: LinuxBackend) -> Self {
        match v {
            LinuxBackend::X11Only => Self::X11Only,
            LinuxBackend::WaylandOnly => Self::WaylandOnly,
            LinuxBackend::X11WithWaylandFallback => Self::X11WithWaylandFallback,
            LinuxBackend::WaylandWithX11Fallback => Self::WaylandWithX11Fallback,
        }
    }
}

impl From<LinuxX11Gl> for mq::miniquad::conf::LinuxX11Gl {
    fn from(v: LinuxX11Gl) -> Self {
        match v {
            LinuxX11Gl::GLXOnly => Self::GLXOnly,
            LinuxX11Gl::EGLOnly => Self::EGLOnly,
            LinuxX11Gl::GLXWithEGLFallback => Self::GLXWithEGLFallback,
            LinuxX11Gl::EGLWithGLXFallback => Self::EGLWithGLXFallback,
        }
    }
}

impl From<WaylandDecorations> for mq::miniquad::conf::WaylandDecorations {
    fn from(v: WaylandDecorations) -> Self {
        match v {
            WaylandDecorations::ServerWithLibDecorFallback => Self::ServerWithLibDecorFallback,
            WaylandDecorations::ServerWithMiniquadFallback => Self::ServerWithMiniquadFallback,
            WaylandDecorations::ServerOnly => Self::ServerOnly,
        }
    }
}


#[pymethods]
impl Config {
    #[new]
    #[pyo3(signature = (
        window_title = "".to_string(),
        window_width= 800,
        window_height= 600,
        fullscreen= false,
        swap_interval= None,
        sample_count= 1,
        window_resizable= true,
        stop_python_when_closing_window= true,
        draw_call_vertex_capacity= 10_000,
        draw_call_index_capacity= 5_000,
        default_filter_mode= FilterMode::Linear,
        high_dpi= true,
        linux= LinuxConfig::default(),
    ))]
    pub const fn new(
        window_title: String,
        window_width: i32,
        window_height: i32,
        fullscreen: bool,
        swap_interval: Option<i32>,
        sample_count: i32,
        window_resizable: bool,
        stop_python_when_closing_window: bool,
        draw_call_vertex_capacity: usize,
        draw_call_index_capacity: usize,
        default_filter_mode: FilterMode,
        high_dpi: bool,
        linux: LinuxConfig,
    ) -> Self {
        Config {
            window_title,
            window_width,
            window_height,
            fullscreen,
            swap_interval,
            sample_count,
            window_resizable,
            stop_python_when_closing_window,
            draw_call_vertex_capacity,
            draw_call_index_capacity,
            default_filter_mode,
            high_dpi,
            linux,
        }
    }
}



impl Config{
    pub fn to_window_config(config: Config) -> macroquad::conf::Conf {
       const im16: [u8; 4 * 16 * 16] = *include_bytes!("../../PyroquadLogo16.raw");
       const im32: [u8; 4 * 32 * 32] = *include_bytes!("../../PyroquadLogo32.raw");
       const im64: [u8; 4 * 64 * 64] = *include_bytes!("../../PyroquadLogo64.raw");


       let ic = Icon{
        /// 16 * 16 image of RGBA pixels (each 4 * u8) in row-major order.
        small: im16,
        /// 32 x 32 image of RGBA pixels (each 4 * u8) in row-major order.
        medium: im32,
        /// 64 x 64 image of RGBA pixels (each 4 * u8) in row-maj5or order.
        big: im64,
       };
       let mut miniConf = mq::miniquad::conf::Conf {
                    window_title: config.window_title,
                    window_width: config.window_width,
                    window_height: config.window_height,
                    fullscreen: config.fullscreen,
                    sample_count: config.sample_count,
                    window_resizable: config.window_resizable,
                    high_dpi: config.high_dpi,
                    icon: Some(ic),
                    ..Default::default()
       };

            miniConf.platform.swap_interval = config.swap_interval;
            miniConf.platform.linux_backend = config.linux.backend.into();
            miniConf.platform.linux_x11_gl = config.linux.x11_gl.into();
            miniConf.platform.wayland_decorations = config.linux.wayland_decorations.into();
            // miniquad wants a &'static str. The engine is configured once per run, so leaking it is fine.
            miniConf.platform.linux_wm_class = Box::leak(config.linux.wm_class.into_boxed_str());

            macroquad::conf::Conf {
                miniquad_conf: miniConf,
                update_on: Some(macroquad::conf::UpdateTrigger::default()),
                default_filter_mode: match config.default_filter_mode{ 
                    FilterMode::Linear => mq::FilterMode::Linear,
                    FilterMode::Nearest => mq::FilterMode::Nearest,
                },
                draw_call_vertex_capacity: config.draw_call_vertex_capacity,
                draw_call_index_capacity: config.draw_call_index_capacity,
            }
    }
}

impl Default for Config {
    fn default() -> Self {
            Config {

                window_title: "Pyroquad".to_string(),
                window_width: 800,
                window_height: 600,
                fullscreen: false,
                swap_interval: None,
                sample_count: 1,
                window_resizable: true,
                stop_python_when_closing_window: true,
                draw_call_vertex_capacity: 10_000,
                draw_call_index_capacity: 5_000,
                default_filter_mode: FilterMode::Linear,
                high_dpi: true,
                linux: LinuxConfig::default(),
            }

    }
}