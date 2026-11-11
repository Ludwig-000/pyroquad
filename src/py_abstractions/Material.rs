use std::sync::Arc;

use macroquad::prelude as mq;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::py_abstractions::PipelineParams::PipelineParams;

use crate::{_pyroquad::{Texture2D}, engine::{CoreLoop::{COMMAND_QUEUE, Command, CommandQueue}, Material::OuterMaterial, PChannel}, py_abstractions::structs::GLAM::Mat4::Mat4};



/// A Shader is a program, compiled at runtime, that operates on the GPU.
/// Typically, shaders modify how objects are drawn.
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Material {
    material: OuterMaterial
}


#[pymethods]
impl Material{

    /// Compiles a shader using the provided vertex_shader and fragment_shader.
    /// a compiled shader can be drawn via 'start_using' and released via 'stop_using'.
    /// only one custom shader can be active at a time.
    /// 
    /// a compiled shader can 'not' be deleted, renamed or modified.
    /// 
    /// `textures` names every extra `uniform sampler2D` the shader uses, so it can be
    /// bound later with `set_texture`. The texture being drawn is always available as
    /// `Texture`, so don't list it here.
    #[new]
    ///
    /// `pipeline_params` holds the depth, blend, cull and stencil settings. The default
    /// draws like 2D (no depth test, no blending); see `PipelineParams` for 3D.
    #[pyo3(signature = (vertex_shader, fragment_shader, name, uniform_descriptions = Vec::new(), textures = Vec::new(), pipeline_params = PipelineParams::default()))]
    fn new(vertex_shader: String, fragment_shader: String, name: String, uniform_descriptions: Vec<UniformDesc>, textures: Vec<String>, pipeline_params: PipelineParams)-> PyResult<Self>{

        // macroquad panics on these instead of returning an error, so check them here.
        for (i, t) in textures.iter().enumerate() {
            if t == "Texture" || t == "_ScreenTexture" {
                return Err(PyValueError::new_err(format!(
                    "the texture name '{t}' is reserved by the engine. Pick another name."
                )));
            }
            if textures[..i].contains(t) {
                return Err(PyValueError::new_err(format!("the texture name '{t}' is listed twice.")));
            }
        }

        let (sender, receiver) = PChannel::PChannel::channel();
        COMMAND_QUEUE.push(
            Command::LoadMaterial { vertex: vertex_shader, frag: fragment_shader, sender, name, uniform_desc: uniform_descriptions, textures, pipeline_params: pipeline_params.into() }
        );
        let mat = receiver.recv()??;

        Ok(Material {
            material: mat
        })
    }


    #[classattr]
    pub const fn DEFAULT_VERTEX_SHADER() -> &'static str {
        "#version 100
        attribute vec3 position;
        attribute vec2 texcoord;
        attribute vec4 color0;
        
        varying lowp vec2 uv;
        varying lowp vec4 color;
        
        uniform mat4 Model;
        uniform mat4 Projection;
        
        void main() {
            gl_Position = Projection * Model * vec4(position, 1);
            color = color0 / 255.0;
            uv = texcoord;
        }"
    }

    
    #[classattr]
    pub const fn DEFAULT_FRAGMENT_SHADER() -> &'static str {
        "#version 100
        varying lowp vec4 color;
        varying lowp vec2 uv;
        
        uniform sampler2D Texture;
        
        void main() {
            gl_FragColor = color * texture2D(Texture, uv);
        }"
    }

    
    /// Sets a uniform declared with `array_count = 1`.
    pub fn set_uniform(&self, name: String, uniform: UniformType) {
        COMMAND_QUEUE.push( Command::SetMaterialUniform { mat: self.material.clone(), name, values: vec![uniform] } );
    }

    /// Sets every element of an array uniform at once.
    /// `values` must hold exactly `array_count` elements, all of the declared type.
    pub fn set_uniform_array(&self, name: String, values: Vec<UniformType>) {
        COMMAND_QUEUE.push( Command::SetMaterialUniform { mat: self.material.clone(), name, values } );
    }

    /// Binds `texture` to the `sampler2D` called `name`.
    /// `name` must be one of the `textures` given when this material was created.
    /// The texture stays bound until it is replaced, also across frames.
    pub fn set_texture(&self, name: String, texture: Texture2D) -> PyResult<()> {
        // macroquad panics on an unknown name instead of returning an error, so check it here.
        let declared = &self.material.inner.textures;
        if !declared.iter().any(|t| **t == *name) {
            return Err(PyValueError::new_err(format!(
                "this material has no texture called '{name}'. Declared textures: {declared:?}"
            )));
        }
        COMMAND_QUEUE.push( Command::SetMaterialTexture { mat: self.material.clone(), name, texture: texture.texture } );
        Ok(())
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
        COMMAND_QUEUE.push( Command::StartUsingMaterial { mat: self.material.clone() });
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
    fn use_default_material(&self){
        COMMAND_QUEUE.push( Command::UseDefaultMaterial );
    }
    
    
}

#[pyfunction]
pub fn use_default_material(){
    COMMAND_QUEUE.push( Command::UseDefaultMaterial );
}



