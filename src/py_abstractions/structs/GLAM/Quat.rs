use glam::Quat as gl;
use pyo3::prelude::*;

use crate::py_abstractions::structs::GLAM::Mat4::Mat4;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec4::Vec4;
use crate::py_abstractions::UniformType::EulerRot;


//
// A python abstraction for the Quat struct from the GLAM crate.
// This file implements all functionality from Glam, replacing uses of Vec2, Vec3, Vec4 and Mat4 with the pyabstracted versions.
//
/// An immutable quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
#[cfg_attr(feature = "abi_314", pyclass(eq,str,frozen, immutable_type, from_py_object))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,frozen, from_py_object))]
#[repr(C)]
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
    pub const fn ZERO() -> Quat {
        Self::from_array([0.0; 4])
    }

    /// The identity quaternion. Corresponds to no rotation.
    #[classattr]
    pub const fn IDENTITY() -> Quat {
        Self::from_xyzw(0.0, 0.0, 0.0, 1.0)
    }

    /// All NaNs.
    #[classattr]
    pub const fn NAN() -> Quat {
        Self::from_array([f32::NAN; 4])
    }

    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing.
    /// Use one of the other constructors instead such as `IDENTITY` or `from_axis_angle`.
    ///
    /// `from_xyzw` is mostly used by unit tests and deserialization.
    ///
    /// Note:
    ///     This function does not check if the input is normalized, it is up to the user to
    ///     provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    #[staticmethod]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a rotation quaternion from a list.
    ///
    /// Note:
    ///     This function does not check if the input is normalized, it is up to the user to
    ///     provide normalized input or to normalized the resulting quaternion.
    #[inline]
    #[must_use]
    #[staticmethod]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }

    /// Creates a new rotation quaternion from a 4D vector.
    ///
    /// Note:
    ///     This function does not check if the input is normalized, it is up to the user to
    ///     provide normalized input or to normalized the resulting quaternion.
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

    /// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    ///
    /// The axis must be a unit vector.
    ///
    /// Raises:
    ///     PanicException: If `axis` is not normalized (only when built with the `glam_assert` feature).
    #[inline]
    #[staticmethod]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        gl::from_axis_angle(axis.into(), angle).into()
    }

    /// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    /// `from_scaled_axis(Vec3.ZERO)` results in the identity quaternion.
    #[inline]
    #[staticmethod]
    pub fn from_scaled_axis(v: Vec3) -> Self {
        gl::from_scaled_axis(v.into()).into()
    }

    /// Creates a quaternion from the `angle` (in radians) around the x axis.
    #[inline]
    #[staticmethod]
    pub fn from_rotation_x(angle: f32) -> Self {
        gl::from_rotation_x(angle).into()
    }

    /// Creates a quaternion from the `angle` (in radians) around the y axis.
    #[inline]
    #[staticmethod]
    pub fn from_rotation_y(angle: f32) -> Self {
        gl::from_rotation_y(angle).into()
    }

    /// Creates a quaternion from the `angle` (in radians) around the z axis.
    #[inline]
    #[staticmethod]
    pub fn from_rotation_z(angle: f32) -> Self {
        gl::from_rotation_z(angle).into()
    }

    /// Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
    #[inline]
    #[staticmethod]
    pub fn from_euler(euler: EulerRot, a: f32, b: f32, c: f32) -> Self {
        gl::from_euler(euler.into(), a, b, c).into()
    }

    /// Creates a quaternion from the upper 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    ///
    /// Raises:
    ///     PanicException: If any column of the upper 3x3 rotation matrix is not normalized
    ///     (only when built with the `glam_assert` feature).
    #[inline]
    #[staticmethod]
    pub fn from_mat4(mat: Mat4) -> Self {
        gl::from_mat4(&mat.into()).into()
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    /// plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `from_rotation_arc(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// Raises:
    ///     PanicException: If `from` or `to` are not normalized (only when built with the `glam_assert` feature).
    #[inline]
    #[staticmethod]
    pub fn from_rotation_arc(from: Vec3, to: Vec3) -> Self {
        gl::from_rotation_arc(from.into(), to.into()).into()
    }

    /// Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
    /// that the resulting quaternion will rotate `from` so that it is colinear with `to`.
    ///
    /// The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
    /// degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    /// Raises:
    ///     PanicException: If `from` or `to` are not normalized (only when built with the `glam_assert` feature).
    #[inline]
    #[staticmethod]
    pub fn from_rotation_arc_colinear(from: Vec3, to: Vec3) -> Self {
        gl::from_rotation_arc_colinear(from.into(), to.into()).into()
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    /// around the z axis. Will rotate at most 180 degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// Raises:
    ///     PanicException: If `from` or `to` are not normalized (only when built with the `glam_assert` feature).
    #[inline]
    #[staticmethod]
    pub fn from_rotation_arc_2d(from: Vec2, to: Vec2) -> Self {
        gl::from_rotation_arc_2d(from.into(), to.into()).into()
    }

    /// Returns the rotation axis (normalized) and angle (in radians) of `self`.
    #[inline]
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let (axis, angle) = gl::from(*self).to_axis_angle();
        (axis.into(), angle)
    }

    /// Returns the rotation axis scaled by the rotation in radians.
    #[inline]
    pub fn to_scaled_axis(&self) -> Vec3 {
        gl::from(*self).to_scaled_axis().into()
    }

    /// Returns the rotation angles for the given euler rotation sequence.
    #[inline]
    pub fn to_euler(&self, euler: EulerRot) -> (f32, f32, f32) {
        gl::from(*self).to_euler(euler.into())
    }

    /// `[x, y, z, w]`
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part of the quaternion.
    #[inline]
    pub const fn xyz(&self) -> Vec3 {
        Vec3::const_new(self.x, self.y, self.z)
    }

    /// Returns the quaternion conjugate of `self`. For a unit quaternion the
    /// conjugate is also the inverse.
    #[inline]
    pub fn conjugate(&self) -> Self {
        gl::from(*self).conjugate().into()
    }

    /// Returns the inverse of a normalized quaternion.
    ///
    /// Typically quaternion inverse returns the conjugate of a normalized quaternion.
    /// Because `self` is assumed to already be unit length this method *does not* normalize
    /// before returning the conjugate.
    ///
    /// Raises:
    ///     PanicException: If `self` is not normalized (only when built with the `glam_assert` feature).
    #[inline]
    pub fn inverse(&self) -> Self {
        gl::from(*self).inverse().into()
    }

    /// Computes the dot product of `self` and `rhs`. The dot product is
    /// equal to the cosine of the angle between two quaternion rotations.
    #[inline]
    pub fn dot(&self, rhs: Self) -> f32 {
        gl::from(*self).dot(rhs.into())
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(&self) -> f32 {
        gl::from(*self).length()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        gl::from(*self).length_squared()
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(&self) -> f32 {
        gl::from(*self).length_recip()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    ///
    /// Raises:
    ///     PanicException: If `self` is zero length (only when built with the `glam_assert` feature).
    #[inline]
    pub fn normalize(&self) -> Self {
        gl::from(*self).normalize().into()
    }

    /// Returns `True` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `False`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        gl::from(*self).is_finite()
    }

    /// Returns `True` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        gl::from(*self).is_nan()
    }

    /// Returns whether `self` is of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(&self) -> bool {
        gl::from(*self).is_normalized()
    }

    /// Returns `True` if the rotation is (approximately) the identity rotation.
    #[inline]
    pub fn is_near_identity(&self) -> bool {
        gl::from(*self).is_near_identity()
    }

    /// Returns the angle (in radians) for the minimal rotation
    /// for transforming this quaternion into another.
    ///
    /// Both quaternions must be normalized.
    ///
    /// Raises:
    ///     PanicException: If `self` or `rhs` are not normalized (only when built with the `glam_assert` feature).
    #[inline]
    pub fn angle_between(&self, rhs: Self) -> f32 {
        gl::from(*self).angle_between(rhs.into())
    }

    /// Returns `True` if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two quaternions contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    #[inline]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        gl::from(*self).abs_diff_eq(rhs.into(), max_abs_diff)
    }

    /// Performs a linear interpolation between `self` and `end` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `end`.
    ///
    /// Raises:
    ///     PanicException: If `self` or `end` are not normalized (only when built with the `glam_assert` feature).
    #[inline]
    pub fn lerp(&self, end: Self, s: f32) -> Self {
        gl::from(*self).lerp(end.into(), s).into()
    }

    /// Performs a spherical linear interpolation between `self` and `end`
    /// based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `end`.
    ///
    /// Raises:
    ///     PanicException: If `self` or `end` are not normalized (only when built with the `glam_assert` feature).
    #[inline]
    pub fn slerp(&self, end: Self, s: f32) -> Self {
        gl::from(*self).slerp(end.into(), s).into()
    }

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    /// Same as `self * rhs`.
    ///
    /// Raises:
    ///     PanicException: If `self` is not normalized (only when built with the `glam_assert` feature).
    #[inline]
    pub fn mul_vec3(&self, rhs: Vec3) -> Vec3 {
        gl::from(*self).mul_vec3(rhs.into()).into()
    }

    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation. Same as `self * rhs`.
    ///
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    ///
    /// Raises:
    ///     PanicException: If `self` or `rhs` are not normalized (only when built with the `glam_assert` feature).
    #[inline]
    pub fn mul_quat(&self, rhs: Self) -> Self {
        gl::from(*self).mul_quat(rhs.into()).into()
    }

    /// `Quat * Quat` combines the rotations, `Quat * float` scales every element.
    /// `Quat * Vec3` rotates the vector.
    #[pyo3(name = "__mul__")]
    #[inline]
    pub fn mul(&self, rhs: QuatOrF32) -> Self {
        let lhs: gl = (*self).into();
        match rhs {
            QuatOrF32::Quat(rhs) => (lhs * gl::from(rhs)).into(),
            QuatOrF32::F32(rhs) => (lhs * rhs).into(),
        }
    }

    /// Divides every element by `rhs`.
    #[pyo3(name = "__truediv__")]
    #[inline]
    pub fn truediv(&self, rhs: f32) -> Self {
        (gl::from(*self) / rhs).into()
    }

    /// Adds two quaternions element-wise.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication.
    #[pyo3(name = "__add__")]
    #[inline]
    pub fn add(&self, rhs: Self) -> Self {
        (gl::from(*self) + gl::from(rhs)).into()
    }

    /// Subtracts the `rhs` quaternion from `self` element-wise.
    ///
    /// The difference is not guaranteed to be normalized.
    #[pyo3(name = "__sub__")]
    #[inline]
    pub fn sub(&self, rhs: Self) -> Self {
        (gl::from(*self) - gl::from(rhs)).into()
    }

    #[pyo3(name = "__neg__")]
    #[inline]
    pub fn neg(&self) -> Self {
        (-gl::from(*self)).into()
    }
}



impl From<gl> for Quat{
    fn from(v: gl) -> Self {
        Quat::from_xyzw(v.x, v.y, v.z, v.w)
    }
}


impl From<Quat> for gl{
    fn from(v: Quat) -> gl {
        gl::from_xyzw(v.x, v.y, v.z, v.w)
    }
}



impl std::fmt::Display for Quat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quat({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[derive(FromPyObject)]
pub enum QuatOrF32 {
    Quat(Quat),
    F32(f32),
}
