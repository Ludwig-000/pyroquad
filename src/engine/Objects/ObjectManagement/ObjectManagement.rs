use crate::engine::Objects::ObjectManagement::{ObjectStorage::*};
use macroquad::prelude as mq;


pub fn draw_all_Objects(obj: &ObjectStorage, viewMat: macroquad::prelude::Mat4){

    unsafe {

        let gl: &mut macroquad::prelude::QuadGl  = macroquad::prelude::get_internal_gl().quad_gl;
        gl.draw_mode(mq::DrawMode::Triangles);


        obj.iter().for_each(|item|{
            match item {
                Object::Cube(cube) => {
                    if cube.draw_each_frame { cube.draw(gl); }
                },
                Object::Mesh(mesh) => {
                    if mesh.draw_each_frame { mesh.draw(gl); }
                },
                Object::Sphere(sphere) => {
                    if sphere.draw_each_frame { sphere.draw(gl); }
                },
                Object::Pill(pill) => {
                    if pill.draw_each_frame { pill.draw(gl); }
                },
                Object::Cylinder(cyl) => {
                    if cyl.draw_each_frame { cyl.draw(gl); }
                },
            }
        });

    }

}