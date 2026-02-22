use std::cell::RefCell;
use std::sync::Arc;
use crate::engine::PChannel::*;
///
/// 
/// 
/// A bunch of functions and structs to mimic "get_internal_gl" and it's functionality
/// 
/// 
/// 



use macroquad::prelude as mq;
use macroquad::window::get_internal_gl;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};
use crate::py_abstractions::structs::GLAM::Mat4::Mat4;
use crate::py_abstractions::{Color::Color, Textures_and_Images::Texture2D, structs::GLAM::{Vec2::Vec2, Vec3::Vec3}};

#[gen_stub_pyclass]
#[pyclass(frozen)]
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub position: mq::Vec3,
    pub uv: mq::Vec2,
    pub color: [u8; 4],
    /// Normal is not used by macroquad and is completely optional.
    /// Might be usefull for custom shaders.
    /// While normal is not used by macroquad, it is completely safe to use it
    /// to pass arbitary user data, hence Vec4.
    pub normal: mq::Vec4,
}

impl Vertex{
    pub fn new(position: Vec3, uv: Vec2, color: Color)-> Vertex{
        Vertex{ 
            position: position.into(),
             uv: uv.into(),
            color: [
                (color.r * 255.)as u8,
                (color.g * 255.)as u8,
                (color.b * 255.)as u8,
                (color.a * 255.)as u8,
            ],
            normal: mq::Vec4::ZERO,
        }
    }
}



impl Into<mq::Vertex> for Vertex{
    fn into(self) -> mq::Vertex {
        mq::Vertex::new2(self.position, self.uv, self.color.into())
    }
}


#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Clone, Copy)]
pub enum DrawMode{
    Triangles,
    Lines,
}


/// A Pipeline, created via 'create_gl_pipeline'
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy)]
pub struct  GlPipeline(mq::GlPipeline);

/// A simple datatype that holds a Vector of Vertices and a Vector of indices.
/// using this Wrapper in conjunction with 'geometry()' avoids cloning any data.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct Geometry{
    vertices: Arc<[mq::Vertex]>,
    indices: Arc<[u16]>,
}

/// NOT YET IMPLEMENTED
/// try to convince Stub gen this is a module.
#[gen_stub_pyclass(module = "pyroquad.internal_gl")] // module = .. does not seem to do anything but does not err?
#[pyclass]
pub struct InternalGL();


// Note:
// All commented out functions are not easily implemented since it's inputs/outputs 
// are hard to convert into python types.
#[gen_stub_pymethods]
#[pymethods]
impl InternalGL{
    #[staticmethod]
    pub fn clear_draw_calls(){
        
        todo!()
    }
    #[staticmethod]
    pub fn delete_pipeline(pipeline: GlPipeline){
        todo!()
    }
    #[staticmethod]
    pub fn depth_test(enable: bool){
        todo!()
    }
    // #[staticmethod]
    // pub fn draw(ctx: &mut dyn miniquad::RenderingBackend, projection: glam::Mat4){
    //     todo!()
    // }
    #[staticmethod]
    pub fn draw_mode(draw_mode: DrawMode){
        todo!()
    }
    #[staticmethod]
    pub fn geometry(geometry: Geometry){
        todo!()
    }
    // #[staticmethod]
    // pub fn get_active_render_pass()-> Option<RenderPass>{
    //     todo!()
    // }
    #[staticmethod]
    pub fn get_viewport_matrix()-> Mat4{
        todo!()
    }
    #[staticmethod]
    pub fn get_viewport()-> (i32,i32,i32,i32){
        todo!()
    }
    #[staticmethod]
    pub fn is_depth_test_enabled()-> bool{
        todo!()
    }
    // #[staticmethod]
    // pub fn make_pipeline(
    //     ctx: &mut (dyn RenderingBackend + 'static), 
    //     shader: ShaderSource<'_>, 
    //     params: PipelineParams, 
    //     uniforms: Vec<UniformDesc, Global>, 
    //     textures: Vec<String, Global>)
    //      -> Result<GlPipeline, Error>{
    //     todo!()
    // }
    #[staticmethod]
    pub fn pipeline(pipeline: Option<GlPipeline>){
        todo!()
    }
    #[staticmethod]
    pub fn pop_model_matrix(){
        todo!()
    }
    #[staticmethod]
    pub fn push_model_matrix(matrix: Mat4){
        todo!()
    }
    // #[staticmethod]
    // pub fn render_pass(render_pass: Option<RenderPass>){
    //     todo!()
    // }
    ///Reset internal state to known default
    #[staticmethod]
    pub fn reset(){
        todo!()
    }
    #[staticmethod]
    pub fn scissor(clip: Option<(i32, i32, i32, i32)>){
        todo!()
    }
    #[staticmethod]
    pub fn set_texture(pipeline: GlPipeline, name: &str, texture: Texture2D){
        todo!()
    }

