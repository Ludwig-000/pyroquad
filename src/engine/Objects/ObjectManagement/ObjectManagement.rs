use crate::engine::Objects::ObjectManagement::{ObjectStorage::*};
use macroquad::prelude as mq;


pub fn draw_all_Objects(obj: &ObjectStorage, viewMat: macroquad::prelude::Mat4){

    unsafe {

        let gl: &mut macroquad::prelude::QuadGl  = macroquad::prelude::get_internal_gl().quad_gl;
        gl.draw_mode(mq::DrawMode::Triangles);


        let _: () = obj.iter().map(|item|{
            match item{
                Object::Cube(cube)=> cube.draw(gl),
                Object::Mesh(mesh)=> mesh.draw(gl),
                Object::Sphere(sphere)=> sphere.draw(gl),
                Object::Pill(pill)=> pill.draw(gl),
                Object::Cylinder(cyl)=> cyl.draw(gl),
            }
        }).collect();

    }

}