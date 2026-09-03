use pyo3::{pyclass, pymethods};


use crate::py_abstractions::structs::GLAM::Vec4::Vec4;



/// A minimal pyclass for basic Quat usage.
#[cfg_attr(feature = "abi_314", pyclass(eq,str,frozen, immutable_type, from_py_object))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,frozen, from_py_object))]
#[derive(Clone, Copy, PartialEq,Debug)]
pub struct Quat {
    #[pyo3(get)]
    pub x: f32,
    #[pyo3(get)]
    pub y: f32,
    #[pyo3(get)]
    pub z: f32,
    #[pyo3(get)]
    pub w: f32,
}

#[pymethods]
impl Quat{


    /// All zeros.
    #[classattr]
    pub fn ZERO() -> Quat { 
        Self::from_array([0.0; 4])
    }

    /// The identity quaternion. Corresponds to no rotation.
    #[classattr]
    pub fn IDENTITY() -> Quat {
        Self::from_xyzw(0.0, 0.0, 0.0, 1.0)
    }

    /// All NANs.
    #[classattr]
    pub fn NAN() -> Quat {
        Self::from_array([f32::NAN; 4])
    }

    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing.
    /// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    /// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    #[staticmethod]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a rotation quaternion from an array.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    #[must_use]
    #[staticmethod]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }

    /// Creates a new rotation quaternion from a 4D vector.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    #[must_use]
    #[staticmethod]
    pub const fn from_vec4(v: Vec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}



impl From<macroquad::prelude::Quat> for Quat{
    fn from(v: macroquad::prelude::Quat) -> Self {
        Quat::from_xyzw(v.x, v.y, v.z, v.w)
    }
}


impl From<Quat> for macroquad::prelude::Quat{
    fn from(v: Quat) -> macroquad::prelude::Quat {
        macroquad::prelude::Quat::from_xyzw(v.x, v.y, v.z, v.w)
    }
}



impl std::fmt::Display for Quat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}
