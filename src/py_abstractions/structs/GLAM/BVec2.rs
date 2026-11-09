
use glam::BVec2 as gl;

use pyo3::prelude::*;

use crate::py_assert;


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


const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

#[pymethods]
impl BVec2 {
    
    #[new]
    #[inline(always)]
    pub const fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }


    /// All false.
    #[classattr]
    pub const fn FALSE() -> BVec2 {
        Self::splat(false)
    }

    /// All true.
    #[classattr]
    pub const fn TRUE() -> BVec2 {
        Self::splat(true)
    }
    
    /// Creates a vector mask with all elements set to `value`.
    #[staticmethod]
    pub const fn splat(value: bool) -> Self {
        BVec2 { x: value, y: value }
    }

    /// Creates a new vector mask from a list of 2 bools.
    #[staticmethod]
    #[inline]
    pub const fn from_array(a: [bool; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }

    /// Returns a bitmask with the lowest 2 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second.
    #[inline]
    pub const fn bitmask(&self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub const fn any(&self) -> bool {
        self.x || self.y
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub const fn all(&self) -> bool {
        self.x && self.y
    }

    /// Returns the element at `index` (0 or 1).
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 1.
    #[inline]
    pub fn test(&self, index: usize) -> PyResult<bool> {
        py_assert!(index < 2, "index out of bounds.");
        Ok(if index == 0 { self.x } else { self.y })
    }

    /// Returns a copy of `self` with the element at `index` (0 or 1) set to `value`.
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 1.
    #[inline]
    pub fn set(&self, index: usize, value: bool) -> PyResult<Self> {
        py_assert!(index < 2, "index out of bounds.");
        Ok(if index == 0 { Self::new(value, self.y) } else { Self::new(self.x, value) })
    }

    /// `[x, y]`
    #[inline]
    pub const fn into_bool_array(&self) -> [bool; 2] {
        [self.x, self.y]
    }

    /// `[x, y]`, with `True` as `0xFFFFFFFF` and `False` as `0`.
    #[inline]
    pub const fn into_u32_array(&self) -> [u32; 2] {
        [MASK[self.x as usize], MASK[self.y as usize]]
    }

    /// Element-wise `and`. Same as `self & rhs`.
    #[inline]
    pub const fn bitand(&self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }

    /// Element-wise `or`. Same as `self | rhs`.
    #[inline]
    pub const fn bitor(&self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }

    /// Element-wise `xor`. Same as `self ^ rhs`.
    #[inline]
    pub const fn bitxor(&self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }

    /// Element-wise `not`. Same as `~self`.
    #[inline]
    pub const fn _not(&self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }

    const fn __and__(&self, rhs: Self) -> Self {
        self.bitand(rhs)
    }

    const fn __or__(&self, rhs: Self) -> Self {
        self.bitor(rhs)
    }

    const fn __xor__(&self, rhs: Self) -> Self {
        self.bitxor(rhs)
    }

    const fn __invert__(&self) -> Self {
        self._not()
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