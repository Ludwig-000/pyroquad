use glam::BVec3 as gl;

use pyo3::prelude::*;

use crate::py_assert;


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
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

#[pymethods]
impl BVec3 {
    #[new]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    /// All false.
    #[classattr]
    pub const fn FALSE() -> BVec3 {
        Self::const_splat(false)
    }

    /// All true.
    #[classattr]
    pub const fn TRUE() -> BVec3 {
        Self::const_splat(true)
    }

    /// Creates a vector mask with all elements set to `value`.
    #[staticmethod]
    pub const fn splat(value: bool) -> Self {
        BVec3 { x: value, y: value, z: value }
    }

    /// Creates a new vector mask from a list of 3 bools.
    #[staticmethod]
    #[inline]
    pub const fn from_array(a: [bool; 3]) -> Self {
        Self { x: a[0], y: a[1], z: a[2] }
    }

    /// Returns a bitmask with the lowest 3 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub const fn bitmask(&self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub const fn any(&self) -> bool {
        self.x || self.y || self.z
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub const fn all(&self) -> bool {
        self.x && self.y && self.z
    }

    /// Returns the element at `index` (0, 1 or 2).
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 2.
    #[inline]
    pub fn test(&self, index: usize) -> PyResult<bool> {
        py_assert!(index < 3, "index out of bounds.");
        Ok(self.into_bool_array()[index])
    }

    /// Returns a copy of `self` with the element at `index` (0, 1 or 2) set to `value`.
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 2.
    #[inline]
    pub fn set(&self, index: usize, value: bool) -> PyResult<Self> {
        py_assert!(index < 3, "index out of bounds.");
        let mut a = self.into_bool_array();
        a[index] = value;
        Ok(Self::from_array(a))
    }

    /// `[x, y, z]`
    #[inline]
    pub const fn into_bool_array(&self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    /// `[x, y, z]`, with `True` as `0xFFFFFFFF` and `False` as `0`.
    #[inline]
    pub const fn into_u32_array(&self) -> [u32; 3] {
        [
            MASK[self.x as usize],
            MASK[self.y as usize],
            MASK[self.z as usize],
        ]
    }

    /// Element-wise `and`. Same as `self & rhs`.
    #[inline]
    pub const fn bitand(&self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
        }
    }

    /// Element-wise `or`. Same as `self | rhs`.
    #[inline]
    pub const fn bitor(&self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
        }
    }

    /// Element-wise `xor`. Same as `self ^ rhs`.
    #[inline]
    pub const fn bitxor(&self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
        }
    }

    /// Element-wise `not`. Same as `~self`.
    #[inline]
    pub const fn _not(&self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
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
