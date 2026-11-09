use glam::Mat4 as gl;
use pyo3::prelude::*;

use crate::py_abstractions::structs::GLAM::Quat::Quat;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec4::Vec4;
use crate::py_abstractions::UniformType::EulerRot;
use crate::py_assert;


//
// A python abstraction for the Mat4 struct from the GLAM crate.
// This file implements all functionality from Glam, replacing uses of Vec3, Vec4 and Quat with the pyabstracted versions.
//
/// An immutable 4x4 column major matrix.
///
/// This 4x4 matrix type features convenience methods for creating and using affine transforms and
/// perspective projections.
///
/// Affine transformations including 3D translation, rotation and scale can be created
/// using methods such as `from_translation()`, `from_quat()`, `from_scale()` and
/// `from_scale_rotation_translation()`.
///
/// Orthographic projections can be created using the methods `orthographic_lh()` for
/// left-handed coordinate systems and `orthographic_rh()` for right-handed
/// systems. The resulting matrix is also an affine transformation.
///
/// The `transform_point3()` and `transform_vector3()` convenience methods are provided for
/// performing affine transformations on 3D vectors and points. These multiply 3D inputs as 4D
/// vectors with an implicit `w` value of `1` for points and `0` for vectors respectively.
///
/// Perspective projections can be created using methods such as `perspective_lh()`,
/// `perspective_infinite_lh()` and `perspective_infinite_reverse_lh()` for left-handed
/// systems and `perspective_rh()`, `perspective_infinite_rh()` and
/// `perspective_infinite_reverse_rh()` for right-handed systems.
///
/// The resulting perspective project can be use to transform 3D vectors as points with
/// perspective correction using the `project_point3()` convenience method.
#[cfg_attr(feature = "abi_314", pyclass(eq,str,frozen, immutable_type, from_py_object))]
#[cfg_attr(not(feature = "abi_314"), pyclass(eq,str,frozen, from_py_object))]
#[repr(C)]
#[derive(Clone,Copy,PartialEq,Debug)]
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

#[pymethods]
impl Mat4 {

    /// Creates a 4x4 matrix from four column vectors.
    #[new]
    pub const fn new(x_axis: Vec4,y_axis: Vec4,z_axis: Vec4,w_axis: Vec4,) -> Self {
        Self::const_new(x_axis, y_axis, z_axis, w_axis)
    }

    /// A 4x4 matrix with all elements set to `0.0`.
    #[classattr]
    pub const fn ZERO() -> Mat4 {
        Self::const_splat(Vec4::const_splat(0.0))
    }

