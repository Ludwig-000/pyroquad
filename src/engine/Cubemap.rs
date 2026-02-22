
use crate::py_abstractions::Textures_and_Images::*;
use macroquad::prelude::UniformDesc;
use macroquad::prelude as mq;

use macroquad::texture::DrawTextureParams;
use macroquad::texture::draw_texture_ex;
use pyo3::prelude::*;
 
use pyo3_stub_gen::{derive::gen_stub_pyfunction};


use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

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

pub fn cubemap_internal_old(tex: Option<mq::Texture2D>, current_cam: &mq::Camera3D){
    
    let col = mq::Color::new(1.,1.,1.,1.);
    let pos = mq::vec3(0.,0.,0.);
    let siz = mq::vec3(10000.0, 10000.0,10000.0);
    mq::draw_cube(pos, siz, tex.as_ref(), mq::WHITE);
}



const SKYBOX_VERTEX_SHADER: &str = r#"#version 100
attribute vec3 position;

varying lowp vec3 v_dir;

// We pass a combined matrix that includes Projection * Rotation(View)
// We do NOT use the standard Model matrix or full View matrix.
uniform mat4 SkyViewProj;

void main() {
    // 1. Direction:
    // Since we draw the cube at (0,0,0), the vertex position IS the direction.
    v_dir = position;
    
    // 2. Position:
    // Apply our custom Rotation+Projection matrix
    vec4 pos = SkyViewProj * vec4(position, 1.0);
    
    // 3. Infinite Depth Trick:
    // Set z = w so that z/w = 1.0 (The far plane).
    gl_Position = pos.xyww;
}
"#;

const SKYBOX_FRAGMENT_SHADER: &str = r#"#version 100
precision mediump float;

varying vec3 v_dir;
uniform sampler2D Texture;

void main() {
    vec3 dir = normalize(v_dir);
    
    // Standard Equirectangular mapping
    // atan(z, x) is standard for forward-Z, but depending on texture source, 
    // you might need atan(x, z). This is the most common standard:
    vec2 uv = vec2(atan(dir.z, dir.x), asin(clamp(dir.y, -1.0, 1.0)));
    
    uv *= vec2(0.1591549, 0.3183098); // Inverse 2PI and PI
    uv += vec2(0.5, 0.5);
    
    // Vertical flip is common for some image formats
    // uv.y = 1.0 - uv.y;
    
    gl_FragColor = texture2D(Texture, uv);
}
"#;
use std::sync::{Mutex, OnceLock};

pub fn cubemap_internal(tex: Option<mq::Texture2D>, current_cam: &mq::Camera3D) {
    static SKYBOX_MAT: OnceLock<Mutex<Option<mq::Material>>> = OnceLock::new();

    let mutex = SKYBOX_MAT.get_or_init(|| Mutex::new(None));
    let mut guard = mutex.lock().unwrap();

    if guard.is_none() {
        let pipeline_params = mq::PipelineParams {
            depth_write: false, 
            depth_test: mq::miniquad::Comparison::LessOrEqual,
            cull_face: mq::miniquad::CullFace::Nothing, 
            ..Default::default()
        };
        let mat = mq::load_material(
            mq::ShaderSource::Glsl {
                vertex: SKYBOX_VERTEX_SHADER,
                fragment: SKYBOX_FRAGMENT_SHADER,
            },
            mq::MaterialParams {
                pipeline_params,
                uniforms: vec![UniformDesc::new("SkyViewProj",  mq::UniformType::Mat4)],
                ..Default::default()
            },
        ).unwrap();

        *guard = Some(mat);
    }

    if let Some(ref material) = *guard {
        mq::gl_use_material(material);
        let aspect = current_cam.aspect.unwrap_or(mq::screen_width() / mq::screen_height());
        let proj = mq::Mat4::perspective_rh_gl(current_cam.fovy, aspect, current_cam.z_near, current_cam.z_far);

        let view_rot_only = mq::Mat4::look_at_rh(
            mq::Vec3::ZERO, 
            current_cam.target - current_cam.position, 
            current_cam.up
        );

        let sky_view_proj = proj * view_rot_only;

        material.set_uniform("SkyViewProj", sky_view_proj);

        mq::draw_cube(mq::Vec3::ZERO, mq::vec3(10.0, 10.0, 10.0), tex.as_ref(), mq::WHITE);

        mq::gl_use_default_material();
    }
}