    #[staticmethod]
    pub fn texture(texture: Option<Texture2D>){
        todo!()
    }
    #[staticmethod]
    pub fn viewport(viewport: Option<(i32, i32, i32, i32)>){
        todo!()
    }

}



pub enum GlEnum{
    ClearDrawCalls,
    DeletePipeline(GlPipeline),
    DepthTest(bool),
    DrawMode(DrawMode),
    Geometry(Geometry),
    GetViewportMatrix(PSyncSender<Mat4>),
    GetViewport(PSyncSender<(i32,i32,i32,i32)>),
    IsDephTestEnabled(PSyncSender<bool>),
    Pipeline(Option<GlPipeline>),
    PopModelMatrx,
    PushModelMatrix(Mat4),
    Reset,
    Scissor(Option<(i32, i32, i32, i32)>),
    SetTexture{
        pipeline: GlPipeline, name: String, texture: Texture2D
    },
    Texture(Option<Texture2D>),
    Viewport(Option<(i32, i32, i32, i32)>),

}

/// this shall only be called from inside Core loop.
/// i am just too lazy to put it there.
pub fn implement_GlEnum(en: GlEnum, gl: & mut mq::QuadGl){
    match en{
        GlEnum::ClearDrawCalls => gl.clear_draw_calls(),
        GlEnum::DeletePipeline(pipe)=> gl.delete_pipeline(pipe.0),
        GlEnum::DepthTest(enable)=> gl.depth_test(enable),
        GlEnum::DrawMode(draw_mode)=> {
            let dm = match draw_mode{
                DrawMode::Lines=> mq::DrawMode::Lines,
                DrawMode::Triangles => mq::DrawMode::Triangles,
            };
            gl.draw_mode(dm);
        }
        GlEnum::Geometry(geomentry)=> gl.geometry(&geomentry.vertices, &geomentry.indices),
        GlEnum::GetViewport(sender)=> {
            let _ = sender.send( gl.get_viewport() );
        }
        GlEnum::GetViewportMatrix(sender)=>{
            let _ = sender.send( gl.get_projection_matrix().into() );
        }
        GlEnum::IsDephTestEnabled(sender)=>{
            let _ = sender.send( gl.is_depth_test_enabled() );
        }
        GlEnum::Pipeline(p)=> gl.pipeline(p.map(|pp| pp.0)),
        GlEnum::PopModelMatrx => gl.pop_model_matrix(),
        GlEnum::PushModelMatrix(mat)=> gl.push_model_matrix(mat.into()),
        GlEnum::Reset => gl.reset(),
        GlEnum::Scissor(clip)=> gl.scissor(clip),
        GlEnum::SetTexture { pipeline, name, texture }=>{
            gl.set_texture(pipeline.0, &name, texture.into());
        }
        GlEnum::Texture(t)=> gl.texture(t.map(|tex| tex.into()).as_ref()),
        GlEnum::Viewport(v)=> gl.viewport(v),

    }
}