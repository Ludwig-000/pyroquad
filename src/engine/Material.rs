/// Rust side of the material implementation
///
///
///


use macroquad::prelude as mq;

use crate::{_pyroquad::UniformType, engine::PArc::PArc};

/// now that is inside a PArc, it may be passed to the python thread.
/// though any calls still have to pass the queue.
#[derive(Clone, Debug)]
pub struct OuterMaterial{
    pub inner: PArc<InnerMaterial>,
    /// read-only outside this module, use `index()`.
    index: u64,
}

impl OuterMaterial {
    pub fn new(inner: InnerMaterial)->Self{
        let ndex= inner.index;
        OuterMaterial {
            inner: PArc::new(inner),
            index: ndex
        }
    }

    /// the number shaderManager assigned to this material.
    #[inline(always)]
    pub fn index(&self) -> u64 { self.index }
}

#[derive(Debug)]
pub struct InnerMaterial {
    pub inner: mq::Material,
    pub name: Box<str>,
    pub index: u64, // each material is assigned a number in shaderManager
    pub textures: Box<[Box<str>]>, // texture names declared at creation. Only these can be passed to `set_texture`.


    pub cached_Uniforms: Vec<UniformType>,
    /// handles of the textures currently bound with `set_texture`, by name.
    /// macroquad's pipeline only stores the raw GPU id, which does not keep the texture alive,
    /// so the material holds the handles itself. Only touched on the engine thread.
    pub bound_textures: std::sync::Mutex<std::collections::HashMap<String, mq::Texture2D>>,
}