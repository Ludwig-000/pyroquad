use pyo3::prelude::*;



/// A Settings-Class for Colliders.
/// the following options exist:
/// 
/// ```
/// ...# no collision or physics
/// >>>ColliderOptions.NONE()
/// ...
/// ...# can detect collisions
/// >>>ColliderOptions.SENSOR()
/// ...
/// ...# both physics and collision but immoveable.
/// >>>ColliderOptions.FIXED(...)
/// ...
/// ...# a dynamic object with physics and collision.
/// >>>ColliderOptions.DYNAMIC(...)
/// ```
#[cfg_attr(feature = "abi_314", pyclass(frozen, immutable_type, from_py_object, eq))]
#[cfg_attr(not(feature = "abi_314"), pyclass(frozen, from_py_object, eq))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Collider{

    /// Disables collisions and physics entirely. The object will not trigger overlap events or block movement.
    NONE(),

    /// A non-physical sensor collider. Detects overlap events without blocking movement or applying physical forces.
    SENSOR(),

    /// An immovable solid obstacle (e.g., floors, walls). Blocks dynamic objects and applies surface properties, but never moves or responds to forces.
    #[pyo3(constructor = (friction = 0.5, restitution = 0.0, is_sensor = false, detect_kinematic = true))]
    FIXED { friction: f32, restitution: f32, is_sensor: bool, detect_kinematic: bool },

    /// A fully simulated dynamic object affected by gravity, external forces, impulses, and collisions.
    #[pyo3(constructor = (gravity_scale =1.0, friction=0.5, restitution=0.7,density= 1.0))]
    DYNAMIC { gravity_scale: f32, friction: f32,restitution: f32, density:f32 }
}