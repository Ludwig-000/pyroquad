use macroquad::prelude as mq;
use macroquad::material::{Material, MaterialParams};
use macroquad::window::miniquad::{ShaderSource, PipelineParams, Comparison, UniformDesc, UniformType};
use std::sync::{LazyLock, Mutex};

pub fn shader_load() {
    {
        const FRAGMENT_SHADER_SRC: &str = include_str!("Basic/FRAG.frag");
        const VERTEX_SHADER_SRC: &str = include_str!("Basic/VERTEX.vert");

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material: Material = mq::load_material(
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
        let light_dir = mq::vec3(lx, ly, lz);
        material.set_uniform("LightDir", (light_dir.x, light_dir.y, light_dir.z));
        store_shader(material);
    }

    {
        const FRAG_SKYBOX: &str = include_str!("Skybox/FRAG.frag");
        const VERT_SKYBOX: &str = include_str!("Skybox/VERTEX.vert");


        let pipeline_params = mq::PipelineParams {
            depth_write: false, 
            depth_test: mq::miniquad::Comparison::LessOrEqual,
            cull_face: mq::miniquad::CullFace::Nothing, 
            ..Default::default()
        };
        let mat = mq::load_material(
            mq::ShaderSource::Glsl {
                vertex: VERT_SKYBOX,
                fragment: FRAG_SKYBOX,
            },
            mq::MaterialParams {
                pipeline_params,
                uniforms: vec![UniformDesc::new("SkyViewProj",  mq::UniformType::Mat4)],
                ..Default::default()
            },
        ).unwrap();
        store_shader(mat);
    }
}







// "Material" is an Reference Counter, so we need to store a reference if we want to use it.
pub static SHADERS: LazyLock<Mutex<Vec<Material>>> = 
    LazyLock::new(|| Mutex::new(Vec::new()));

pub fn store_shader(material: Material) {
    SHADERS.lock().unwrap().push(material);
}

pub fn get_shader(index: usize) -> Option<Material> {
    SHADERS.lock().unwrap().get(index).cloned()
}
