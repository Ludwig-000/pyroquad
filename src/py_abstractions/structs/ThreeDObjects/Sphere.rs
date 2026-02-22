use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use pyo3::exceptions::*;
use crate::engine::PChannel::PChannel;
use crate::py_abstractions::Textures_and_Images::Texture2D;
use crate::py_abstractions::structs::ThreeDObjects::PhysicsHandle::Physics;
use std::hash::{Hash, Hasher};

use slotmap::DefaultKey;
use slotmap::Key;

use crate::engine::Objects::ObjectDataCache;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::{ColliderOptions, InnerColliderOptions};
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage::FunctionKey;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;

#[gen_stub_pyclass]
#[pyclass(weakref)]
pub struct Sphere{
    key: ObjectKey, // The key to the actual underlying cube, stored inside "ObjectStorage".

    // Key to a function inside 'function storage', which will be run each frame by the engine.
    function_key: Option<FunctionKey>,

    /// we add a cache for trivial data, which can be used if the object is not
    /// influenced by outside forces F.E. Gravity.
    cache: Option<ObjectDataCache::ThreeDObjCache>,

    /// a collection of physics-related methods, that can be applied to the object.
    /// 'physics' will be set to Some() if the object is innitialized as a dynamic object.
    #[pyo3(get)]
    pub physics: Option<Py<Physics>>,
}

crate::implement_basic_magic_methods3D!(Sphere);
crate::implement_basic_getter_methods3D!(Sphere);
crate::implement_basic_setter_methods3D!(Sphere);
crate::implement_check_collision3D!(Sphere);
crate::implement_set_collider3D!(Sphere);
crate::implement_tick3D!(Sphere,  r#"Sphere()"#);
crate::implement_manual_drawing_options3D!(Sphere,  r#"Sphere()"#);
crate::implement_remove_tick3D!(Sphere);
crate::implement_Drop3D!(Sphere);

#[gen_stub_pymethods]
#[pymethods]
impl Sphere {

    #[pyo3(signature = (position= Vec3::ZERO(), rotation = Vec3::ZERO(),scale= Vec3::ONE(), color = Color::WHITE(),texture=None, collider_type = ColliderOptions::NONE()))]
    #[new]
    pub fn new(
        py: Python<'_>,
        position: Vec3,
        rotation: Vec3,
        scale: Vec3,
        color: Color,
        texture: Option<Texture2D>,
        collider_type: ColliderOptions,
    ) -> PyResult<Py<Sphere>> {

        let (sender, receiver) = PChannel::sync_channel(1);

        let cache  =match collider_type.0{
            InnerColliderOptions::Dynamic { gravity_scale, friction, restitution, density }=>{
                None
            },
            _=> {ObjectDataCache::ThreeDObjCache::new(true, position.into(), rotation.into(), scale.into(), color.into())}
        };
        let placeholder_struct = Self { key: ObjectKey::null(),function_key: None,  cache, physics: None};
        let cube_handle: Py<Self> = Py::new(py, placeholder_struct)?; 
        
        let weak_ref_handle: Py<PyWeakref> = {
            let bound_cube = cube_handle.bind(py); 
            let weak_ref_ref = PyWeakrefReference::new(&bound_cube)?;
            weak_ref_ref.cast_into::<PyWeakref>()?.unbind() 
        };

        COMMAND_QUEUE.push(Command::CreateSphere { 
            size: scale.into(), 
            position: position.into(), 
            rotation: rotation.into(), 
            color: color.into(),
            texture: texture.map(|t|t.into()),
            collider: collider_type,
            weak_ref: weak_ref_handle.clone_ref(py),
            sender 
        });
        
        let key = receiver.recv()?;
        
        let mut cube_ref = cube_handle.borrow_mut(py);
        cube_ref.key = key;

        match collider_type.0{
            InnerColliderOptions::Dynamic { gravity_scale, friction, restitution, density }=>{
                let phys_struct = Physics {
                    identity: weak_ref_handle,
                    handle: key,
                };
                cube_ref.physics = Some(Py::new(py, phys_struct)?);
            },
            _ => {}
        }

        drop(cube_ref);

        Ok(cube_handle)
    }
        
        

}