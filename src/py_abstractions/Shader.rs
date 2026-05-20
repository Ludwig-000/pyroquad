use macroquad::prelude;
use pyo3::prelude::*;
use pyo3_stub_gen::{derive::*};



/// A Shader is a program, compiled at runtime, that operates on the GPU.
/// Typically, shaders modify how objects are drawn.
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shader {
    #[pyo3(get)]
    name: &'static str,
}


#[gen_stub_pymethods]
#[pymethods]
impl Shader{

    /// Compiles a shader using the provided vertex_shader and fragment_shader.
    /// a compiled shader can be drawn via 'start_using' and released via 'stop_using'.
    /// only one custom shader can be active at a time.
    /// 
    /// a compiled shader can 'not' be deleted, renamed or modified.
    /// 
    #[new]
    fn new(vertex_shader: Option<String>, fragment_shader: Option<String>, name: String)-> Self{
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
    fn get_internal_shader(name: String)-> Shader{
        todo!()
    }
    
    /// Provides a list of all compiled shaders.
    #[staticmethod]
    fn list_internal_shaders()-> Vec<Shader>{
        todo!()
    }
    
}


/// Not yet implmented, but will be used to set uniform variables for shaders.
#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Debug)]
pub enum ShaderSource {
    Glsl { vertex: String, fragment: String },
    Msl { program: String },
}
