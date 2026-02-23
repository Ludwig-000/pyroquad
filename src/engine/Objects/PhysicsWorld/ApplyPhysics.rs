use crate::{engine::Objects::PhysicsWorld::{Rapier::{ObjectHandle, RapierWorld}}, 
    py_abstractions::structs::ThreeDObjects::PhysicsHandle::PhysicsEnum};


use rapier3d::prelude::*;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::BVec3::BVec3;




pub fn apply_physics_enum(settings: PhysicsEnum, world: &mut RapierWorld, handle: &ObjectHandle) {

    let rb = match world.rigidBS.get_mut(handle.rigid_body_handle) {
        Some(r) => r,
        None => panic!("Could not get Rigid body from handle."),
    };

    match settings {
        // ------------------------------------------------------------------
        // Velocity Management
        // ------------------------------------------------------------------
        PhysicsEnum::SetLinearVelocity(v) => {
            rb.set_linvel(vector![v.x, v.y, v.z], true);
        }
        PhysicsEnum::GetLinearVelocity( sender) => {
            let v = rb.linvel();
            let _ = sender.send(Vec3 { x: v.x, y: v.y, z: v.z });
        }
        PhysicsEnum::SetAngularVelocity(v) => {
            rb.set_angvel(vector![v.x, v.y, v.z], true);
        }
        PhysicsEnum::GetAngularVelocity( sender) => {
            let v = rb.angvel();
            let _ = sender.send(Vec3 { x: v.x, y: v.y, z: v.z });
        }

        // ------------------------------------------------------------------
        // External Influences
        // ------------------------------------------------------------------
        PhysicsEnum::ApplyImpulse(v) => {
            rb.apply_impulse(vector![v.x, v.y, v.z], true);
        }
        PhysicsEnum::AddForce(v) => {
            
            rb.add_force(vector![v.x, v.y, v.z], true);
        }
        PhysicsEnum::AddTorque(v) => {
            rb.add_torque(vector![v.x, v.y, v.z], true);
        }
        PhysicsEnum::AddTorqueImpulse(v) => {
            rb.apply_torque_impulse(vector![v.x, v.y, v.z], true);
        }

        // ------------------------------------------------------------------
        // Constraints & Locking
        // ------------------------------------------------------------------
        PhysicsEnum::LockRotationXAxes(lock) => {
            let mut locks = rb.locked_axes();
            locks.set(LockedAxes::ROTATION_LOCKED_X, lock);
            rb.set_locked_axes(locks, true);
        }
        PhysicsEnum::LockRotationYAxes(lock) => {
            let mut locks = rb.locked_axes();
            locks.set(LockedAxes::ROTATION_LOCKED_Y, lock);
            rb.set_locked_axes(locks, true);
        }
        PhysicsEnum::LockRotationZAxes(lock) => {
            let mut locks = rb.locked_axes();
            locks.set(LockedAxes::ROTATION_LOCKED_Z, lock);
            rb.set_locked_axes(locks, true);
        }
        PhysicsEnum::GetRotationLocks( sender) => {
            let locks = rb.locked_axes();
            let res = BVec3 {
                x: locks.contains(LockedAxes::ROTATION_LOCKED_X),
                y: locks.contains(LockedAxes::ROTATION_LOCKED_Y),
                z: locks.contains(LockedAxes::ROTATION_LOCKED_Z),
            };
            let _ = sender.send(res);
        }

        // ------------------------------------------------------------------
        // Mass & Material Properties (RigidBody)
        // ------------------------------------------------------------------
        PhysicsEnum::SetGravityScale(val) => {
            rb.set_gravity_scale(val, true);
        }
        PhysicsEnum::GetGravityScale( sender) => {
            let _ = sender.send(rb.gravity_scale());
        }
        PhysicsEnum::SetLinearDamping(val) => {
            rb.set_linear_damping(val);
        }
        PhysicsEnum::GetLinearDamping( sender) => {
            let _ = sender.send(rb.linear_damping());
        }
        PhysicsEnum::SetAngularDamping(val) => {
            rb.set_angular_damping(val);
        }
        PhysicsEnum::GetAngularDamping(sender) => {
            let _ = sender.send(rb.angular_damping());
        }
        PhysicsEnum::EnableCcd(enabled) => {
            rb.enable_ccd(enabled);
        }
        PhysicsEnum::IsCcdEnabled(sender) => {
            let _ = sender.send(rb.is_ccd_enabled());
        }

        // ------------------------------------------------------------------
        // Mass & Material Properties (Collider)
        // ------------------------------------------------------------------
        // These require accessing the ColliderSet using the collider_handle
        // Note: We use a block here to scope the mutable borrow of 'world.coll'
        // separate from 'rb' if needed, though here we just need to access 'world.coll'.
        PhysicsEnum::SetFriction(val) => {
            if let Some(coll) = world.coll.get_mut(handle.collider_handle) {
                coll.set_friction(val);
            }
        }
        PhysicsEnum::GetFriction( sender) => {
            if let Some(coll) = world.coll.get(handle.collider_handle) {
                let _ = sender.send(coll.friction());
            } else {
                // If collider is missing, send 0.0 or handle error
                let _ = sender.send(0.0); 
            }
        }
        PhysicsEnum::SetRestitution(val) => {
            if let Some(coll) = world.coll.get_mut(handle.collider_handle) {
                coll.set_restitution(val);
            }
        }
        PhysicsEnum::GetRestitution( sender) => {
            if let Some(coll) = world.coll.get(handle.collider_handle) {
                let _ = sender.send(coll.restitution());
            } else {
                let _ = sender.send(0.0);
            }
        }
        
        PhysicsEnum::SetDensity(val) => {
                // 1. Update collider density
                if let Some(coll) = world.coll.get_mut(handle.collider_handle) {
                coll.set_density(val);
            }
            // 2. IMPORTANT: Tell RigidBody to recompute mass from new density
            // We need to re-borrow RB here because we might have dropped it 
            // implicitly if we structured this differently, but here rb is still open.
            // However, recompute requires reference to the ColliderSet.
            // 'rb' holds a mutable reference to 'world.rigidBS'.
            // 'world.coll' is a separate field. Rust borrow checker should allow this 
            // if we are careful, but usually we call a helper on the Set.
            
            // Re-fetching RB to ensure we have access alongside ColliderSet
                if let Some(rb) = world.rigidBS.get_mut(handle.rigid_body_handle) {
                    rb.recompute_mass_properties_from_colliders(&world.coll);
                }
        }
        PhysicsEnum::GetDensity( sender) => {
            if let Some(coll) = world.coll.get(handle.collider_handle) {
                let _ = sender.send(coll.density());
            } else {
                let _ = sender.send(0.0);
            }
        }

        PhysicsEnum::GetMass( sender) => {
            let _ = sender.send(rb.mass());
        }
    }
}