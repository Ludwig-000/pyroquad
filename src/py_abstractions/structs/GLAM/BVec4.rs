use glam::BVec4 as gl;

use pyo3::prelude::*;

use crate::py_assert;


/// An immutable Boolean Vector with 4 elements: x,y,z,w.
#[cfg_attr(feature = "abi_314", pyclass(eq,str,hash,frozen, immutable_type, from_py_object))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,hash,frozen, from_py_object))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct BVec4 {
    #[pyo3(get)]
    pub x: bool,
    #[pyo3(get)]
    pub y: bool,
    #[pyo3(get)]
    pub z: bool,
    #[pyo3(get)]
    pub w: bool,
}

impl BVec4 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: bool, y: bool, z: bool, w: bool) -> Self {
    BVec4 { x, y, z, w }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn const_splat(value: bool) -> Self {
    BVec4 { x: value, y: value, z: value, w: value }
    }
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

#[pymethods]
impl BVec4 {
    #[new]
    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self { x, y, z, w }
    }

    /// All false.
    #[classattr]
    pub const fn FALSE() -> BVec4 {
        Self::const_splat(false)
    }

    /// All true.
    #[classattr]
    pub const fn TRUE() -> BVec4 {
        Self::const_splat(true)
    }

    /// Creates a vector mask with all elements set to `value`.
    #[staticmethod]
    pub const fn splat(value: bool) -> Self {
        BVec4 { x: value, y: value, z: value, w: value }
    }

    /// Creates a new vector mask from a list of 4 bools.
    #[staticmethod]
    #[inline]
    pub const fn from_array(a: [bool; 4]) -> Self {
        Self { x: a[0], y: a[1], z: a[2], w: a[3] }
    }

    /// Returns a bitmask with the lowest 4 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub const fn bitmask(&self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2 | (self.w as u32) << 3
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub const fn any(&self) -> bool {
        self.x || self.y || self.z || self.w
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub const fn all(&self) -> bool {
        self.x && self.y && self.z && self.w
    }

    /// Returns the element at `index` (0, 1, 2 or 3).
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 3.
    #[inline]
    pub fn test(&self, index: usize) -> PyResult<bool> {
        py_assert!(index < 4, "index out of bounds.");
        Ok(self.into_bool_array()[index])
    }

    /// Returns a copy of `self` with the element at `index` (0, 1, 2 or 3) set to `value`.
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 3.
    #[inline]
    pub fn set(&self, index: usize, value: bool) -> PyResult<Self> {
        py_assert!(index < 4, "index out of bounds.");
        let mut a = self.into_bool_array();
        a[index] = value;
        Ok(Self::from_array(a))
    }

    /// `[x, y, z, w]`
    #[inline]
    pub const fn into_bool_array(&self) -> [bool; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// `[x, y, z, w]`, with `True` as `0xFFFFFFFF` and `False` as `0`.
    #[inline]
    pub const fn into_u32_array(&self) -> [u32; 4] {
        [
            MASK[self.x as usize],
            MASK[self.y as usize],
            MASK[self.z as usize],
            MASK[self.w as usize],
        ]
    }

    /// Element-wise `and`. Same as `self & rhs`.
    #[inline]
    pub const fn bitand(&self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
            w: self.w & rhs.w,
        }
    }

    /// Element-wise `or`. Same as `self | rhs`.
    #[inline]
    pub const fn bitor(&self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
            w: self.w | rhs.w,
        }
    }

    /// Element-wise `xor`. Same as `self ^ rhs`.
    #[inline]
    pub const fn bitxor(&self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
            w: self.w ^ rhs.w,
        }
    }

    /// Element-wise `not`. Same as `~self`.
    #[inline]
    pub const fn _not(&self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
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



impl std::fmt::Display for BVec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BVec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}




impl From<BVec4> for gl {
    fn from(v: BVec4) -> Self {
        gl::new(v.x, v.y, v.z, v.w)
    }
}


impl From<gl> for BVec4 {
    fn from(v: gl) -> Self {
        BVec4 {
        x: v.x,
        y: v.y,
        z: v.z,
        w: v.w,
        }
    }
}
