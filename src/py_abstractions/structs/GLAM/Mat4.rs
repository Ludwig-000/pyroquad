use glam::Mat4 as gl;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use crate::py_abstractions::structs::GLAM::Vec4::Vec4;

/// Barebones Mat4 type, in order to implement Viewport stuff.
/// TODO: complete with full functionality
#[gen_stub_pyclass]
#[cfg_attr(feature = "abi_314", pyclass(eq,str,frozen, immutable_type))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,frozen))]
#[derive(Clone,Copy,PartialEq,Debug)]
#[repr(C)]
pub struct Mat4 {
    #[pyo3(get)]
    pub x_axis: Vec4,
    #[pyo3(get)]
    pub y_axis: Vec4,
    #[pyo3(get)]
    pub z_axis: Vec4,
    #[pyo3(get)]
    pub w_axis: Vec4,
}

impl Mat4 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x_axis: Vec4,y_axis: Vec4,z_axis: Vec4,w_axis: Vec4,) -> Self {
        Mat4 { x_axis, y_axis, z_axis, w_axis }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn const_splat(value: Vec4) -> Self {
        Mat4 { x_axis: value, y_axis: value, z_axis: value, w_axis: value }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Mat4 {
    
    #[new]
    pub fn new(x_axis: Vec4,y_axis: Vec4,z_axis: Vec4,w_axis: Vec4,) -> Self {
        Self::const_new(x_axis, y_axis, z_axis, w_axis)
    }



}




impl std::fmt::Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat4({}, {}, {}, {})", self.x_axis, self.y_axis, self.z_axis, self.w_axis)
    }
}



impl From<Mat4> for gl {
    fn from(v: Mat4) -> Self {
        gl::from_cols(
            v.x_axis.into(),
            v.y_axis.into(),
            v.z_axis.into(),
            v.w_axis.into()
        )
    }
}


impl From<gl> for Mat4 {
    fn from(v: gl) -> Self {
        Mat4::const_new(
            v.x_axis.into(),
            v.y_axis.into(),
            v.z_axis.into(),
            v.w_axis.into()
        )
    }
}