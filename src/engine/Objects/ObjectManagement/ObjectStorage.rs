use crate::{engine::Objects::{Cube::Cube, Cylinder::Cylinder, Mesh::Mesh, ObjectManagement::SyncObjectTransforms::sync_transforms_inner, PhysicsWorld::ApplyPhysics::apply_physics_enum, Pill::Pill, Sphere::Sphere}, py_abstractions::structs::ThreeDObjects::{ColliderOptions::ColliderOptions, PhysicsHandle::PhysicsEnum}};
use pyo3::prelude::*;
use pyo3::types::PyWeakref;
use slotmap::*;
use std::{sync::Arc, time::Instant};
use crate::engine::Objects::PhysicsWorld::Rapier::{ObjectHandle, RapierWorld};

use crate::engine::PChannel::PSyncSender;
use macroquad::prelude as mq;

#[derive(Clone)]
pub enum Object {
    Cube(Cube),
    Sphere(Sphere),
    Mesh(Mesh),
    Pill(Pill),
    Cylinder(Cylinder),
}

#[derive(Clone, Copy)]
pub struct GlueData{
    pub reverse_lookup: ObjectKey,
    pub obj_handle:  Option<ObjectHandle>,
}

new_key_type! { pub struct ObjectKey; }
pub struct ObjectStorage {
    // Maps: Object Key -> (Vector Index, WeakRef)
    pub keymap: SlotMap<ObjectKey, (usize, Arc<Py<PyWeakref>>)>,
    
    pub storage: Vec<Object>,
    
    pub glue_data: Vec<GlueData>,
    
    pub physics_world: RapierWorld,
}
impl ObjectStorage {
    pub fn new() -> ObjectStorage {
        ObjectStorage {
            keymap: SlotMap::default(),
            storage: Vec::new(),
            glue_data: Vec::new(),
            physics_world: RapierWorld::new(),
        }
    }

    pub fn push(&mut self, obj: Object,collider: ColliderOptions, weak_ref_handle: Py<PyWeakref>) -> ObjectKey {

        
        let idx = self.storage.len(); // the index of where the obj will be placed eventually.
        let key = self.keymap.insert((idx, Arc::new(weak_ref_handle)));

        // gets the glue data.
        let glue  = GlueData{  reverse_lookup: key, obj_handle: self.physics_world.insert_object(&obj, key,collider)};
        self.glue_data.push(glue);

        self.storage.push(obj);
        
        key
    }

    /// Returns the Oject key through the Sender, before the object has been created.
    pub fn quick_push<F: FnOnce()-> Object>(&mut self,
        collider: ColliderOptions,
        sender: PSyncSender<ObjectKey>,
        weak_ref_handle: Py<PyWeakref>,
        object_factory: F){


        let idx = self.storage.len();
        let key = self.keymap.insert((idx, Arc::new(weak_ref_handle)));
        let _ = sender.send(key);

        let obj = object_factory();

        let glue  = GlueData{  reverse_lookup: key, obj_handle: self.physics_world.insert_object(&obj, key,collider)};
        self.glue_data.push(glue);

        self.storage.push(obj);
    }

    pub fn remove_object(&mut self, key: ObjectKey) {
    
        let idx_to_remove = match self.keymap.get(key) {
            Some(val) => val.0,
            None => panic!("Key not Found"), 
        };
    
        let last_idx = self.storage.len() - 1; 
    
        self.storage.swap(idx_to_remove, last_idx);
        
        self.glue_data.swap(idx_to_remove, last_idx);
    
        if idx_to_remove != last_idx {
            
            let key_of_moved_obj = self.glue_data[idx_to_remove].reverse_lookup;
    
            if let Some((stored_idx, _)) = self.keymap.get_mut(key_of_moved_obj) {
                *stored_idx = idx_to_remove;
            }
        }

        self.storage.pop();

        

        let obj_data=  self.glue_data.pop().expect("tried to remove obj-data, but array is empty.");

        if let Some(obj_handle ) = obj_data.obj_handle {
            self.physics_world.remove_object(obj_handle);
        }
        
        self.keymap.remove(key);
        
    }

    
    pub fn get_pyref(&self, key: ObjectKey) -> Arc<Py<PyWeakref>> {
        self.keymap.get(key).expect("WeakRef not found for key").1.clone()
    }

    /// changing values here, may result in a de-sync with the mesh.
    /// thats why its unsafe.
    pub unsafe fn get_handle_mut(&mut self, key: ObjectKey) -> &mut Option<ObjectHandle>{
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        &mut self.glue_data.get_mut(*vec_idx).expect("missing object data").obj_handle
    }

    pub fn get_handle(&self, key: ObjectKey) -> &Option<ObjectHandle>{
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        &self.glue_data.get(*vec_idx).expect("missing object data").obj_handle
    }

    /// changing values here, may result in a de-sync with the collider.
    /// thats why its unsafe.
    pub unsafe fn get_mut(&mut self, key: ObjectKey) -> &mut Object {

        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        self.storage.get_mut(*vec_idx).expect("missing object")
        
    }


