use macroquad::prelude as mq;
use pyo3::PyResult;
use pyo3::{pyclass, pyfunction,pymethods};


use crate::engine::PChannel::PChannel;

use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
 
use crate::engine::PArc::PArc;



#[pyclass(from_py_object)]
#[derive(Clone, Debug)]
pub struct RenderTarget {
   render_target:  PArc<mq::RenderTarget>,
}


impl From<mq::RenderTarget> for RenderTarget{
    fn from(r: mq::RenderTarget) -> Self {
        RenderTarget{ render_target: PArc::new(r) }
    }
}


impl From<RenderTarget> for mq::RenderTarget{
    fn from(r: RenderTarget) -> Self {
        (*r.render_target).clone()
    }
}
impl From<&RenderTarget> for mq::RenderTarget{
    fn from(r: &RenderTarget) -> Self {
        (*r.render_target).clone()
    }
}






/// A shortcut to create a render target with no depth buffer and `sample_count: 4`

#[pyfunction]
pub fn render_target_msaa(width: u32, height: u32) -> PyResult<RenderTarget> {
    let (tx, rx) = PChannel::channel();
    COMMAND_QUEUE.push( Command::RenderTargetMsaa{width,height,sender: tx} );

    let render_target = rx.recv()?;
    Ok(RenderTarget { render_target })
}



#[pyfunction]
#[pyo3(signature = (width, height, params = None))]
pub fn render_target(width: u32, height: u32, params: Option<RenderTargetParams>) -> PyResult<RenderTarget> {

    let (sender, receiver) = PChannel::channel();

    COMMAND_QUEUE.push( Command::RenderTargetEx { width, height, params: params.map(Into::into), sender});

    let render_target = receiver.recv()?;
    Ok(RenderTarget { render_target })
}





#[pyclass(from_py_object)]
#[derive(Clone,Copy, PartialEq, Debug)]
pub struct RenderTargetParams {
    /// 1 means no multi sampling.
    /// Note that sample_count > 1 is not supported on GL2, GLES2 and WebGL1
    #[pyo3(get, set)]
    pub sample_count: i32,

    /// depth: true creates a depth render target attachment and allows
    /// such a render target being used for a depth-testing cameras
    #[pyo3(get, set)]
    pub depth: bool,
}


#[pymethods]
impl RenderTargetParams {

    #[new]
    #[pyo3(signature = (sample_count = 1, depth = false))]
    pub fn new(sample_count: i32, depth: bool) -> Self {
        Self { sample_count, depth }
    }

}

impl From<mq::RenderTargetParams> for RenderTargetParams{
    fn from(r: mq::RenderTargetParams) -> Self {
        RenderTargetParams { sample_count: r.sample_count, depth: r.depth}
    }
}



impl From<RenderTargetParams> for mq::RenderTargetParams{
    fn from(r: RenderTargetParams) -> Self {
        mq::RenderTargetParams {sample_count: r.sample_count, depth: r.depth}
    }
}
