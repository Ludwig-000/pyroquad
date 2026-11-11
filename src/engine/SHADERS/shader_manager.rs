use macroquad::{material::MaterialParams, prelude as mq};

use crate::{_pyroquad::UniformDesc, engine::{CameraManager::clone_camera3d, Material::{InnerMaterial, OuterMaterial}, PError::PError}};

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum Premade_Shader_Type {
    None = 0,
    Basic = 1,
    SkyBox = 2,
}

/// all shader switches are lazy, since doing them is expensive.
pub struct ShaderManager {
    current_shader: u64,
    shader_refs: Vec<OuterMaterial>, // keep a strong ref to all materials used this frame to prevent drops.

    builtin_shader_overwritten_by_custom_shader: bool,
    
    top_shader_index: u64, // increases for each new material
    
    basic_shader: mq::Material,
    skybox_shader: mq::Material,
}

impl ShaderManager {
    pub fn new(basic_shader: mq::Material, skybox_shader: mq::Material) -> ShaderManager{
        ShaderManager { 
            current_shader: 0, 
            shader_refs: Vec::new(),
            builtin_shader_overwritten_by_custom_shader: false, 
            top_shader_index: 2 ,
            basic_shader, 
            skybox_shader 
        }
    }


    /// each draw call inherently either wants a 2d or 3d shader.
    /// custom shaders are designated on top of these.
    pub fn set_desired_builtin_shader(&mut self, shader: Premade_Shader_Type, current_cam: &Option<macroquad::prelude::Camera3D>){
        if shader as u64 == self.current_shader || self.builtin_shader_overwritten_by_custom_shader {
            return;
        }


        match shader {
            Premade_Shader_Type::None => {mq::gl_use_default_material(); self.current_shader=0;},
            Premade_Shader_Type::Basic => {mq::gl_use_material( &self.basic_shader ); self.current_shader=1;},
            Premade_Shader_Type::SkyBox => {
                mq::gl_use_material( &self.skybox_shader ); 
                self.current_shader=2;
                
                let current_cam = clone_camera3d(current_cam.as_ref().unwrap());
                let aspect = current_cam.aspect.unwrap_or(mq::screen_width() / mq::screen_height());
                let proj = mq::Mat4::perspective_rh_gl(current_cam.fovy, aspect, current_cam.z_near, current_cam.z_far);

                let view_rot_only = mq::Mat4::look_at_rh(
                    mq::Vec3::ZERO, 
                    current_cam.target - current_cam.position, 
                    current_cam.up
                );
                let sky_view_proj = proj * view_rot_only;
                self.skybox_shader.set_uniform("SkyViewProj", sky_view_proj);

            },
        }
    }

    pub fn new_material(&mut self, vert: String, frag: String,  name: String , uniforms: Vec<UniformDesc>, textures: Vec<String>, pipeline_params: mq::PipelineParams) -> Result<OuterMaterial, PError> {

        let shader_source = mq::ShaderSource::Glsl { vertex: &vert, fragment: &frag };

        let uniforms: Vec<mq::UniformDesc> = uniforms.iter().map(UniformDesc::to_mq).collect();

        let params = MaterialParams {
            pipeline_params,
            uniforms,
            textures: textures.clone(),
            ..Default::default()
        };
        let shader= mq::load_material(shader_source, params)?;

        self.top_shader_index+=1;

        Ok(OuterMaterial::new(InnerMaterial {
            inner: shader,
            name: name.into_boxed_str(),
            index: self.top_shader_index,
            cached_Uniforms: Vec::new(),
            textures: textures.into_iter().map(String::into_boxed_str).collect(),
            bound_textures: Default::default(),
        }))
    }


    pub fn set_custom_shader(&mut self, mat: OuterMaterial) {
        self.builtin_shader_overwritten_by_custom_shader = true;

        if self.current_shader == mat.index() {
            return;
        }
        self.current_shader = mat.index();

        mq::gl_use_material(&mat.inner.inner);

        self.shader_refs.push(mat);

    }

    pub fn use_default_material(&mut self) {
        self.builtin_shader_overwritten_by_custom_shader = false;
    }

    /// called inside next_frame
    pub fn new_frame_shader_update(&mut self) {

        self.current_shader = Premade_Shader_Type::None as u64;
        self.builtin_shader_overwritten_by_custom_shader = false;

        mq::gl_use_default_material();
    }


    pub fn garbage_collect(&mut self) {
        self.shader_refs = Vec::new();
    }
}

