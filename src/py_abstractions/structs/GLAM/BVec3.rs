
use glam::BVec3 as gl;

use pyo3::prelude::*;


/// An immutable Boolean Vector with 3 elements: x,y,z.
#[cfg_attr(feature = "abi_314", pyclass(eq,str,hash,frozen, immutable_type, from_py_object))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,hash,frozen, from_py_object))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct BVec3 {
    #[pyo3(get)]
    pub x: bool,
    #[pyo3(get)]
    pub y: bool,
    #[pyo3(get)]
    pub z: bool,
}

impl BVec3 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: bool, y: bool, z: bool) -> Self {
    BVec3 { x, y, z }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn const_splat(value: bool) -> Self {
    BVec3 { x: value, y: value, z: value }
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 3]) -> Self {
        Self { x: a[0],y :  a[1],z:  a[2]   }
    }
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

#[pymethods]
impl BVec3 {
    #[new]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    /// All false.
    #[classattr]
    fn FALSE() -> BVec3 {
        Self::const_splat(false)
    }

    /// All true.
    #[classattr]
    fn TRUE() -> BVec3 {
        Self::const_splat(true)
    }

    #[staticmethod]
    pub fn splat(value: bool) -> Self {
        BVec3 { x: value, y: value, z: value }
    }

    #[inline]
    pub fn bitmask(&self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.x || self.y || self.z
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(&self) -> bool {
        self.x && self.y && self.z
    }

    #[inline]
    pub fn test(&self, index: usize) -> bool {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    pub fn set(&self, index: usize, value: bool) -> Self{
        match index {
            0 => Self::const_new(value, self.y, self.z),
            1 => Self::const_new(self.x, value, self.z),
            2 => Self::const_new(self.x, self.y, value),
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    pub fn into_bool_array(&self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn into_u32_array(&self) -> [u32; 3] {
        [
            MASK[self.x as usize],
            MASK[self.y as usize],
            MASK[self.z as usize],
        ]
    }

    #[inline]
    pub fn bitand(&self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
        }
    }

    #[inline]
    pub fn bitor(&self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
        }
    }

    #[inline]
    fn bitxor(&self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
        }
    }

    #[inline]
    fn _not(&self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }


}



impl std::fmt::Display for BVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BVec3({}, {}, {})", self.x, self.y, self.z)
    }
}




impl From<BVec3> for gl {
    fn from(v: BVec3) -> Self {
        gl::new(v.x, v.y, v.z)
    }
}


impl From<gl> for BVec3 {
    fn from(v: gl) -> Self {
        BVec3 {
        x: v.x,
        y: v.y,
        z: v.z,
        }
    }
}