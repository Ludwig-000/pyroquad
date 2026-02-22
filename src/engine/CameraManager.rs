/// All internal Camera Fnctions MUST pass through this file.
/// 

use macroquad::{camera::{Camera2D, Camera3D}, prelude as mq};


// because for some reason, macroquad's cameras don't implement clone
pub fn clone_camera3d(cam: &Camera3D)-> Camera3D{
    mq::Camera3D{
        position: cam.position,
        target: cam.target,
        up: cam.up,
        fovy: cam.fovy,
        aspect: cam.aspect,
        projection: cam.projection,
        render_target: cam.render_target.clone(),
        viewport: cam.viewport,
        z_near: cam.z_near,
        z_far: cam.z_far
    }
}

// because for some reason, macroquad's cameras don't implement clone
pub fn clone_camera2d(cam: &Camera2D)-> Camera2D{
    mq::Camera2D{
        rotation: cam.rotation,
        zoom: cam.zoom,
        target: cam.target,
        offset: cam.offset,
        render_target: cam.render_target.clone(),
        viewport: cam.viewport,
    }
}

pub struct CamMemory{
    pub current_cam: Camera,
    pub last_3d_camera: Option<mq::Camera3D>,
    pub last_2d_camera: Option<mq::Camera2D>
}
impl CamMemory{
    pub fn new()-> CamMemory{
        CamMemory{
            current_cam: Camera::Camera2D(mq::Camera2D::default()),
            last_2d_camera: Some(mq::Camera2D::default()),
            last_3d_camera: None
        }
    }
    pub fn push(&mut self, camera: &Camera){
        match &self.current_cam{
            Camera::Camera2D(cam)=> self.last_2d_camera = Some(clone_camera2d(&cam)),
            Camera::Camera3D(cam)=> self.last_3d_camera = Some(clone_camera3d(&cam)),
        }
        match camera{
            Camera::Camera2D(cam)=>{
                self.current_cam = Camera::Camera2D(clone_camera2d(cam));
            }
            Camera::Camera3D(cam)=>{
                self.current_cam = Camera::Camera3D(clone_camera3d(cam));
            }
        }
    }
    
}
pub fn camera_font_scale(world_font_size: f32)-> (u16, f32, f32){
    #[allow(clippy::disallowed_methods)]
    mq::camera_font_scale(world_font_size)
}
pub fn push_camera_state(){
    #[allow(clippy::disallowed_methods)]
    mq::push_camera_state();
}
pub fn pop_camera_state(){
    #[allow(clippy::disallowed_methods)]
    mq::pop_camera_state();
}
pub fn set_default_camera(cam_memory: &mut CamMemory){
    cam_memory.push( &Camera::Camera2D(mq::Camera2D::default()));
    #[allow(clippy::disallowed_methods)]
    mq::set_default_camera();
}
pub fn set_camera(cam_memory: &mut CamMemory, camera: Camera){
    cam_memory.push(&camera);

    #[allow(clippy::disallowed_methods)]
    match camera{
        Camera::Camera2D(cam)=> mq::set_camera(&cam),
        Camera::Camera3D(cam)=> mq::set_camera(&cam),
    }
}
pub enum Camera{
    Camera2D(mq::Camera2D),
    Camera3D(mq::Camera3D)
}