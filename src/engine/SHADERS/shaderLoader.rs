use macroquad::prelude::*;
use macroquad::material::{Material, MaterialParams};
use macroquad::window::miniquad::{ShaderSource, PipelineParams, Comparison, UniformDesc, UniformType};
use std::sync::Mutex;

pub fn shader_load() {
    {
        const FRAGMENT_SHADER_SRC: &str = include_str!("Basic/FRAG.frag");
        const VERTEX_SHADER_SRC: &str = include_str!("Basic/VERTEX.vert");

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material: Material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER_SRC,
                fragment: FRAGMENT_SHADER_SRC,
            },
            MaterialParams {
                pipeline_params,
                uniforms: vec![UniformDesc::new("LightDir", UniformType::Float3)],
                ..Default::default()
            },
        ).expect("Shader compilation failed");

        // Light direction: North-East (azimuth 45), elevation +15:
        let elev = 15f32.to_radians();
        let azim = 60f32.to_radians(); // NE
        let lx = elev.cos() * azim.cos();
        let ly = elev.sin();
        let lz = elev.cos() * azim.sin();
        let light_dir = vec3(lx, ly, lz);
        material.set_uniform("LightDir", (light_dir.x, light_dir.y, light_dir.z));
        store_shader(material);
    }

    {
        const GRAGMENT_SHADER_SKYBOX: &str = include_str!("Skybox/FRAG.frag");
        const VERTEX_SHADER_SKYBOX: &str = include_str!("Skybox/VERTEX.vert");
        let skybox_material = load_material(
            ShaderSource::Glsl {
            vertex: VERTEX_SHADER_SKYBOX,
            fragment: GRAGMENT_SHADER_SKYBOX,
        },
            MaterialParams {
                // tell MQ to bind a cubemap to "u_skybox"
                textures: vec!["u_skybox".to_string()],
                ..Default::default()
            },
        ).unwrap();
        store_shader(skybox_material);
    }
}







// "Material" is an Reference Counter, so we need to store a reference if we want to use it.
lazy_static::lazy_static! {
    pub static ref SHADERS: Mutex<Vec<Material>> = Mutex::new(Vec::new());
}

pub fn store_shader(material: Material) {
    SHADERS.lock().unwrap().push(material);
}

pub fn get_shader(index: usize) -> Option<Material> {
    SHADERS.lock().unwrap().get(index).cloned()
}
