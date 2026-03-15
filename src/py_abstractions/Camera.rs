use macroquad::camera::Camera;
use pyo3::prelude::*;
 
use macroquad::prelude as mq;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use crate::engine::PChannel::PChannel;
use crate::py_abstractions::structs::GLAM::Mat4::Mat4;
use pyo3_stub_gen::{derive::gen_stub_pyfunction,derive::*};

use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::RenderTarget::*;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Camera2D {

    /// Rotation in degrees.
    #[pyo3(get, set)]
    pub rotation: f32,
    /// Scaling, should be (1.0, 1.0) by default.
    #[pyo3(get, set)]
    pub zoom: Vec2,
    /// Rotation and zoom origin.
    #[pyo3(get, set)]
    pub target: Vec2,
    /// Displacement from target.
    #[pyo3(get, set)]
    pub offset: Vec2,

    /// If "render_target" is set - camera will render to texture.
    ///
    /// Otherwise to the screen.
    #[pyo3(get, set)]
    pub render_target: Option<RenderTarget>,

    /// Part of the screen to render to.
    ///
    /// None means the whole screen.
    ///
    /// Viewport do not affect camera space, just the render position on the screen.
    ///
    /// Useful for things like splitscreen.
    #[pyo3(get, set)]
    pub viewport: Option<(i32, i32, i32, i32)>,
}

#[gen_stub_pymethods]
#[pymethods]  
impl Camera2D {

    #[new]
    #[pyo3(signature = (
        rotation = 0.0,
        zoom = Vec2::new(1.0, 1.0),
        target = Vec2::new(0.0, 0.0),
        offset = Vec2::new(0.0, 0.0),
        render_target = None,
        viewport = None
    ))]
    fn new(
        rotation: f32,
        zoom: Vec2,
        target: Vec2,
        offset: Vec2,
        render_target: Option<RenderTarget>,
        viewport: Option<(i32, i32, i32, i32)>,
    ) -> Self {
        Camera2D {
            rotation,
            zoom,
            target,
            offset,
            render_target,
            viewport,
        }
    }

    /// Returns the world space position for a 2d camera screen space position.
    /// Point is a screen space position, often mouse x and y.
    pub fn screen_to_world(&self, point: Vec2) -> PyResult<Vec2>{
        let (sender, receiver) = PChannel::sync_channel(1);
        COMMAND_QUEUE.push( Command::Camera2DScreenToWorld { cam:  self.clone().into(), point: point.into(), sender } );
        Ok(receiver.recv()?.into())
    }

    /// Returns the screen space position for a 2d camera world space position.
    /// Screen position in window space - from (0, 0) to (screen_width, screen_height()).
    pub fn world_to_screen(&self, point: Vec2) -> PyResult<Vec2>{
        let (sender, receiver) = PChannel::sync_channel(1);
        COMMAND_QUEUE.push( Command::Camera2DWorldToScreen { cam:  self.clone().into(), point: point.into(), sender } );
        Ok(receiver.recv()?.into())
    }


    pub fn matrix(&self) -> PyResult<Mat4>{
        let (sender, receiver) = PChannel::sync_channel(1);
        COMMAND_QUEUE.push( Command::Camera2DToMatrix { cam: self.clone().into() , sender });
        Ok(receiver.recv()?.into())
    }
    /// Creates a camera based on a Retangle.
    /// Takes A 2D rectangle, defined by its top-left corner, width and height.
    #[staticmethod]
    pub fn from_display_rect(top_left_x: f32, top_left_y: f32, width: f32, height: f32) -> Camera2D {
        let rec =  mq::Rect::new(top_left_x, top_left_y,width, height);
        
        mq::Camera2D::from_display_rect(rec).into()
    }

    /// Set active 2D camera.
    #[staticmethod]
    pub fn set_camera(camera: Camera2D) {
        
        let cam: mq::Camera2D = camera.into();
        COMMAND_QUEUE.push(Command::SetCamera { camera_2d: Some(cam), camera_3d: None });


    }
}



impl From<mq::Camera2D> for Camera2D {
    fn from(r: mq::Camera2D) -> Self {
        Camera2D {
            rotation: r.rotation,
            zoom: r.zoom.into(),
            target: r.target.into(),
            offset: r.offset.into(),
            render_target: r.render_target.map(|rt| rt.into()),
            viewport: r.viewport,
        }
    }
}

impl From<Camera2D> for mq::Camera2D {
    fn from(r: Camera2D) -> Self {
        mq::Camera2D {
            rotation: r.rotation,
            zoom: r.zoom.into(),
            target: r.target.into(),
            offset: r.offset.into(),
            render_target: r.render_target.map(|rt| rt.into()),
            viewport: r.viewport,
        }
    }
}