    /// A 4x4 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[classattr]
    pub const fn IDENTITY() -> Mat4 {
        Self::const_new(
            Vec4::const_new(1.0, 0.0, 0.0, 0.0),
            Vec4::const_new(0.0, 1.0, 0.0, 0.0),
            Vec4::const_new(0.0, 0.0, 1.0, 0.0),
            Vec4::const_new(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// All NaNs.
    #[classattr]
    pub const fn NAN() -> Mat4 {
        Self::const_splat(Vec4::const_splat(f32::NAN))
    }

    /// Creates a 4x4 matrix from four column vectors.
    #[staticmethod]
    #[inline]
    pub const fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self::const_new(x_axis, y_axis, z_axis, w_axis)
    }

    /// Creates a 4x4 matrix from a list of 16 floats stored in column major order.
    #[staticmethod]
    #[inline]
    pub const fn from_cols_array(m: [f32; 16]) -> Self {
        Self::const_new(
            Vec4::const_new(m[0], m[1], m[2], m[3]),
            Vec4::const_new(m[4], m[5], m[6], m[7]),
            Vec4::const_new(m[8], m[9], m[10], m[11]),
            Vec4::const_new(m[12], m[13], m[14], m[15]),
        )
    }

    /// Creates a list of 16 floats storing data in column major order.
    #[inline]
    pub const fn to_cols_array(&self) -> [f32; 16] {
        let [x, y, z, w] = [self.x_axis, self.y_axis, self.z_axis, self.w_axis];
        [
            x.x, x.y, x.z, x.w,
            y.x, y.y, y.z, y.w,
            z.x, z.y, z.z, z.w,
            w.x, w.y, w.z, w.w,
        ]
    }

    /// Creates a 4x4 matrix from a list of 4 lists of 4 floats, where each inner list is a column.
    #[staticmethod]
    #[inline]
    pub const fn from_cols_array_2d(m: [[f32; 4]; 4]) -> Self {
        Self::const_new(
            Vec4::from_array(m[0]),
            Vec4::from_array(m[1]),
            Vec4::from_array(m[2]),
            Vec4::from_array(m[3]),
        )
    }

    /// Creates a list of 4 lists of 4 floats, where each inner list is a column.
    #[inline]
    pub const fn to_cols_array_2d(&self) -> [[f32; 4]; 4] {
        [
            self.x_axis.to_array(),
            self.y_axis.to_array(),
            self.z_axis.to_array(),
            self.w_axis.to_array(),
        ]
    }

    /// Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[staticmethod]
    #[inline]
    pub const fn from_diagonal(diagonal: Vec4) -> Self {
        let d = diagonal;
        Self::const_new(
            Vec4::const_new(d.x, 0.0, 0.0, 0.0),
            Vec4::const_new(0.0, d.y, 0.0, 0.0),
            Vec4::const_new(0.0, 0.0, d.z, 0.0),
            Vec4::const_new(0.0, 0.0, 0.0, d.w),
        )
    }

    /// Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
    /// `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    ///
    /// Raises:
    ///     PanicException: If `rotation` is not normalized (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        gl::from_scale_rotation_translation(scale.into(), rotation.into(), translation.into()).into()
    }

    /// Creates an affine transformation matrix from the given 3D `rotation` and `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    ///
    /// Raises:
    ///     PanicException: If `rotation` is not normalized (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        gl::from_rotation_translation(rotation.into(), translation.into()).into()
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is
    /// expected to be a 3D affine transformation matrix otherwise the output will be invalid.
    ///
    /// Raises:
    ///     PanicException: If the determinant of `self` is zero or if the resulting scale vector
    ///     contains any zero elements (only when built with the `glam_assert` feature).
    #[inline]
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        let (scale, rotation, translation) = gl::from(*self).to_scale_rotation_translation();
        (scale.into(), rotation.into(), translation.into())
    }