/// A value for a uniform.
/// Each variant matches one GLSL type: Float3 is a `vec3`, Int2 an `ivec2`, Mat4 a `mat4`.
/// For an array uniform, pass a list of these to `Material.set_uniform_array`.
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub enum UniformType {
    /// One 32-bit wide float (GLSL `float`)
    Float1( [f32; 1] ),
    /// Two 32-bit wide floats (GLSL `vec2`)
    Float2( [f32; 2] ),
    /// Three 32-bit wide floats (GLSL `vec3`)
    Float3( [f32; 3] ),
    /// Four 32-bit wide floats (GLSL `vec4`)
    Float4( [f32; 4] ),
    /// One signed 32-bit integer (GLSL `int`)
    Int1( [i32; 1] ),
    /// Two signed 32-bit integers (GLSL `ivec2`)
    Int2( [i32; 2] ),
    /// Three signed 32-bit integers (GLSL `ivec3`)
    Int3( [i32; 3] ),
    /// Four signed 32-bit integers (GLSL `ivec4`)
    Int4( [i32; 4] ),
    /// Four by four matrix of 32-bit floats (GLSL `mat4`)
    Mat4( Mat4 ),
}

impl UniformType {
    /// The macroquad type this value belongs to.
    pub fn kind(&self) -> mq::UniformType {
        match self {
            UniformType::Float1(_) => mq::UniformType::Float1,
            UniformType::Float2(_) => mq::UniformType::Float2,
            UniformType::Float3(_) => mq::UniformType::Float3,
            UniformType::Float4(_) => mq::UniformType::Float4,
            UniformType::Int1(_) => mq::UniformType::Int1,
            UniformType::Int2(_) => mq::UniformType::Int2,
            UniformType::Int3(_) => mq::UniformType::Int3,
            UniformType::Int4(_) => mq::UniformType::Int4,
            UniformType::Mat4(_) => mq::UniformType::Mat4,
        }
    }

    /// Appends this value as 4-byte words, in the layout macroquad's uniform buffer expects.
    /// An array uniform is these words back to back, so the engine passes the joined words
    /// of all elements to `set_uniform_array`.
    ///
    /// The words are `f32` only because macroquad's `ToBytes` has no `u8` impl.
    /// Ints are stored bit-for-bit (`f32::from_bits`); macroquad only copies the memory,
    /// so the shader still receives the exact integer.
    pub fn write_words(&self, out: &mut Vec<f32>) {
        fn f(out: &mut Vec<f32>, v: &[f32]) { out.extend_from_slice(v); }
        fn i(out: &mut Vec<f32>, v: &[i32]) { out.extend(v.iter().map(|x| f32::from_bits(*x as u32))); }
        match self {
            UniformType::Float1(v) => f(out, v),
            UniformType::Float2(v) => f(out, v),
            UniformType::Float3(v) => f(out, v),
            UniformType::Float4(v) => f(out, v),
            UniformType::Int1(v) => i(out, v),
            UniformType::Int2(v) => i(out, v),
            UniformType::Int3(v) => i(out, v),
            UniformType::Int4(v) => i(out, v),
            UniformType::Mat4(m) => f(out, &mq::Mat4::from(*m).to_cols_array()),
        }
    }
}




/// Declares one uniform of a shader: its name, GLSL type and how many elements it has.
/// `array_count` is 1 for a plain uniform, and N for `uniform vec4 colors[N];`.
///
/// The names `Projection`, `Model` and `_Time` are declared by the engine for every material.
/// Use them in GLSL, but don't list them here.
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub enum UniformDesc {
    /// GLSL `float`
    #[pyo3(constructor = (name, array_count = 1))]
    Float1 { name: String, array_count: usize },
    /// GLSL `vec2`
    #[pyo3(constructor = (name, array_count = 1))]
    Float2 { name: String, array_count: usize },
    /// GLSL `vec3`
    #[pyo3(constructor = (name, array_count = 1))]
    Float3 { name: String, array_count: usize },
    /// GLSL `vec4`
    #[pyo3(constructor = (name, array_count = 1))]
    Float4 { name: String, array_count: usize },
    /// GLSL `int`
    #[pyo3(constructor = (name, array_count = 1))]
    Int1 { name: String, array_count: usize },
    /// GLSL `ivec2`
    #[pyo3(constructor = (name, array_count = 1))]
    Int2 { name: String, array_count: usize },
    /// GLSL `ivec3`
    #[pyo3(constructor = (name, array_count = 1))]
    Int3 { name: String, array_count: usize },
    /// GLSL `ivec4`
    #[pyo3(constructor = (name, array_count = 1))]
    Int4 { name: String, array_count: usize },
    /// GLSL `mat4`
    #[pyo3(constructor = (name, array_count = 1))]
    Mat4 { name: String, array_count: usize },
}

impl UniformDesc {
    /// (name, macroquad type, array_count)
    pub fn parts(&self) -> (&str, mq::UniformType, usize) {
        let (name, kind, count) = match self {
            UniformDesc::Float1 { name, array_count } => (name, mq::UniformType::Float1, array_count),
            UniformDesc::Float2 { name, array_count } => (name, mq::UniformType::Float2, array_count),
            UniformDesc::Float3 { name, array_count } => (name, mq::UniformType::Float3, array_count),
            UniformDesc::Float4 { name, array_count } => (name, mq::UniformType::Float4, array_count),
            UniformDesc::Int1 { name, array_count } => (name, mq::UniformType::Int1, array_count),
            UniformDesc::Int2 { name, array_count } => (name, mq::UniformType::Int2, array_count),
            UniformDesc::Int3 { name, array_count } => (name, mq::UniformType::Int3, array_count),
            UniformDesc::Int4 { name, array_count } => (name, mq::UniformType::Int4, array_count),
            UniformDesc::Mat4 { name, array_count } => (name, mq::UniformType::Mat4, array_count),
        };
        (name.as_str(), kind, *count)
    }

    pub fn to_mq(&self) -> mq::UniformDesc {
        let (name, kind, count) = self.parts();
        mq::UniformDesc::new(name, kind).array(count)
    }
}
