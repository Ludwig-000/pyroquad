
use glam::Vec4 as gl;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// Barebones Vector type, in order to implement Mat4
/// TODO: complete with full functionality
#[gen_stub_pyclass]
#[cfg_attr(feature = "abi_314", pyclass(eq,str,frozen, immutable_type))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,frozen))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq,Debug)]
pub struct Vec4 {
    #[pyo3(get)]
    pub x: f32,
    #[pyo3(get)]
    pub y: f32,
    #[pyo3(get)]
    pub z: f32,
    #[pyo3(get)]
    pub w: f32,
}

impl Vec4 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: f32, y: f32, z: f32, w:f32) -> Self {
        Vec4 { x, y, z, w }
    }

    // Const constructor for splat values
    #[inline(always)]
    pub const fn const_splat(value: f32) -> Self {
        Self::const_new(value,value,value,value)
    }

    #[inline]
    #[must_use]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self { x: a[0],y :  a[1],z:  a[2], w: a[3]   }
    }
}


#[gen_stub_pymethods]
#[pymethods]
impl Vec4 {
    #[new]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::const_new(x, y, z, w)
    }

    /// All 0.
    #[classattr]
    fn ZERO() -> Vec4 {
        Self::const_splat(0.0)
    }

    /// All 1.
    #[classattr]
    fn ONE() -> Vec4 {
        Self::const_splat(1.0)
    }

    #[staticmethod]
    pub fn splat(value: f32) -> Self {
        Self::const_splat(value)
    }
}



impl std::fmt::Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}




impl From<Vec4> for gl {
    fn from(v: Vec4) -> Self {
        gl::new(v.x, v.y, v.z, v.w)
    }
}


impl From<gl> for Vec4 {
    fn from(v: gl) -> Self {
        Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w
        }
    }
}