/// default 2D Camera
#[gen_stub_pyfunction]
#[pyfunction]
pub fn set_default_camera()  {
    COMMAND_QUEUE.push(Command::SetDefaultCamera());
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn push_camera_state()  {
    COMMAND_QUEUE.push(Command::PushCameraState);
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn pop_camera_state()  {
    COMMAND_QUEUE.push(Command::PopCameraState);
}

/// From given font size in world space gives (font_size, font_scale and font_aspect) params to make rasterized font looks good in currently active camera
#[gen_stub_pyfunction]
#[pyfunction]
pub fn camera_font_scale(world_font_size: f32)-> PyResult<(u16, f32, f32)>{
    let (sender, reciever) =  PChannel::sync_channel(1);
    COMMAND_QUEUE.push(Command::CameraFontScale { world_font_size, sender });
    Ok(reciever.recv()?)
}
















/// TODO: make it easy to 'target' a specific location in space.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Camera3D {

    /// Camera position.
    #[pyo3(get, set)]
    pub position: Vec3,

    /// Camera target it looks-at.
    #[pyo3(get, set)]
    pub target: Vec3,

    /// Camera up vector (rotation over its axis).
    #[pyo3(get, set)]
    pub up: Vec3,

    /// Camera field-of-view aperture in Y (radians)
    /// in perspective, used as near plane width in orthographic.
    #[pyo3(get, set)]
    pub fovy: f32,

    /// Screen aspect ratio.
    ///
    /// By default aspect is calculated with screen_width() / screen_height() on each frame.
    #[pyo3(get, set)]
    pub aspect: Option<f32>,

    /// Camera projection type, perspective or orthographics.
    #[pyo3(get, set)]
    pub projection: Projection,

    /// If "render_target" is set - camera will render to texture.
    ///
    /// Otherwise to the screen.
    #[pyo3(get, set)]
    pub render_target: Option<RenderTarget>,

    /// Part of the screen to render to.
    ///
    /// None means the whole screen.
    ///
    /// Viewport do not affect camera space, just the render position on the screen.
    ///
    /// Useful for things like splitscreen.
    #[pyo3(get, set)]
    pub viewport: Option<(i32, i32, i32, i32)>,

    /// Camera near plane
    #[pyo3(get, set)]
    pub z_near: f32,

    /// Camera far plane
    #[pyo3(get, set)]
    pub z_far: f32,

}


#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Projection {
    Perspective,
    Orthographics,
}
impl From<mq::Projection> for Projection {
    fn from(pr: mq::Projection) -> Self {
        match pr {
            mq::Projection::Perspective => Projection::Perspective,
            mq::Projection::Orthographics => Projection::Orthographics,
        }
    }
}
impl From<Projection> for mq::Projection {
    fn from(pr: Projection) -> Self {
        match pr {
            Projection::Perspective => mq::Projection::Perspective,
            Projection::Orthographics => mq::Projection::Orthographics,
        }
    }
}



#[gen_stub_pymethods]
#[pymethods]  
impl Camera3D {
    #[new]
    #[pyo3(signature = (
        position = Vec3::new(0.0, -10.0, 0.0),
        target = Vec3::new(0.0, 0.0, 0.0),
        aspect = None,
        up = Vec3::new(0.0, 0.0, 1.0),
        fovy = 45.0_f32.to_radians(),
        projection = Projection::Perspective,
        render_target = None,
        viewport = None,
        z_near = 0.01,
        z_far = 10000.0,
    ))]
    fn new(
        position: Vec3,
        target: Vec3,
        aspect: Option<f32>,
        up: Vec3,
        fovy: f32,
        projection: Projection,
        render_target: Option<RenderTarget>,
        viewport: Option<(i32, i32, i32, i32)>,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Camera3D {
            position,
            target,
            up,
            fovy,
            aspect,
            projection,
            render_target,
            viewport,
            z_near,
            z_far,
        }
    }

    #[staticmethod]
    /// Set active 3D camera.
    pub fn set_camera(camera: Camera3D) {
        let cam: mq::Camera3D = camera.into();
        COMMAND_QUEUE.push(Command::SetCamera { camera_2d: None, camera_3d: Some(cam) });
    }
}

impl Default for Camera3D {
    fn default() -> Camera3D {
        mq::Camera3D::default().into()
    }
}


impl From<mq::Camera3D> for Camera3D {
    fn from(r: mq::Camera3D) -> Self {
        Camera3D {
            position: r.position.into(),
            target: r.target.into(),
            up: r.up.into(),
            fovy: r.fovy,
            aspect: r.aspect,
            projection: r.projection.into(),
            render_target: r.render_target.map(|rt| rt.into()),
            viewport: r.viewport,
            z_near: r.z_near,
            z_far: r.z_far,
        }
    }
}

impl From<Camera3D> for mq::Camera3D {
    fn from(r: Camera3D) -> Self {
        mq::Camera3D {
            position: r.position.into(),
            target: r.target.into(),
            up: r.up.into(),
            fovy: r.fovy,
            aspect: r.aspect,
            projection: r.projection.into(),
            render_target: r.render_target.map(|rt| rt.into()),
            viewport: r.viewport,
            z_near: r.z_near,
            z_far: r.z_far,
        }
    }
}