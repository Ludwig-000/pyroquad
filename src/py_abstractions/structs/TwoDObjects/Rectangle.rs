use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3_stub_gen::derive::* ;
use pyo3::exceptions::*;

use crate::py_abstractions::Textures_and_Images::Texture2D;


use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::Color::Color;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunStorage::FunctionKey;
use crate::py_abstractions::structs::TwoDObjects::Circle::Circle;
use crate::py_abstractions::structs::TwoDObjects::collision::Shape;
use crate::py_abstractions::structs::TwoDObjects::collision::collides_with_rec_circ;
use crate::py_abstractions::structs::TwoDObjects::collision::collides_with_rec_rec;
use crate::py_assert;
use pyo3::types::PyDict;

#[gen_stub_pyclass]
#[pyclass(eq, weakref)]
#[derive(PartialEq)]
pub struct Rectangle{

    #[pyo3(get,set)]
    pub position: Vec2,
    
    /// rotation is in radians.
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
    #[pyo3(signature = ( position=Vec2::new(100.,100.), rotation=0.0, scale= Vec2::new(100.,100.), color = Color::WHITE(), texture = None))]
    pub fn new(position: Vec2, rotation: f32, scale: Vec2, color: Color, texture: Option<Texture2D>)-> Self{
        Rectangle { position, rotation, scale, color, texture, function_key: None }
    }

    /// defining a rectangle by its pivot can be annoying, so this allows for an alternate constructor.
    /// with x1,y1 being top left, x2,y2 being bottom right
    #[staticmethod]
    #[pyo3(signature = ( x1=0., y1=0., x2=100., y2=100., color= Color::WHITE(), texture=None))]
    pub fn from_xy(x1: f32, y1: f32, x2: f32, y2: f32, color: Color, texture: Option<Texture2D>) -> PyResult<Rectangle>{
        py_assert!(x2 > x1, "x2 must be greated than x1");
        py_assert!(y2 > y1, "y2 must be greater than y1");

        Ok(Rectangle{
            position: Vec2::const_new((x2+x1)/2.0, (y2+y1)/2.0),
            rotation: 0.,
            scale: Vec2::const_new(x2-x1, y2-y1),
            color,
            texture,
            function_key: None,
        })
    }

    pub fn draw(&self){
        COMMAND_QUEUE.push(  Command::DrawRectangleFromPyClass(self.partial_clone()));
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


impl Rectangle{
    /// Does NOT clone the function key.
    fn partial_clone(&self) -> Rectangle{
        Rectangle { position: self.position, 
            rotation: self.rotation, 
            scale: self.scale, 
            color: self.color, 
            texture: self.texture.clone(), 
            function_key: None }
    }
}