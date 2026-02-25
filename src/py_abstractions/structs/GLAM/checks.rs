

/// THis check is technically non-critical but is needed for fast maths
#[allow(non_snake_case)]
const _: () = {
    use std::mem::{size_of, align_of};
    use crate::py_abstractions::structs::GLAM;

    macro_rules! assert_layout {
        ($mine:ty, $glam:ty) => {
            if size_of::<$mine>() != size_of::<$glam>() {
                let _ = ["Size mismatch between Rust and Glam types"][1];
            }
            if align_of::<$mine>() != align_of::<$glam>() {
                let _ = ["Alignment mismatch between Rust and Glam types"][1];
            }
        };
    }

    assert_layout!(GLAM::Vec2::Vec2, glam::Vec2);
    assert_layout!(GLAM::Vec3::Vec3, glam::Vec3);
    assert_layout!(GLAM::Vec4::Vec4, glam::Vec4);
    assert_layout!(GLAM::BVec2::BVec2, glam::BVec2);
    assert_layout!(GLAM::BVec3::BVec3, glam::BVec3);
    assert_layout!(GLAM::Mat4::Mat4, glam::Mat4);
};
