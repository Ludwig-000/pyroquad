
use pyo3::{ pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};


use crate::py_abstractions::structs::GLAM::BVec3::BVec3;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use pyo3::prelude::PyResult;
use crate::engine::CoreLoop::{Command,COMMAND_QUEUE};
use pyo3::prelude::*;

use pyo3::ffi;
use crate::engine::PChannel::{self,};


use pyo3::types::{PyWeakref};

use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;

/// Physics that are bound to a Dynamic 3D object.
/// This class can only be used by making a Dynamic 3D object, and accessing it's 'physics' field.
#[gen_stub_pyclass]
#[cfg_attr(feature = "abi_314", pyclass(frozen, immutable_type))]
#[cfg_attr(not(feature = "abi_314"), pyclass(frozen))]
pub struct Physics{
    pub identity: Py<PyWeakref>,
    pub handle: ObjectKey
}

#[gen_stub_pymethods]
#[pymethods]
impl Physics{
    pub fn set_linear_velocity(&self, py: Python<'_>, velocity: Vec3)-> PyResult<()> {
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetLinearVelocity(velocity), self.handle
            )
        );
        Ok(())
    }

    pub fn get_linear_velocity(&self, py: Python<'_>) -> PyResult<Vec3> {
        self.is_alive(py)?;
        let (tx, rx) = PChannel::PChannel::sync_channel(1);
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::GetLinearVelocity(tx), self.handle
            )
        );

        let res = rx.recv()?;
        Ok(res)
    }

    pub fn set_angular_velocity(&self, py: Python<'_>, velocity: Vec3) -> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetAngularVelocity(velocity), self.handle
            )
        );
        Ok(())
    }

    pub fn get_angular_velocity(&self, py: Python<'_>) -> PyResult<Vec3>{
        self.is_alive(py)?;
        let (tx, rx) = PChannel::PChannel::sync_channel(1);
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::GetAngularVelocity(tx), self.handle
            )
        );

        let res = rx.recv()?;
        Ok(res)
    }

    pub fn apply_impulse(&self, py: Python<'_>, impulse: Vec3)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::ApplyImpulse(impulse), self.handle
            )
        );
        Ok(())
    }

    pub fn add_force(&self, py: Python<'_>, force: Vec3)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::AddForce(force), self.handle
            )
        );
        Ok(())
    }

    pub fn add_torque(&self, py: Python<'_>, force: Vec3)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::AddTorque(force), self.handle
            )
        );
        Ok(())
    }

    pub fn add_torque_impulse(&self, py: Python<'_>, impulse: Vec3)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::AddTorqueImpulse(impulse), self.handle
            )
        );
        Ok(())
    }

    pub fn lock_rotation_x_Axes(&self, py: Python<'_>, set: bool)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::LockRotationXAxes(set), self.handle
            )
        );
        Ok(())
    }

    pub fn lock_rotation_y_Axes(&self, py: Python<'_>, set: bool)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::LockRotationYAxes(set), self.handle
            )
        );
        Ok(())
    }

    pub fn lock_rotation_z_Axes(&self, py: Python<'_>, set: bool)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::LockRotationZAxes(set), self.handle
            )
        );
        Ok(())
    }

    pub fn set_gravity_scale(&self, py: Python<'_>, gravity: f32)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetGravityScale(gravity), self.handle
            )
        );
        Ok(())
    }

    pub fn set_friction(&self, py: Python<'_>, friction: f32)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetFriction(friction), self.handle
            )
        );
        Ok(())
    }

    pub fn set_restitution(&self, py: Python<'_>, restitiution: f32)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetRestitution(restitiution), self.handle
            )
        );
        Ok(())
    }

    pub fn set_density(&self, py: Python<'_>, density: f32)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetDensity(density),
                self.handle
                
            )
        );
        Ok(())
    }

    pub fn set_linear_damping(&self, py: Python<'_>, damping: f32)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetLinearDamping(damping), self.handle
            )
        );
        Ok(())
    }

    pub fn set_angular_damping(&self, py: Python<'_>, damping: f32)-> PyResult<()>{
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::SetAngularDamping(damping), self.handle
            )
        );
        Ok(())
    }

    /// continuous collision detection.
    pub fn enable_ccd(&self, py: Python<'_>, enabled: bool) -> PyResult<()>{ 
        self.is_alive(py)?;
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::EnableCcd(enabled), self.handle
            )
        );
        Ok(()) 
    }

    pub fn is_ccd_enabled(&self, py: Python<'_>) -> PyResult<bool>{ 
        self.is_alive(py)?;
        let (tx, rx) = PChannel::PChannel::sync_channel(1);
        COMMAND_QUEUE.push(
            Command::PhysicsEnum(
                PhysicsEnum::IsCcdEnabled(tx), self.handle
            )
        );

        let res = rx.recv()?;
        Ok(res) 
    }
}

impl Physics{

    /// we have a pretty big problem if we don't run this checks.
    /// The engine assumes any key that still exists is also valid.
    /// Since a Physics handle can outlive it's Object of origin while still holding it's key,
    /// we need this check.
    pub fn is_alive<'py>(&self, py: Python<'py>)-> PyResult<()>{
        let alive=  unsafe {
            let weak_ptr = self.identity.as_ptr();
            let target_ptr = ffi::PyWeakref_GetObject(weak_ptr);
            !target_ptr.is_null() && target_ptr != ffi::Py_None()
        };
        if !alive {
            return Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                
                "The object, which this physics class is attatched to, no longer exists.
                A Physics handle may not outlive the Object it is linked to."

            ));
        }
        Ok(())
        
    }
}



pub enum PhysicsEnum {
    // --- Velocity Management ---
    SetLinearVelocity(Vec3),
    /// Returns current Vec3 velocity
    GetLinearVelocity( PChannel::PSyncSender<Vec3>),
    
    SetAngularVelocity(Vec3),
    /// Returns current Vec3 angular velocity
    GetAngularVelocity( PChannel::PSyncSender<Vec3>),

    // --- External Influences ---
    ApplyImpulse(Vec3),
    AddForce(Vec3),

    AddTorque(Vec3),
    AddTorqueImpulse(Vec3),

    // --- Constraints & Locking ---
    LockRotationXAxes(bool),
    LockRotationYAxes(bool),
    LockRotationZAxes(bool),
    /// Returns (x, y, z) lock status as a tuple or BVec3 equivalent
    GetRotationLocks( PChannel::PSyncSender<BVec3>),

    // --- Mass & Material Properties ---
    SetGravityScale(f32),
    GetGravityScale( PChannel::PSyncSender<f32>),
    
    GetMass( PChannel::PSyncSender<f32>),
    
    SetFriction(f32),
    GetFriction( PChannel::PSyncSender<f32>),
    
    SetRestitution(f32),
    GetRestitution( PChannel::PSyncSender<f32>),
    
    SetDensity(f32, ),
    GetDensity( PChannel::PSyncSender<f32>),

    SetLinearDamping(f32),
    GetLinearDamping( PChannel::PSyncSender<f32>),
    
    SetAngularDamping(f32),
    GetAngularDamping( PChannel::PSyncSender<f32>),

    EnableCcd(bool),
    IsCcdEnabled( PChannel::PSyncSender<bool>),
}