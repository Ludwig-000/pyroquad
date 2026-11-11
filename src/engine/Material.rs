/// Rust side of the material implementation
///
///
///


use std::{collections::HashMap, sync::{Mutex, atomic::{AtomicBool, Ordering}}};

use macroquad::prelude as mq;

use crate::engine::PArc::PArc;

/// now that is inside a PArc, it may be passed to the python thread.
/// though any calls still have to pass the queue.
#[derive(Clone, Debug)]
pub struct OuterMaterial{
    pub inner: PArc<InnerMaterial>,
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

    /// Only stores the value. It reaches macroquad in [`Self::sync_uniforms`], right before
    /// the next draw with this material. Engine thread only.
    pub fn set_uniform(&self, name: String, words: Vec<f32>) {
        self.inner.uniform_cache.lock().unwrap().insert(name.into_boxed_str(), words);
        self.inner.uniforms_dirty.store(true, Ordering::Relaxed);
    }

    /// MUST be called before every draw while this material is active.
    ///
    /// Each new draw call snapshots the material's uniforms, and rendering copies every
    /// snapshot back into the material. So a value set after the last draw would be rolled back
    /// by the next render (end of frame, camera change, `get_screen_data`).
    /// Writing the cache before the first draw after a set makes every snapshot match the cache,
    /// which makes the rollback harmless.
    #[inline]
    pub fn sync_uniforms(&self) {
        if !self.inner.uniforms_dirty.swap(false, Ordering::Relaxed) {
            return;
        }
        for (name, words) in self.inner.uniform_cache.lock().unwrap().iter() {
            self.inner.inner.set_uniform_array(name, &words[..]);
        }
    }
}

#[derive(Debug)]
pub struct InnerMaterial {
    pub inner: mq::Material,
    pub name: Box<str>,
    pub index: u64, // each material is assigned a number in shaderManager
    pub textures: Box<[Box<str>]>, // texture names declared at creation. Only these can be passed to `set_texture`.

    /// the last value set for each uniform, as the words handed to macroquad.
    /// macroquad's own copy is not reliable, see [`OuterMaterial::sync_uniforms`].
    pub uniform_cache: Mutex<HashMap<Box<str>, Vec<f32>>>,
    /// set when the cache changed and macroquad has not been given the new values yet.
    pub uniforms_dirty: AtomicBool,
    /// handles of the textures currently bound with `set_texture`, by name.
    /// macroquad's pipeline only stores the raw GPU id, which does not keep the texture alive,
    /// so the material holds the handles itself. Only touched on the engine thread.
    pub bound_textures: Mutex<HashMap<String, mq::Texture2D>>,
}