    /// Creates an affine transformation matrix from the given `rotation` quaternion.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    ///
    /// Raises:
    ///     PanicException: If `rotation` is not normalized (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn from_quat(rotation: Quat) -> Self {
        gl::from_quat(rotation.into()).into()
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    #[staticmethod]
    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        gl::from_translation(translation.into()).into()
    }

    /// Creates an affine transformation matrix containing a 3D rotation around a normalized
    /// rotation `axis` of `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    ///
    /// Raises:
    ///     PanicException: If `axis` is not normalized (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        gl::from_axis_angle(axis.into(), angle).into()
    }

    /// Creates a affine transformation matrix containing a rotation from the given euler
    /// rotation sequence and angles (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    #[staticmethod]
    #[inline]
    pub fn from_euler(order: EulerRot, a: f32, b: f32, c: f32) -> Self {
        gl::from_euler(order.into(), a, b, c).into()
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    #[staticmethod]
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        gl::from_rotation_x(angle).into()
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    #[staticmethod]
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        gl::from_rotation_y(angle).into()
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    #[staticmethod]
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        gl::from_rotation_z(angle).into()
    }

    /// Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// `transform_point3()` and `transform_vector3()`.
    ///
    /// Raises:
    ///     PanicException: If all elements of `scale` are zero (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        gl::from_scale(scale.into()).into()
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 3.
    #[inline]
    pub fn col(&self, index: usize) -> PyResult<Vec4> {
        py_assert!(index < 4, "index out of bounds.");
        Ok(gl::from(*self).col(index).into())
    }

    /// Returns the matrix row for the given `index`.
    ///
    /// Raises:
    ///     BaseException: If `index` is greater than 3.
    #[inline]
    pub fn row(&self, index: usize) -> PyResult<Vec4> {
        py_assert!(index < 4, "index out of bounds.");
        Ok(gl::from(*self).row(index).into())
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

    /// Returns the transpose of `self`.
    #[inline]
    pub fn transpose(&self) -> Self {
        gl::from(*self).transpose().into()
    }

    /// Returns the determinant of `self`.
    #[inline]
    pub fn determinant(&self) -> f32 {
        gl::from(*self).determinant()
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    ///
    /// Raises:
    ///     PanicException: If the determinant of `self` is zero (only when built with the `glam_assert` feature).
    #[inline]
    pub fn inverse(&self) -> Self {
        gl::from(*self).inverse().into()
    }

    /// Creates a left-handed view matrix using a camera position, an up direction, and a facing
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[staticmethod]
    #[inline]
    pub fn look_to_lh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        gl::look_to_lh(eye.into(), dir.into(), up.into()).into()
    }

    /// Creates a right-handed view matrix using a camera position, an up direction, and a facing
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[staticmethod]
    #[inline]
    pub fn look_to_rh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        gl::look_to_rh(eye.into(), dir.into(), up.into()).into()
    }

    /// Creates a left-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// Raises:
    ///     PanicException: If `up` is not normalized (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        gl::look_at_lh(eye.into(), center.into(), up.into()).into()
    }

    /// Creates a right-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// Raises:
    ///     PanicException: If `up` is not normalized (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        gl::look_at_rh(eye.into(), center.into(), up.into()).into()
    }

    /// Creates a right-handed perspective projection matrix with `[-1,1]` depth range.
    ///
    /// Useful to map the standard right-handed coordinate system into what OpenGL expects.
    ///
    /// This is the same as the OpenGL `gluPerspective` function.
    #[staticmethod]
    #[inline]
    pub fn perspective_rh_gl(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        gl::perspective_rh_gl(fov_y_radians, aspect_ratio, z_near, z_far).into()
    }

    /// Creates a left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// Useful to map the standard left-handed coordinate system into what WebGPU/Metal/Direct3D expect.
    ///
    /// Raises:
    ///     PanicException: If `z_near` or `z_far` are less than or equal to zero
    ///     (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn perspective_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        gl::perspective_lh(fov_y_radians, aspect_ratio, z_near, z_far).into()
    }

    /// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// Useful to map the standard right-handed coordinate system into what WebGPU/Metal/Direct3D expect.
    ///
    /// Raises:
    ///     PanicException: If `z_near` or `z_far` are less than or equal to zero
    ///     (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn perspective_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        gl::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far).into()
    }

    /// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// Like `perspective_lh`, but with an infinite value for `z_far`.
    /// The result is that points near `z_near` are mapped to depth `0`, and as they move towards infinity the depth approaches `1`.
    ///
    /// Raises:
    ///     PanicException: If `z_near` is less than or equal to zero
    ///     (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn perspective_infinite_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        gl::perspective_infinite_lh(fov_y_radians, aspect_ratio, z_near).into()
    }

    /// Creates an infinite reverse left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// Similar to `perspective_infinite_lh`, but maps `Z = z_near` to a depth of `1` and `Z = infinity` to a depth of `0`.
    ///
    /// Raises:
    ///     PanicException: If `z_near` is less than or equal to zero
    ///     (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn perspective_infinite_reverse_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        gl::perspective_infinite_reverse_lh(fov_y_radians, aspect_ratio, z_near).into()
    }

    /// Creates an infinite right-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// Like `perspective_rh`, but with an infinite value for `z_far`.
    /// The result is that points near `z_near` are mapped to depth `0`, and as they move towards infinity the depth approaches `1`.
    ///
    /// Raises:
    ///     PanicException: If `z_near` is less than or equal to zero
    ///     (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn perspective_infinite_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        gl::perspective_infinite_rh(fov_y_radians, aspect_ratio, z_near).into()
    }

    /// Creates an infinite reverse right-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// Similar to `perspective_infinite_rh`, but maps `Z = z_near` to a depth of `1` and `Z = infinity` to a depth of `0`.
    ///
    /// Raises:
    ///     PanicException: If `z_near` is less than or equal to zero
    ///     (only when built with the `glam_assert` feature).
    #[staticmethod]
    #[inline]
    pub fn perspective_infinite_reverse_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        gl::perspective_infinite_reverse_rh(fov_y_radians, aspect_ratio, z_near).into()
    }

    /// Creates a right-handed orthographic projection matrix with `[-1,1]` depth
    /// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
    ///
    /// Useful to map a right-handed coordinate system to the normalized device coordinates that OpenGL expects.
    #[staticmethod]
    #[inline]
    pub fn orthographic_rh_gl(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        gl::orthographic_rh_gl(left, right, bottom, top, near, far).into()
    }

    /// Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
    ///
    /// Useful to map a left-handed coordinate system to the normalized device coordinates that WebGPU/Direct3D/Metal expect.
    #[staticmethod]
    #[inline]
    pub fn orthographic_lh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        gl::orthographic_lh(left, right, bottom, top, near, far).into()
    }

    /// Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
    ///
    /// Useful to map a right-handed coordinate system to the normalized device coordinates that WebGPU/Direct3D/Metal expect.
    #[staticmethod]
    #[inline]
    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        gl::orthographic_rh(left, right, bottom, top, near, far).into()
    }

    /// Transforms the given 3D vector as a point, applying perspective correction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
    /// The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
    ///
    /// This method assumes that `self` contains a projective transform.
    #[inline]
    pub fn project_point3(&self, rhs: Vec3) -> Vec3 {
        gl::from(*self).project_point3(rhs.into()).into()
    }

    /// Transforms the given 3D vector as a point.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    /// `1.0`.
    ///
    /// This method assumes that `self` contains a valid affine transform. It does not perform
    /// a perspective divide, if `self` contains a perspective transform, or if you are unsure,
    /// the `project_point3()` method should be used instead.
    ///
    /// Raises:
    ///     PanicException: If `self` is not an affine transform (only when built with the `glam_assert` feature).
    #[inline]
    pub fn transform_point3(&self, rhs: Vec3) -> Vec3 {
        gl::from(*self).transform_point3(rhs.into()).into()
    }

    /// Transforms the give 3D vector as a direction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    /// `0.0`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    ///
    /// Raises:
    ///     PanicException: If `self` is not an affine transform (only when built with the `glam_assert` feature).
    #[inline]
    pub fn transform_vector3(&self, rhs: Vec3) -> Vec3 {
        gl::from(*self).transform_vector3(rhs.into()).into()
    }

    /// Transforms a 4D vector. Same as `self * rhs`.
    #[inline]
    pub fn mul_vec4(&self, rhs: Vec4) -> Vec4 {
        gl::from(*self).mul_vec4(rhs.into()).into()
    }

    /// Multiplies two 4x4 matrices. Same as `self * rhs`.
    #[inline]
    pub fn mul_mat4(&self, rhs: Self) -> Self {
        gl::from(*self).mul_mat4(&rhs.into()).into()
    }

    /// Adds two 4x4 matrices. Same as `self + rhs`.
    #[inline]
    pub fn add_mat4(&self, rhs: Self) -> Self {
        gl::from(*self).add_mat4(&rhs.into()).into()
    }

    /// Subtracts two 4x4 matrices. Same as `self - rhs`.
    #[inline]
    pub fn sub_mat4(&self, rhs: Self) -> Self {
        gl::from(*self).sub_mat4(&rhs.into()).into()
    }

    /// Multiplies a 4x4 matrix by a scalar. Same as `self * rhs`.
    #[inline]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        gl::from(*self).mul_scalar(rhs).into()
    }

    /// Divides a 4x4 matrix by a scalar. Same as `self / rhs`.
    #[inline]
    pub fn div_scalar(&self, rhs: f32) -> Self {
        gl::from(*self).div_scalar(rhs).into()
    }

    /// Returns `True` if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two matrices contain similar elements. It works best
    /// when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    #[inline]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        gl::from(*self).abs_diff_eq(rhs.into(), max_abs_diff)
    }

    /// Takes the absolute value of each element in `self`.
    #[inline]
    pub fn abs(&self) -> Self {
        gl::from(*self).abs().into()
    }

    /// `Mat4 * Mat4` multiplies the matrices, `Mat4 * float` scales every element.
    /// `Mat4 * Vec4` transforms the vector.
    #[pyo3(name = "__mul__")]
    #[inline]
    pub fn mul(&self, rhs: Mat4OrF32) -> Self {
        let lhs: gl = (*self).into();
        match rhs {
            Mat4OrF32::Mat4(rhs) => (lhs * gl::from(rhs)).into(),
            Mat4OrF32::F32(rhs) => (lhs * rhs).into(),
        }
    }

    #[pyo3(name = "__rmul__")]
    #[inline]
    pub fn rmul(&self, lhs: f32) -> Self {
        (lhs * gl::from(*self)).into()
    }

    #[pyo3(name = "__truediv__")]
    #[inline]
    pub fn truediv(&self, rhs: f32) -> Self {
        (gl::from(*self) / rhs).into()
    }

    #[pyo3(name = "__rtruediv__")]
    #[inline]
    pub fn rtruediv(&self, lhs: f32) -> Self {
        (lhs / gl::from(*self)).into()
    }

    #[pyo3(name = "__add__")]
    #[inline]
    pub fn add(&self, rhs: Self) -> Self {
        (gl::from(*self) + gl::from(rhs)).into()
    }

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

#[derive(FromPyObject)]
pub enum Mat4OrF32 {
    Mat4(Mat4),
    F32(f32),
}
