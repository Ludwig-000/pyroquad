use pyo3::prelude::*;

use pyo3::types::{PyWeakref, PyWeakrefReference};
use pyo3::exceptions::*;

use crate::py_abstractions::Textures_and_Images::Texture2D;
use crate::engine::PChannel::PChannel;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunStorage::FunctionKey;
use crate::py_abstractions::structs::ThreeDObjects::PhysicsHandle::Physics;
use std::hash::{Hash, Hasher};

use slotmap::Key;
use crate::engine::Objects::ObjectDataCache;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::ThreeDObjects::Collider::Collider;
use crate::py_abstractions::Color::Color;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;


#[pyclass(weakref, dict)]
pub struct Cube{
    key: ObjectKey, // The key to the actual underlying cube, stored inside "ObjectStorage".

    // Key to a function inside 'function storage', which will be run each frame by the engine.
    function_key: Option<FunctionKey>,

    /// we add a cache for trivial data, which can be used if the object is not
    /// influenced by outside forces F.E. Gravity.
    cache: Option<ObjectDataCache::ThreeDObjCache>,

    /// a collection of physics-related methods, that can be applied to the object.
    /// 'physics' will not be `None` if the object is innitialized as a dynamic object.
    #[pyo3(get)]
    pub physics: Option<Py<Physics>>,
}


crate::implement_basic_magic_methods3D!(Cube);
crate::implement_basic_getter_methods3D!(Cube);
crate::implement_basic_setter_methods3D!(Cube);
crate::implement_check_collision3D!(Cube);
crate::implement_set_collider3D!(Cube);
crate::implement_tick3D!(Cube,  r#"Cube()"#);
crate::implement_remove_tick3D!(Cube);
crate::implement_Drop3D!(Cube);
crate::implement_manual_drawing_options3D!(Cube,  r#"Cube()"#);
crate::implement_rotate_around3D!(Cube,  r#"Cube()"#);

#[pymethods]
impl Cube {
    
    #[pyo3(signature = (position= Vec3::ZERO(), rotation = Vec3::ZERO(),scale= Vec3::ONE(), color = Color::WHITE(), texture=None,collider_type = Collider::NONE()))]
    #[new]
    pub fn new(
        py: Python<'_>,
        position: Vec3,
        rotation: Vec3,
        scale: Vec3,
        color: Color,
        texture: Option<Texture2D>,
        collider_type: Collider,
    ) -> PyResult<Py<Cube>> {

        let (sender, receiver) = PChannel::channel();

        let cache = match collider_type{
            Collider::DYNAMIC { .. }=>{
                None
            },
            _=> {ObjectDataCache::ThreeDObjCache::new(position.into(), rotation.into(), scale.into(), color.into())}
        };
        let placeholder_struct: Cube = Cube { key: ObjectKey::null(),function_key: None,  cache, physics: None};
        let cube_handle: Py<Cube> = Py::new(py, placeholder_struct)?; 
        
        let weak_ref_handle: Py<PyWeakref> = {
            let bound_cube = cube_handle.bind(py); 
            let weak_ref_ref = PyWeakrefReference::new(bound_cube)?;
            weak_ref_ref.cast_into::<PyWeakref>()?.unbind() 
        };

        COMMAND_QUEUE.push(Command::CreateCube { 
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

        if let Collider::DYNAMIC { .. } = collider_type{
            let phys_struct = Physics {
                identity: weak_ref_handle,
                handle: key,
            };
            cube_ref.physics = Some(Py::new(py, phys_struct)?);
        }

        drop(cube_ref);

        Ok(cube_handle)
    }

    /// defining a cube by its center can be annoying, so this allows for an alternate constructor.
    /// x1,y1,z1 and x2,y2,z2 are two opposite corners of the cube.
    #[staticmethod]
    #[pyo3(signature = (x1=0., y1=0., z1=0., x2=1., y2=1., z2=1., color = Color::WHITE(), texture=None, collider_type = Collider::NONE()))]
    pub fn from_xyz(
        py: Python<'_>,
        x1: f32, y1: f32, z1: f32,
        x2: f32, y2: f32, z2: f32,
        color: Color,
        texture: Option<Texture2D>,
        collider_type: Collider,
    ) -> PyResult<Py<Cube>> {
        let position = Vec3::const_new((x1 + x2) / 2.0, (y1 + y2) / 2.0, (z1 + z2) / 2.0);
        let scale = Vec3::const_new((x2 - x1).abs(), (y2 - y1).abs(), (z2 - z1).abs());

        Cube::new(py, position, Vec3::ZERO(), scale, color, texture, collider_type)
    }
}
