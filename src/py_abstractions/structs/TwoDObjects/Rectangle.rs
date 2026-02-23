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
pub struct Rectangle{

    #[pyo3(get,set)]
    pub position: Vec2,
    #[pyo3(get,set)]
    pub rotation: f32,
    #[pyo3(get,set)]
    pub scale: Vec2,
    #[pyo3(get,set)]
    pub color: Color,

    #[pyo3(get,set)]
    pub texture: Option<Texture2D>,

    function_key: Option<FunctionKey>
}

crate::implement_Drop2D!(Rectangle);
crate::implement_tick2D!(Rectangle,  r#"Rectangle()"#);


#[gen_stub_pymethods]
#[pymethods]
impl Rectangle{
    #[new]
    pub fn new(position: Vec2, rotation: f32, scale: Vec2, color: Color)-> Self{
        Rectangle { position, rotation, scale, color, texture: None, function_key: None }
    }

    pub fn draw(&self){
        COMMAND_QUEUE.push(  Command::DrawRectangleFromPyClass(self.clone()));
    }

}