use macroquad::prelude::Vertex as mq_vert;
use pyo3::ffi::PyCallable_Check;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use pyo3_stub_gen::inventory::submit;
use slotmap::DefaultKey;
use slotmap::Key;
use std::hash::{Hash, Hasher};

use crate::engine::Objects::ObjectDataCache::ThreeDObjCache;
use crate::engine::PError::PError;
use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::{ColliderOptions, InnerColliderOptions};
use crate::py_abstractions::structs::ThreeDObjects::PhysicsHandle::Physics;
use crate::py_abstractions::{Loading::FileData::FileData, Textures_and_Images::Texture2D};
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::Objects::Mesh as internal_mesh;
use pyo3::prelude::PyResult;
use crate::engine::CoreLoop::{Command,COMMAND_QUEUE};
use pyo3::prelude::*;

use crate::engine::PChannel::PChannel;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage::FunctionKey;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;

use pyo3::exceptions::*;



use crate::engine::Objects::ObjectDataCache;

use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;
use crate::py_abstractions::structs::ThreeDObjects::ObjectMacros::*;

#[gen_stub_pyclass]
#[pyclass(weakref)]
pub struct Mesh{
	pub key: ObjectKey,
    pub function_key: Option<FunctionKey>,

    // an object's data can only be cached, if it can NOT be influenced by anything external.
    // F.E.: gravity.
    pub cache: Option<ObjectDataCache::ThreeDObjCache>,

    /// a collection of physics-related methods, that can be applied to the object.
    /// 'physics' will be set to Some() if the object is innitialized as a dynamic object.
    #[pyo3(get)]
    pub physics: Option<Py<Physics>>,
}

crate::implement_basic_magic_methods3D!(Mesh);
crate::implement_basic_getter_methods3D!(Mesh);
crate::implement_basic_setter_methods3D!(Mesh);
crate::implement_check_collision3D!(Mesh);
crate::implement_set_collider3D!(Mesh);
crate::implement_tick3D!(Mesh, r#"Mesh.from_file_data(...)"#);
crate::implement_manual_drawing_options3D!(Mesh, r#"Mesh.from_file_data(...)"#);
crate::implement_remove_tick3D!(Mesh);
crate::implement_Drop3D!(Mesh);

#[gen_stub_pymethods]
#[pymethods]
impl Mesh{

    #[staticmethod]
    pub fn from_file_data(py: Python<'_>,data: FileData, texture: Option<Texture2D>,collider_type: ColliderOptions)-> PyResult<Py<Mesh>>{
        
        let (sender, receiver) = PChannel::sync_channel(1);

        let placeholder_struct: Self = Self { key: ObjectKey::null(), 
            function_key: None, 
            cache: None,
            physics: None,
        };

        let mesh_handle: Py<Self> = Py::new(py, placeholder_struct)?; 
        
        let weak_ref_handle: Py<PyWeakref> = {
            let bound_mesh = mesh_handle.bind(py); 
            let weak_ref_ref = PyWeakrefReference::new(&bound_mesh)?;
            weak_ref_ref.cast_into::<PyWeakref>()?.unbind() 
        };

        let mesh =  internal_mesh::Mesh::load_from_gltf(&data.bytes, texture.map(|t|t.into())).map_err(|e|{
            PError::GLTFError(e)
        })?;

        COMMAND_QUEUE.push(Command::CreateMesh { mesh, collider: collider_type, weak_ref: weak_ref_handle.clone_ref(py), sender });
        
        let key = receiver.recv()?;
        
        let mut mesh_ref = mesh_handle.borrow_mut(py);
        mesh_ref.key = key;


        match collider_type.0{
            InnerColliderOptions::Dynamic { gravity_scale, friction, restitution, density }=>{
                let phys_struct = Physics {
                    identity: weak_ref_handle,
                    handle: key,
                };
                mesh_ref.physics = Some(Py::new(py, phys_struct)?);
            },
            _ => {}
        }

        drop(mesh_ref);
        Ok(mesh_handle) 
    }

}