    pub fn get(&self, key: ObjectKey) -> &Object {
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        self.storage.get(*vec_idx).expect("missing object")
    }

    pub fn len(&self)-> usize{
        self.storage.len()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Object> {
        self.storage.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Object> {
        self.storage.iter()
    }


    pub fn step_physics(&mut self, distance: f32){
        self.physics_world.step(distance);
        self.sync_transforms();
    }

    /// returns false if collision is disabled.
    pub fn does_collide(&mut self, key: ObjectKey)-> bool{

        if let Some(object_handle )= self.get_handle(key){
            return self.physics_world.has_collision(object_handle.collider_handle)
        }
        false
    }

    /// returns empty if collision is disabled.
    pub fn collides_with(&mut self, key: ObjectKey)-> Vec<ObjectKey>{
        if let Some(object_handle )= self.get_handle(key){
            return self.physics_world.get_collided_keys(object_handle.collider_handle)
        }
        Vec::new()
    }

    pub fn keys_to_py(&mut self, keys: Vec<ObjectKey>)-> Vec<Arc<Py<PyWeakref>>>{
        let mut v = Vec::new();
        for key in keys{
            let p = self.get_pyref(key);
            v.push(p);
        }
        v
    }

    /// the user-provided function is responsible for changing ALL internal obj values.
    pub fn change_obj_position<T: FnOnce(&mut Object)>(&mut self, location: &mq::Vec3,  key: ObjectKey,obj_recalc: T){

        {
            if let Some(glue) = *self.get_handle(key){
                self.physics_world.move_object(&glue.rigid_body_handle, location);
            }
        }
        let obj  = unsafe {self.get_mut(key)};
        obj_recalc(obj);
        
    }
    /// the user-provided function is responsible for changing ALL internal obj values.
    pub fn change_obj_rotation<T: FnOnce(&mut Object)>(&mut self, rotation: &mq::Vec3, key: ObjectKey, obj_recalc: T){

        {
            if let Some(glue) = *self.get_handle(key){
                self.physics_world.rotate_object(&glue.rigid_body_handle, rotation);
            }
        }
        let obj  = unsafe {self.get_mut(key)};
        obj_recalc(obj);
        
    }
    /// the user-provided function is responsible for changing ALL internal obj values.
    pub fn change_obj_scale<T: FnOnce(&mut Object)>(&mut self, scale: &mq::Vec3, key: ObjectKey, obj_recalc: T) {
        {
            let obj = unsafe { self.get_mut(key) };
            obj_recalc(obj);
        }

        let (obj_clone, handle_copy) = {
            let obj: Object = self.get(key).clone();
            let handle = *self.get_handle(key);
            (obj, handle)
        };

        if let Some(handle) = handle_copy {
            self.physics_world.scale_object(&handle.collider_handle, &obj_clone, scale);
        }
    }

    pub fn set_collision_for_object(&mut self, key: ObjectKey, collider: ColliderOptions) {
        let vec_idx = self.keymap.get(key).expect("key not known to the map.").0;

        if let Some(old_handle) = self.glue_data[vec_idx].obj_handle.take() {
            self.physics_world.remove_object(old_handle);
        }

        let obj = &self.storage[vec_idx];

        let new_handle = self.physics_world.insert_object(obj, key, collider);
        
        self.glue_data[vec_idx].obj_handle = new_handle;
    }


    pub fn sync_transforms(&mut self) {
        sync_transforms_inner(self);
    }


    pub fn apply_physics_enum(&mut self, settings: PhysicsEnum, handle: &ObjectHandle) {
        let world = &mut self.physics_world;
        apply_physics_enum(settings, world, handle);
    }


}





//REFERENCE CODE FOR LATER:
// 
// use rayon::prelude::*;
// use std::sync::atomic::{AtomicUsize, Ordering};

// struct Mesh {
//     x: f32,
//     y: f32,
// }

// // Wrapper to allow passing the raw pointer to threads
// struct UnsafeSlice<'a, T> {
//     slice: &'a mut [T],
// }

// unsafe impl<'a, T> Sync for UnsafeSlice<'a, T> {}
// unsafe impl<'a, T> Send for UnsafeSlice<'a, T> {}

// impl<'a, T> UnsafeSlice<'a, T> {
//     fn new(slice: &'a mut [T]) -> Self {
//         Self { slice }
//     }

//     /// SAFETY: Caller must ensure no two threads access the same index
//     unsafe fn get_mut(&self, index: usize) -> &mut T {
//         &mut self.slice[index]
//     }
// }

// fn update_meshes_unsafe(meshes: &mut Vec<Mesh>, indices: &[usize]) {
//     let unsafe_slice = UnsafeSlice::new(meshes);

//     // Parallel iterate over the INDICES, not the meshes
//     indices.par_iter().for_each(|&idx| {
//         unsafe {
//             let mesh = unsafe_slice.get_mut(idx);
//             // heavy calculation
//             mesh.x += 1.0;
//         }
//     });
// }