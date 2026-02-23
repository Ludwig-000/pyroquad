use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;


/// A Settings-Class for Colliders.
/// the following options exist:
/// 
/// ```
/// ...# no collision or physics
/// >>>ColliderOptions.NONE 
/// ...
/// ...# collision but no physics
/// >>>ColliderOptions.STATIC
/// ...
/// ...# both physics and collision.
/// >>>ColliderOptions.DYNAMIC(...)
/// ```
#[gen_stub_pyclass]
#[cfg_attr(feature = "abi_314", pyclass(frozen, immutable_type))]
#[cfg_attr(not(feature = "abi_314"), pyclass(frozen))]
#[derive(Clone, Copy)]
pub struct ColliderOptions(pub InnerColliderOptions);

#[gen_stub_pymethods]
#[pymethods]
impl ColliderOptions{

    #[classattr]
    pub fn NONE() -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::None)
    }

    #[classattr]
    pub fn STATIC() -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::Static)
    }

    #[pyo3(signature=(gravity_scale =1.0, friction=0.5, restitution=0.7,density= 1.0))]
    #[staticmethod]
    pub fn DYNAMIC(gravity_scale: f32, friction: f32,restitution: f32, density:f32 ) -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::Dynamic { gravity_scale,friction,restitution,density })
    }
}

#[derive(Clone,Copy)]
pub enum InnerColliderOptions{
    None,
    Static,
    Dynamic{
        gravity_scale: f32,
        friction: f32,
        restitution: f32,
        density: f32
    }
}