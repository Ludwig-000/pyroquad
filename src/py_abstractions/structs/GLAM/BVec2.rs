
use glam::BVec2 as gl;

use pyo3::prelude::*;


/// An immutable Boolean Vector with 2 elements: x,y.
#[cfg_attr(feature = "abi_314", pyclass(eq,str,hash,frozen, immutable_type, from_py_object))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,hash,frozen, from_py_object))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq,Debug,Eq, Hash)]
pub struct BVec2 {
    #[pyo3(get)]
    pub x: bool,
    #[pyo3(get)]
    pub y: bool,
}

impl BVec2 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: bool, y: bool) -> Self {
    BVec2 { x, y }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn const_splat(value: bool) -> Self {
    BVec2 { x: value, y: value }
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 2]) -> Self {
        Self { x: a[0],y :  a[1] }
    }
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

#[pymethods]
impl BVec2 {
    
    #[new]
    pub fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    /// All false.
    #[classattr]
    fn FALSE() -> BVec2 {
        Self::const_splat(false)
    }

    /// All true.
    #[classattr]
    fn TRUE() -> BVec2 {
        Self::const_splat(true)
    }
    
    #[staticmethod]
    pub fn splat(value: bool) -> Self {
        BVec2 { x: value, y: value }
    }
    
    #[inline]
    pub fn bitmask(&self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(&self) -> bool {
        self.x || self.y
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(&self) -> bool {
        self.x && self.y
    }

    /// Returns the value of index 1 or index 2.
    #[inline]
    pub fn test(&self, index: usize) -> bool {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("index out of bounds"),
        }
    }

    /// Sets the value of index 1 or index 2.
    #[inline]
    pub fn set(&self, index: usize, value: bool)-> Self {
        match index {
            0 => Self::const_new(value, self.y),
            1 => Self::const_new(self.x, value),
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    pub fn into_bool_array(&self) -> [bool; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub fn into_u32_array(&self) -> [u32; 2] {
        [MASK[self.x as usize], MASK[self.y as usize]]
    }

    #[inline]
    pub fn bitand(&self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }

    #[inline]
    pub fn bitor(&self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }

    #[inline]
    fn bitxor(&self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }

    #[inline]
    fn _not(&self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }


}




impl std::fmt::Display for BVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BVec2({}, {})", self.x, self.y)
    }
}



impl From<BVec2> for gl {
    fn from(v: BVec2) -> Self {
        gl::new(v.x, v.y)
    }
}


impl From<gl> for BVec2 {
    fn from(v: gl) -> Self {
        BVec2 {
        x: v.x,
        y: v.y,
        }
    }
}