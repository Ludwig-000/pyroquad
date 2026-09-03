use pyo3::prelude::*;



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
#[cfg_attr(feature = "abi_314", pyclass(frozen, immutable_type, from_py_object, eq))]
#[cfg_attr(not(feature = "abi_314"), pyclass(frozen, from_py_object, eq))]
#[derive(Clone, Copy, PartialEq)]
pub struct ColliderOptions(pub InnerColliderOptions);

#[pymethods]
impl ColliderOptions{

    /// Disables collisions and physics entirely. The object will not trigger overlap events or block movement.
    #[classattr]
    pub fn NONE() -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::None)
    }

    /// A non-physical sensor collider. Detects overlap events without blocking movement or applying physical forces.
    #[classattr]
    pub fn STATIC() -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::Static)
    }

    /// An immovable solid obstacle (e.g., floors, walls). Blocks dynamic objects and applies surface properties, but never moves or responds to forces.
    #[pyo3(signature = (friction = 0.5, restitution = 0.0, is_sensor = false, detect_kinematic = true))]
    #[staticmethod]
    pub fn FIXED(friction: f32, restitution: f32, is_sensor: bool, detect_kinematic: bool) -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::Fixed { friction, restitution, is_sensor, detect_kinematic })
    }

    /// A fully simulated dynamic object affected by gravity, external forces, impulses, and collisions.
    #[pyo3(signature=(gravity_scale =1.0, friction=0.5, restitution=0.7,density= 1.0))]
    #[staticmethod]
    pub fn DYNAMIC(gravity_scale: f32, friction: f32,restitution: f32, density:f32 ) -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::Dynamic { gravity_scale,friction,restitution,density })
    }
    
}

#[derive(Clone,Copy, PartialEq)]
pub enum InnerColliderOptions{
    None,
    Static,
    Fixed{
        friction: f32,
        restitution: f32,
        is_sensor: bool,
        detect_kinematic: bool,
    },
    Dynamic{
        gravity_scale: f32,
        friction: f32,
        restitution: f32,
        density: f32
    }
}