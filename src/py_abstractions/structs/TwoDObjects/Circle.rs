use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;

use pyo3::exceptions::*;

use crate::py_abstractions::Textures_and_Images::Texture2D;


use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage::FunctionKey;


#[gen_stub_pyclass]
#[pyclass(eq)]
#[derive(Clone, PartialEq)]
pub struct Circle{
    #[pyo3(get,set)]
    pub position: Vec2,
    #[pyo3(get,set)]
    pub rotation: f32,
    #[pyo3(get,set)]
    pub radius: f32,
    #[pyo3(get,set)]
    pub color: Color,
    #[pyo3(get,set)]
    pub sides: u32,
    #[pyo3(get,set)]
    pub texture: Option<Texture2D>,

    function_key: Option<FunctionKey>
}
crate::implement_Drop2D!(Circle);
crate::implement_tick2D!(Circle,  r#"Circle()"#);

#[gen_stub_pymethods]
#[pymethods]
impl Circle{
    #[new]
    pub fn new(position: Vec2, rotation: f32, radius: f32, color: Color)-> Self{
        Circle { position, rotation, radius, color, sides: 20,texture: None, function_key: None }
    }
    pub fn draw(&self){
        COMMAND_QUEUE.push(  Command::DrawCircleFromPyClass(self.clone()));
    }
}