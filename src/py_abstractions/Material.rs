use std::sync::Arc;

use macroquad::prelude as mq;
use pyo3::prelude::*;

use crate::engine::{CoreLoop::{COMMAND_QUEUE, Command, CommandQueue}, PChannel};



/// A Shader is a program, compiled at runtime, that operates on the GPU.
/// Typically, shaders modify how objects are drawn.
#[pyclass(from_py_object, frozen, eq)]
#[derive(Clone, PartialEq)]
pub struct Material {
    #[pyo3(get)]
    name: String,

    mat: mq::Material,
}


#[pymethods]
impl Material{

    /// Compiles a shader using the provided vertex_shader and fragment_shader.
    /// a compiled shader can be drawn via 'start_using' and released via 'stop_using'.
    /// only one custom shader can be active at a time.
    /// 
    /// a compiled shader can 'not' be deleted, renamed or modified.
    /// 
    #[new]
    fn new(vertex_shader: String, fragment_shader: String, name: String)-> PyResult<Self>{

        let shader = ShaderSource::Glsl { vertex: vertex_shader, fragment: fragment_shader };
        let (sender, receiver) = PChannel::PChannel::channel();
        COMMAND_QUEUE.push(
            Command::LoadMaterial { shader, sender }
        );
        let mat = receiver.recv()??;

        Ok(Material {
            name,
            mat
        })
    }

    /// Each drawable object has an internal default-shader.
    /// This function is used to overwrite the default preference and apply a custom ( or internal ) shader for any draw calls following  this function.
    /// 
    /// example usage:
    /// ```
    /// while True:
    /// 
    ///     # will draw using the default 2d shader.
    ///     draw_rectangle(...)
    /// 
    ///     myShader.start_using()
    ///     
    ///     # will draw rectangle using the custom shader.
    ///     draw_rectangle(...)
    /// 
    ///     myShader.stop_using()
    /// 
    ///      # will draw using the default 2d shader.
    ///     draw_rectangle(...)
    /// 
    ///     # this will reset the usage of custom shaders, so 'myShader.start_using()' 
    ///     # will have to be called again next frame.
    ///     next_frame()
    /// ```
    fn start_using(&self){
        todo!()
    }



    /// Each drawable object has an internal default-shader.
    /// This function is used to overwrite the default preference and apply a custom ( or internal ) shader for any draw calls following  this function.
    /// 
    /// example usage:
    /// ```
    /// while True:
    /// 
    ///     # will draw using the default 2d shader.
    ///     draw_rectangle(...)
    /// 
    ///     myShader.start_using()
    ///     
    ///     # will draw rectangle using the custom shader.
    ///     draw_rectangle(...)
    /// 
    ///     myShader.stop_using()
    /// 
    ///      # will draw using the default 2d shader.
    ///     draw_rectangle(...)
    /// 
    ///     # this will reset the usage of custom shaders, so 'myShader.start_using()' 
    ///     # will have to be called again next frame.
    ///     next_frame()
    /// ```
    fn stop_using(&self){
        todo!()
    }
    

    #[staticmethod]
    fn get_internal_shader(name: String)-> Material{
        todo!()
    }
    
    /// Provides a list of all compiled shaders.
    #[staticmethod]
    fn list_internal_shaders()-> Vec<Material>{
        todo!()
    }
    
}


/// Not yet implmented, but will be used to set uniform variables for shaders.

#[pyclass]
#[derive(Debug)]
pub enum ShaderSource {
    Glsl { vertex: String, fragment: String },
    Msl { program: String },
}
