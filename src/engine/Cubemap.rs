
use crate::py_abstractions::Textures_and_Images::*;
use macroquad::prelude as mq;

use macroquad::texture::DrawTextureParams;
use macroquad::texture::draw_texture_ex;
use pyo3::prelude::*;
 
use pyo3_stub_gen::{derive::gen_stub_pyfunction};


use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;


// TODO!
// convert the 3d cam to a 2d cam, apply a shader on the matrix, then re-apply the 3d cam
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cubemap(texture: Texture2D) {

    let col = mq::Color::new(1.,1.,1.,1.);
    let pos = mq::vec3(0.,0.,0.);
    let siz = mq::vec3(10000.0, 10000.0,10000.0);
    let texture_unpacked = texture.into();
    COMMAND_QUEUE.push(  Command::DrawCubemap{
        texture: Some(texture_unpacked)} );
}

pub fn cubemap_internal(tex: Option<mq::Texture2D>){
    
    let col = mq::Color::new(1.,1.,1.,1.);
    let pos = mq::vec3(0.,0.,0.);
    let siz = mq::vec3(10000.0, 10000.0,10000.0);
    mq::draw_cube(pos, siz, tex.as_ref(), mq::WHITE);
}
