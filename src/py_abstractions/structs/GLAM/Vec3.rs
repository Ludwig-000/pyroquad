use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::structs::GLAM::BVec3::BVec3;
use glam::Vec3 as gl;
use glam::Vec2 as glVec2;
use pyo3::prelude::*;
use std::fmt;
use pyo3_stub_gen::derive::*;


//
// A python abstraction for the Vec3 struct from the GLAM crate.
// This file implements all functionality from Glam, replacing uses of BVec3 and Vec2 with the pyabstracted versions.
//
#[gen_stub_pyclass]
#[cfg_attr(feature = "abi_314", pyclass(eq,str,frozen, immutable_type))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,frozen))]
#[derive(Clone, Copy, PartialEq,Debug)]
pub struct Vec3 {
    #[pyo3(get)]
    pub x: f32,
    #[pyo3(get)]
    pub y: f32,
    #[pyo3(get)]
    pub z: f32,
}

impl Vec3 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn const_splat(value: f32) -> Self {
        Vec3 { x: value, y: value, z: value }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Vec3 {
    #[new]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// A vector with all elements set to `0.0`.
    #[classattr]
    pub fn ZERO() -> Vec3 {
        Self::const_splat(0.0)
    }

    /// A vector with all elements set to `1.0`.
    #[classattr]
    pub fn ONE() -> Vec3 {
        Self::const_splat(1.0)
    }

    /// A vector with all elements set to `-1.0`.
    #[classattr]
    pub fn NEG_ONE() -> Vec3 {
        Self::const_splat(-1.0)
    }

    /// A vector with all elements set to `f32::MIN`.
    #[classattr]
    pub fn MIN() -> Vec3 {
        Self::const_splat(f32::MIN)
    }

    /// A vector with all elements set to `f32::MAX`.
    #[classattr]
    pub fn MAX() -> Vec3 {
        Self::const_splat(f32::MAX)
    }

    /// A vector with all elements set to `f32::NAN`.
    #[classattr]
    pub fn NAN() -> Vec3 {
        Self::const_splat(f32::NAN)
    }

    /// A vector with all elements set to `f32::INFINITY`.
    #[classattr]
    pub fn INFINITY() -> Vec3 {
        Self::const_splat(f32::INFINITY)
    }

    /// A vector with all elements set to `f32::NEG_INFINITY`.
    #[classattr]
    pub fn NEG_INFINITY() -> Vec3 {
        Self::const_splat(f32::NEG_INFINITY)
    }

    /// The unit vector in the X direction `(1.0, 0.0, 0.0)`.
    #[classattr]
    pub fn X() -> Vec3 {
        Self::const_new(1.0, 0.0, 0.0)
    }

    /// The unit vector in the Y direction `(0.0, 1.0, 0.0)`.
    #[classattr]
    pub fn Y() -> Vec3 {
        Self::const_new(0.0, 1.0, 0.0)
    }

    /// The unit vector in the Z direction `(0.0, 0.0, 1.0)`.
    #[classattr]
    pub fn Z() -> Vec3 {
        Self::const_new(0.0, 0.0, 1.0)
    }

    /// The unit vector in the negative X direction `(-1.0, 0.0, 0.0)`.
    #[classattr]
    pub fn NEG_X() -> Vec3 {
        Self::const_new(-1.0, 0.0, 0.0)
    }

    /// The unit vector in the negative Y direction `(0.0, -1.0, 0.0)`.
    #[classattr]
    pub fn NEG_Y() -> Vec3 {
        Self::const_new(0.0, -1.0, 0.0)
    }

    /// The unit vector in the negative Z direction `(0.0, 0.0, -1.0)`.
    #[classattr]
    pub fn NEG_Z() -> Vec3 {
        Self::const_new(0.0, 0.0, -1.0)
    }

    /// The X, Y, and Z unit vectors as a list `[X, Y, Z]`.
    #[classattr]
    pub fn AXES() -> Vec<Vec3> {
        vec![Self::X(), Self::Y(), Self::Z()]
    }



    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[staticmethod]
    #[inline]
    pub fn select(mask: &BVec3, if_true: Self, if_false: Self) -> Self {
        
        Self {
            x: if mask.test(0) { if_true.x } else { if_false.x },
            y: if mask.test(1) { if_true.y } else { if_false.y },
            z: if mask.test(2) { if_true.z } else { if_false.z },
        }
    }

    /// Creates a new vector from an array.
    #[staticmethod]
    #[inline]
    pub fn from_array(a: [f32; 3]) -> Self {
        Self::const_new(a[0], a[1], a[2])
    }

    // constructor for splat values
    #[staticmethod]
    #[inline(always)]
    pub fn splat(value: f32) -> Vec3 {
        Vec3 { x: value, y: value, z: value }
    }

    /// `[x, y, z]`
    #[inline]
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn truncate(&self) -> Vec2 {
        let s: gl=  (*self).into();
        let g2: glVec2 = s.truncate();
        g2.into()         
    }


    /// Creates a 3D vector from `self` with the given value of `x`.
    #[inline]
    pub fn with_x(&self, x: f32) -> Self {
        Self::const_new(x, self.y, self.z)
    }

    /// Creates a 3D vector from `self` with the given value of `y`.
    #[inline]
    pub fn with_y(&self, y: f32) -> Self {
        Self::const_new(self.x, y, self.z)
    }

    /// Creates a 3D vector from `self` with the given value of `z`.
    #[inline]
    pub fn with_z(&self, z: f32) -> Self {
        Self::const_new(self.x, self.y, z)
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> f32 {
        glam::Vec3::from(*self).dot(glam::Vec3::from(rhs))
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot_into_vec(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        a.dot_into_vec(rhs.into()).into()     
    }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        a.cross(rhs.into()).into()     
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    pub fn min(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        a.min(rhs.into()).into()   
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    pub fn max(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        a.max(rhs.into()).into()   
    }


    
    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = min.into();
        let c: gl = max.into();
        a.clamp(b,c).into() 
    }


    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    pub fn min_element(&self) -> f32 {
        let a: gl=(*self).into();
        a.min_element()
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    pub fn max_element(&self) -> f32 {
        let a: gl=(*self).into();
        a.max_element()
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    pub fn element_sum(&self) -> f32 {
        let a: gl=(*self).into();
        a.element_sum()
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    pub fn element_product(&self) -> f32 {
        let a: gl=(*self).into();
        a.element_product()
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpeq(&self, rhs: Self) -> BVec3 {
        let a: gl =  (*self).into();
        let b: gl =  rhs.into();
        a.cmpeq(b).into()
        
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpne(&self, rhs: Self) -> BVec3 {
        let a: gl =  (*self).into();
        let b: gl =  rhs.into();
        a.cmpne(b).into()
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpge(&self, rhs: Self) -> BVec3 {
        let a: gl =  (*self).into();
        let b: gl =  rhs.into();
        a.cmpge(b).into()
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpgt(&self, rhs: Self) -> BVec3 {
        let a: gl =  (*self).into();
        let b: gl =  rhs.into();
        a.cmpgt(b).into()
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmple(&self, rhs: Self) -> BVec3 {
        let a: gl =  (*self).into();
        let b: gl =  rhs.into();
        a.cmple(b).into()
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmplt(&self, rhs: Self) -> BVec3 {
        let a: gl =  (*self).into();
        let b: gl =  rhs.into();
        a.cmplt(b).into()
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(&self) -> Self {
        let a: gl= (*self).into();
        a.abs().into()    
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(&self) -> Self {
        let a: gl= (*self).into();
        a.signum().into()    
    }

    /// Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[inline]
    pub fn copysign(&self, rhs: Self) -> Self {
        let a: gl= (*self).into();
        let b: gl= rhs.into();
        a.copysign(b).into()    
    }

    /// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn is_negative_bitmask(&self) -> u32 {
        let a: gl= (*self).into();
        a.is_negative_bitmask()
    }


    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        let a: gl= (*self).into();
        a.is_finite()  
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        let a: gl= (*self).into();
        a.is_nan()
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline]
    pub fn is_nan_mask(&self) -> BVec3 {
        let a: gl= (*self).into();
        a.is_nan_mask().into()    
    }


    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    pub fn length(&self) -> f32 {
        let a: gl= (*self).into();
        a.length()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    pub fn length_squared(&self) -> f32 {
        let a: gl= (*self).into();
        a.length_squared()
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(&self) -> f32 {
        let a: gl= (*self).into();
        a.length_recip()
    }

     /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(&self, rhs: Self) -> f32 {
        let a: gl= (*self).into();
        let b: gl= rhs.into();
        a.distance(b)
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(&self, rhs: Self) -> f32 {
        let a: gl= (*self).into();
        let b: gl= rhs.into();
        a.distance_squared(b)
    }

    /// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
    #[inline]
    pub fn div_euclid(&self, rhs: Self) -> Self {
        let a: gl= (*self).into();
        let b: gl= rhs.into();
        a.div_euclid(b).into()
    }

    /// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
    ///
    /// [Euclidean division]: f32::rem_euclid
    #[inline]
    pub fn rem_euclid(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.rem_euclid(b).into()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[inline]
    pub fn normalize(&self) -> Self {
        let a: gl = (*self).into();
        a.normalize().into()
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero()`].
    #[inline]
    pub fn try_normalize(&self) -> Option<Self> {
        let a: gl = (*self).into();
        let result = a.try_normalize();
        result.map(|v| v.into())
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns a
    /// fallback value.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be the fallback value.
    ///
    /// See also [`Self::try_normalize()`].
    #[inline]
    pub fn normalize_or(&self, fallback: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = fallback.into();
        a.normalize_or(b).into()
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize()`].
    #[inline]
    pub fn normalize_or_zero(&self) -> Self {
        let a: gl = (*self).into();
        a.normalize_or_zero().into()
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of approximately `1e-4`.
    #[inline]
    pub fn is_normalized(&self) -> bool {
        let a: gl = (*self).into();
        a.is_normalized()
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[inline]
    pub fn project_onto(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.project_onto(b).into()
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[inline]
    pub fn reject_from(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.reject_from(b).into()
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn project_onto_normalized(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.project_onto_normalized(b).into()
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn reject_from_normalized(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.reject_from_normalized(b).into()
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(&self) -> Self {
        let a: gl = (*self).into();
        a.round().into()
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(&self) -> Self {
        let a: gl = (*self).into();
        a.floor().into()
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    pub fn ceil(&self) -> Self {
        let a: gl = (*self).into();
        a.ceil().into()
    }

    /// Returns a vector containing the integer part each element of `self`. This means numbers are
    /// always truncated towards zero.
    #[inline]
    pub fn trunc(&self) -> Self {
        let a: gl = (*self).into();
        a.trunc().into()
    }

    /// Returns a vector containing the fractional part of the vector as `self - self.trunc()`.
    ///
    /// Note that this differs from the GLSL implementation of `fract` which returns
    /// `self - self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    pub fn fract(&self) -> Self {
        let a: gl = (*self).into();
        a.fract().into()
    }

    /// Returns a vector containing the fractional part of the vector as `self - self.floor()`.
    ///
    /// Note that this differs from the Rust implementation of `fract` which returns
    /// `self - self.trunc()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    pub fn fract_gl(&self) -> Self {
        let a: gl = (*self).into();
        a.fract_gl().into()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline]
    pub fn exp(&self) -> Self {
        let a: gl = (*self).into();
        a.exp().into()
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    pub fn powf(&self, n: f32) -> Self {
        let a: gl = (*self).into();
        a.powf(n).into()
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    pub fn recip(&self) -> Self {
        let a: gl = (*self).into();
        a.recip().into()
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    pub fn lerp(&self, rhs: Self, s: f32) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.lerp(b, s).into()
    }

    /// Moves towards `rhs` based on the value `d`.
    ///
    /// When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to
    /// `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.
    #[inline]
    pub fn move_towards(&self, rhs: Self, d: f32) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.move_towards(b, d).into()
    }

    /// Calculates the midpoint between `self` and `rhs`.
    ///
    /// The midpoint is the average of, or halfway point between, two vectors.
    /// `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)`
    /// while being slightly cheaper to compute.
    #[inline]
    pub fn midpoint(&self, rhs: Self) -> Self {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.midpoint(b).into()
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.abs_diff_eq(b, max_abs_diff)
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp_length(&self, min: f32, max: f32) -> Self {
        let a: gl = (*self).into();
        a.clamp_length(min, max).into()
    }

    /// Returns a vector with a length no more than `max`
    #[inline]
    pub fn clamp_length_max(&self, max: f32) -> Self {
        let a: gl = (*self).into();
        a.clamp_length_max(max).into()
    }

    /// Returns a vector with a length no less than `min`
    #[inline]
    pub fn clamp_length_min(&self, min: f32) -> Self {
        let a: gl = (*self).into();
        a.clamp_length_min(min).into()
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated fma CPU instruction. However, this is not always true,
    /// and will be heavily dependant on designing algorithms with specific target hardware in
    /// mind.
    #[inline]
    pub fn mul_add(&self, a: Self, b: Self) -> Self {
        let a_gl: gl = (*self).into();
        let b_gl: gl = a.into();
        let c_gl: gl = b.into();
        a_gl.mul_add(b_gl, c_gl).into()
    }

    /// Returns the angle (in radians) between two vectors.
    ///
    /// The inputs do not need to be unit vectors however they must be non-zero.
    #[inline]
    pub fn angle_between(&self, rhs: Self) -> f32 {
        let a: gl = (*self).into();
        let b: gl = rhs.into();
        a.angle_between(b)
    }

    /// Returns some vector that is orthogonal to the given one.
    ///
    /// The input vector must be finite and non-zero.
    ///
    /// The output vector is not necessarily unit length. For that use
    /// [`Self::any_orthonormal_vector()`] instead.
    #[inline]
    pub fn any_orthogonal_vector(&self) -> Self {
        let a: gl = (*self).into();
        a.any_orthogonal_vector().into()
    }

    /// Returns any unit vector that is orthogonal to the given one.
    ///
    /// The input vector must be unit length.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_vector(&self) -> Self {
        let a: gl = (*self).into();
        a.any_orthonormal_vector().into()
    }

    /// Given a unit vector return two other vectors that together form an orthonormal
    /// basis. That is, all three vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_pair(&self) -> (Vec3, Vec3) {
        let a: gl = (*self).into();
        let (b, c) = a.any_orthonormal_pair();
        (b.into(), c.into())
    }

    /// The default `Vec3` is `[0.0, 0.0, 0.0]`.
    #[pyo3(name = "zero")]
    #[staticmethod]
    #[inline]
    pub fn default() -> Self {
        let a: gl = gl::default();
        a.into()
    }


    #[pyo3(name = "__truediv__")]
    #[inline]
    pub fn truediv(&self, rhs: Vec3OrF32) -> PyResult<Self> {
        let lhs: gl = (*self).into();
        match rhs {
            Vec3OrF32::F32(rhs_into) => {
                Ok((lhs / rhs_into).into())
            }
            Vec3OrF32::Vec3(rhs_into) => {
                let b: gl = rhs_into.into();
                Ok((lhs / b).into())
            }
        }
    }
    



    //#[pyo3(name = "__itruediv__")]
    //#[inline]
    //pub fn __itruediv__(&mut self, rhs: &PyAny) -> PyResult<Self> {
    //    let mut lhs: gl = (*self).into();
    //    if let Ok(rhs_self) = rhs.extract::<Self>() {
    //        let b: gl = rhs_self.into();
    //        lhs /= b;
    //        *self = lhs.into();
    //        Ok(self.clone())
    //    } else if let Ok(val_f32) = rhs.extract::<f32>() {
    //        lhs /= val_f32;
    //        *self = lhs.into();
    //        Ok(self.clone())
    //    } else {
    //        Err(PyNotImplementedError::new_err("Unsupported operand type for /=",))
    //    }
    //}


    #[pyo3(name = "__mul__")]
    #[inline]
    pub fn mul(&self, rhs: Vec3OrF32) -> PyResult<Self> {
        let lhs: gl = (*self).into();
        match rhs {
            Vec3OrF32::F32(rhs_into) =>{
                Ok((lhs * rhs_into).into())
            }
            Vec3OrF32::Vec3(rhs_into) =>{
                let b: gl = rhs_into.into();
                Ok((lhs*b).into())
            }
        }
        
    }

    //#[pyo3(name = "__imul__")]
    //#[inline]
    //pub fn __imul__ (&mut self, rhs: &PyAny) -> PyResult<Self> {
    //    let mut lhs: gl = (*self).into();

    //    if let Ok(rhs_self) = rhs.extract::<Self>() {
    //        let b: gl = rhs_self.into();
    //        lhs*=b;
    //        Ok(lhs.into())
    //    } else if let Ok(val_f32) = rhs.extract::<f32>() {
    //        lhs*=val_f32;
    //        Ok(lhs.into())
    //    } else {
    //        Err(PyNotImplementedError::new_err("Unsupported operand type for *=",))
    //    }
    //}

    #[pyo3(name = "__rmul__")]
    #[inline]
    pub fn rmul(&self, lhs: f32) -> Self {
        let a: gl = (*self).into();
        (lhs * a).into()
    }

    #[pyo3(name = "__add__")]
    #[inline]
    pub fn add(&self, rhs: Vec3OrF32) -> PyResult<Self> {
        let lhs: gl = (*self).into();
        match rhs {
            Vec3OrF32::F32(rhs_into) => {
                Ok((lhs + rhs_into).into())
            }
            Vec3OrF32::Vec3(rhs_into) => {
                let b: gl = rhs_into.into();
                Ok((lhs + b).into())
            }
        }
    }

    // #[pyo3(name = "__iadd__")]
    // #[inline]
    // pub fn __iadd__ (&self, rhs: &Vec3OrF32) -> PyResult<Self> {
    //    let mut lhs: gl = (*self).into();

    //    match rhs {
    //     Vec3OrF32::Vec3(rhs) => {
    //         let rhs: gl  = *rhs.into();
    //         (lhs + rhs.into()).into()
    //     }
    //     Vec3OrF32::F32(rhs) => {
    //         (lhs + rhs).into()
    //     }
        
    //    }
       
    // }

    #[pyo3(name = "__radd__")]
    #[inline]
    pub fn add_f32_vec3(&self, lhs: f32) -> Self {
        let a: gl = (*self).into();
        (lhs + a).into()
    }


    #[pyo3(name = "__sub__")]
    #[inline]
    pub fn sub(&self, rhs: Vec3OrF32) -> PyResult<Self> {
        let lhs: gl = (*self).into();
        match rhs {
            Vec3OrF32::F32(rhs_into) => {
                Ok((lhs - rhs_into).into())
            }
            Vec3OrF32::Vec3(rhs_into) => {
                let b: gl = rhs_into.into();
                Ok((lhs - b).into())
            }
        }
    }

    //#[pyo3(name = "__isub__")]
    //#[inline]
    //pub fn __isub__ (&mut self, rhs: &PyAny) -> PyResult<Self> {
    //    let mut lhs: gl = (*self).into();

    //    if let Ok(rhs_self) = rhs.extract::<Self>() {
    //        let b: gl = rhs_self.into();
    //        lhs-=b;
    //        Ok(lhs.into())
    //    } else if let Ok(val_f32) = rhs.extract::<f32>() {
    //        lhs-=val_f32;
    //        Ok(lhs.into())
    //    } else {
    //        Err(PyNotImplementedError::new_err("Unsupported operand type for -=",))
    //    }
    //}

    #[pyo3(name = "__rsub__")]
    #[inline]
    pub fn sub_f32_vec3(&self, lhs: f32) -> Self {
        let a: gl = (*self).into();
        (lhs - a).into()
    }
    


    #[pyo3(name = "__mod__")]
    #[inline]
    pub fn __mod__(&self, rhs: Vec3OrF32) -> PyResult<Self> {
        let lhs: gl = (*self).into();
        match rhs {
            Vec3OrF32::F32(rhs_into) => {
                Ok((lhs % rhs_into).into())
            }
            Vec3OrF32::Vec3(rhs_into) => {
                let b: gl = rhs_into.into(); 
                Ok((lhs % b).into())
            }
        }
    }


    //#[pyo3(name = "__imod__")]
    //#[inline]
    //pub fn __imod__ (&mut self, rhs: &PyAny) -> PyResult<Self> {
    //    let mut lhs: gl = (*self).into();

    //    if let Ok(rhs_self) = rhs.extract::<Self>() {
    //        let b: gl = rhs_self.into();
    //        lhs%=b;
    //        Ok(lhs.into())
    //    } else if let Ok(val_f32) = rhs.extract::<f32>() {
    //        lhs%=val_f32;
    //        Ok(lhs.into())
    //    } else {
    //        Err(PyNotImplementedError::new_err("Unsupported operand type for %=",))
    //    }
    //}

    #[pyo3(name = "__rmod__")]
    #[inline]
    pub fn rem_f32_vec3(&self, lhs: f32) -> Self {
        let a: gl = (*self).into();
        (lhs % a).into()
    }

}



impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}



impl From<Vec3> for gl {
    fn from(v: Vec3) -> Self {
        gl::new(v.x, v.y, v.z)
    }
}


impl From<gl> for Vec3 {
    fn from(v: gl) -> Self {
        Vec3 { x: v.x, y: v.y, z: v.z, }
    }
}

#[derive(FromPyObject)]
pub enum Vec3OrF32 {
    Vec3(Vec3),
    F32(f32),
}


use pyo3_stub_gen::{PyStubType, TypeInfo};

impl PyStubType for Vec3OrF32 {
    fn type_input() -> TypeInfo {
        TypeInfo::builtin("Union['Vec3', float]")
    }
    fn type_output() -> TypeInfo {
        TypeInfo::builtin("Union['Vec3', float]")
    }
}