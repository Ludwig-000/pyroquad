use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use macroquad::prelude as mq;

use pyo3_stub_gen::derive::* ;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, PartialEq, Debug)]
pub struct Config {

    /// Name of the window which the engine runs in.
    #[pyo3(get, set)]
    pub window_title: String,

    /// width in pixels. "fullscreen = true" will overwrite this.
    #[pyo3(get, set)] 
    pub window_width: i32,

    /// height in pixels. "fullscreen = true" will overwrite this.
    #[pyo3(get, set)] 
    pub window_height: i32,

    /// false -> creates a window with the before above selected width and heigt.
    /// true -> creates a "windowed fullscreen" window with a resultion equal to the monitor resolution.
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
    pub stop_pyton_when_closing_window: bool    
}


#[gen_stub_pymethods]
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
        stop_pyton_when_closing_window= true,
    ))]
    pub fn new(
        window_title: String,
        window_width: i32,
        window_height: i32,
        fullscreen: bool,
        swap_interval: Option<i32>,
        sample_count: i32,
        window_resizable: bool,
        stop_pyton_when_closing_window: bool,
    ) -> Self {
        Config {
            window_title,
            window_width,
            window_height,
            fullscreen,
            swap_interval,
            sample_count,
            window_resizable,
            stop_pyton_when_closing_window,
        }
    }
}



impl Config{
    pub fn to_window_config(config: Config) -> macroquad::conf::Conf {
       let mut miniConf = mq::miniquad::conf::Conf {
                    window_title: config.window_title,
                    window_width: config.window_width,
                    window_height: config.window_height,
                    fullscreen: config.fullscreen,
                    sample_count: config.sample_count,
                    window_resizable: config.window_resizable,
                    ..Default::default()
       };

            miniConf.platform.swap_interval = config.swap_interval;

            macroquad::conf::Conf {
                miniquad_conf: miniConf,
                update_on: Some(macroquad::conf::UpdateTrigger::default()),
                default_filter_mode: macroquad::prelude::FilterMode::Linear,
                draw_call_vertex_capacity: 10000,
                draw_call_index_capacity: 5000,
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
                stop_pyton_when_closing_window: true,
            }

    }
}