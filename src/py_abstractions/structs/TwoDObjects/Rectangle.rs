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
use crate::py_abstractions::structs::TwoDObjects::Circle::Circle;
use crate::py_abstractions::structs::TwoDObjects::collision::Shape;
use crate::py_abstractions::structs::TwoDObjects::collision::collides_with_rec_circ;
use crate::py_abstractions::structs::TwoDObjects::collision::collides_with_rec_rec;
use pyo3::types::PyDict;

#[gen_stub_pyclass]
#[pyclass(eq)]
#[derive(PartialEq)]
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

crate::implement_magic_methods2D!(Rectangle);

#[gen_stub_pymethods]
#[pymethods]
impl Rectangle{
    #[new]
    #[pyo3(signature = ( position, rotation, scale, color, texture = None))]
    pub fn new(position: Vec2, rotation: f32, scale: Vec2, color: Color, texture: Option<Texture2D>)-> Self{
        Rectangle { position, rotation, scale, color, texture, function_key: None }
    }

    pub fn draw(&self){
        COMMAND_QUEUE.push(  Command::DrawRectangleFromPyClass(partial_clone(self)));
    }

    /// Check collision between two 2D Objects.
    /// A Circle's collider assumes a perfect Circle.
    pub fn collides_with(&self, rhs: Shape<'_>)-> bool{
        match rhs{
            Shape::Circ(c)=> {
                let circle = c.borrow();
                collides_with_rec_circ(self, &circle)
            },
            Shape::Rect(r)=>{
                let other_rect = r.borrow(); 
                collides_with_rec_rec(self, &other_rect)
            }
        }
    }


    /// takes a list of 2D shapes, and returns every element that Collides with self.
    pub fn collides_with_list<'py>(&self, rhs: Vec<Shape<'py>>)-> Vec<Shape<'py>>{

        rhs.into_iter().filter_map(|element|{
            let is_hit = match &element {
                Shape::Rect(rect_bound) => {
                    let other = rect_bound.borrow();
                    collides_with_rec_rec(self, &other)
                }
                Shape::Circ(circ_bound) => {
                    let other = circ_bound.borrow();
                    collides_with_rec_circ(self, &other)
                }
            };
            if is_hit {
                Some(element)
            } else {None}

        }).collect()
    }


    pub fn max_x(&self) -> f32 {
        let (sin_r, cos_r) = self.rotation.sin_cos();
        let half_w = (self.scale.x / 2.0).abs();
        let half_h = (self.scale.y / 2.0).abs();
        
        self.position.x + (half_w * cos_r.abs() + half_h * sin_r.abs())
    }

    pub fn min_x(&self) -> f32 {
        let (sin_r, cos_r) = self.rotation.sin_cos();
        let half_w = (self.scale.x / 2.0).abs();
        let half_h = (self.scale.y / 2.0).abs();
        
        self.position.x - (half_w * cos_r.abs() + half_h * sin_r.abs())
    }

    pub fn max_y(&self) -> f32 {
        let (sin_r, cos_r) = self.rotation.sin_cos();
        let half_w = (self.scale.x / 2.0).abs();
        let half_h = (self.scale.y / 2.0).abs();
        self.position.y + (half_w * sin_r.abs() + half_h * cos_r.abs())
    }

    pub fn min_y(&self) -> f32 {
        let (sin_r, cos_r) = self.rotation.sin_cos();
        let half_w = (self.scale.x / 2.0).abs();
        let half_h = (self.scale.y / 2.0).abs();
        
        self.position.y - (half_w * sin_r.abs() + half_h * cos_r.abs())
    }
}

fn partial_clone(rec: &Rectangle) -> Rectangle{
    Rectangle { position: rec.position, 
        rotation: rec.rotation, 
        scale: rec.scale, 
        color: rec.color, 
        texture: rec.texture.clone(), 
        function_key: None }
}