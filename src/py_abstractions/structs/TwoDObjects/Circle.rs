use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;

use pyo3::exceptions::*;

use crate::py_abstractions::Textures_and_Images::Texture2D;


use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::Color::Color;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunStorage::FunctionKey;
use crate::py_abstractions::structs::TwoDObjects::Rectangle::Rectangle;
use crate::py_abstractions::structs::TwoDObjects::collision::Shape;
use crate::py_abstractions::structs::TwoDObjects::collision::collides_with_circl_circ;
use crate::py_abstractions::structs::TwoDObjects::collision::collides_with_rec_circ;
use pyo3::types::PyDict;

#[gen_stub_pyclass]
#[pyclass(eq, weakref, dict)]
#[derive(PartialEq)]
pub struct Circle{
    #[pyo3(get,set)]
    pub position: Vec2,

    /// rotation is in radians.
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

crate::implement_magic_methods2D!(Circle);
#[gen_stub_pymethods]
#[pymethods]
impl Circle{
    #[new]
    #[pyo3(signature = ( position, rotation, radius, color, texture = None))]
    pub fn new(position: Vec2, rotation: f32, radius: f32, color: Color, texture: Option<Texture2D>)-> Self{
        Circle { position, rotation, radius, color, sides: 20,texture, function_key: None }
    }
    pub fn draw(&self){
        COMMAND_QUEUE.push(  Command::DrawCircleFromPyClass(self.partial_clone()));
    }

    
    /// Check collision between two 2D Objects.
    /// A Circle's collider assumes a perfect Circle.
    pub fn collides_with(&self, rhs: Shape<'_>)-> bool{
        match rhs{
            Shape::Circ(c)=> {
                let circle = c.borrow();
                collides_with_circl_circ(self, &circle)
            },
            Shape::Rect(r)=>{
                let other_rect = r.borrow(); 
                collides_with_rec_circ(&other_rect, self)
            }
        }
    }

    /// takes a list of 2D shapes, and returns every element that Collides with self.
    pub fn collides_with_list<'py>(&self, rhs: Vec<Shape<'py>>)-> Vec<Shape<'py>>{

        rhs.into_iter().filter_map(|element|{
            let is_hit = match &element {
                Shape::Rect(rect_bound) => {
                    let other = rect_bound.borrow();
                    collides_with_rec_circ(&other, self)
                }
                Shape::Circ(circ_bound) => {
                    let other = circ_bound.borrow();
                    collides_with_circl_circ(self, &other)
                }
            };
            if is_hit {
                Some(element)
            } else {None}
        }).collect()
    }


    pub fn max_x(&self) -> f32{
        self.position.x + self.radius
    }
    pub fn min_x(&self) -> f32{
        self.position.x - self.radius
    }
    pub fn max_y(&self) -> f32{
        self.position.y + self.radius
    }
    pub fn min_y(&self) -> f32{
        self.position.y - self.radius
    }
    


}


impl Circle{
    /// Does NOT clone the function key.
    fn partial_clone(&self) -> Circle{
        Circle { position: self.position, 
            rotation: self.rotation, 
            radius: self.radius, 
            sides: self.sides,
            color: self.color, 
            texture: self.texture.clone(), 
            function_key: None 
        }
    }
}
