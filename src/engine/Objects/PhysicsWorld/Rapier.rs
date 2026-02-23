use rapier3d::{prelude::*};
use glam::Vec3;
use slotmap::{Key, KeyData};
use crate::{engine::Objects::ObjectManagement::ObjectStorage::{self as obj}, py_abstractions::structs::ThreeDObjects::ColliderOptions::{ColliderOptions, InnerColliderOptions}};
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;



#[derive(Clone, Copy)]
pub struct ObjectHandle {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}




pub struct Transforms<'a>{
    pos: &'a Vec3,
    rot: &'a Vec3,
    scale: &'a Vec3,
}
pub struct RapierWorld{
    // pipeline: CollisionPipeline,
    pub pipeline: PhysicsPipeline,

    pub bvh: BroadPhaseBvh,
    pub coll: ColliderSet,
    pub narrowP: NarrowPhase,
    pub rigidBS: RigidBodySet,

    // unused stuff Rapier needs to function.
    pub islands: IslandManager,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,

    // possibly evil idk yet.
    pub ccd_solver: CCDSolver,
    pub integration_parameters: IntegrationParameters,
}
impl RapierWorld{
    pub fn new()-> RapierWorld{
        RapierWorld{
            pipeline: PhysicsPipeline::new(),
            bvh: BroadPhaseBvh::new(),
            coll: ColliderSet::new(),
            narrowP: NarrowPhase::new(),
            rigidBS: RigidBodySet::new(),
            islands: IslandManager::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            integration_parameters: IntegrationParameters::default(),
        }
    }

    pub fn move_object(&mut self, handle: &RigidBodyHandle, pos: &Vec3) {
        let rb  = self.rigidBS.get_mut(*handle).expect("Missing Rigid body handle!!??");
        rb.set_translation(vector![pos.x, pos.y, pos.z], true);
    

    }
    pub fn rotate_object(&mut self, handle: &RigidBodyHandle, rot: &Vec3) {
        use rapier3d::na::UnitQuaternion;
        let rb = self.rigidBS.get_mut(*handle).expect("Missing Rigid body handle!!??");
        let rotation = UnitQuaternion::from_euler_angles(rot.x, rot.y, rot.z);
        rb.set_rotation(rotation, true);
        
    }
    pub fn scale_object(&mut self, handle: &ColliderHandle, obj_type:  &obj::Object, scale: &Vec3){
        match obj_type{
            obj::Object::Cube(_) => {
                let c = self.coll.get_mut(*handle).expect("Missing collider despite holding handle.");
                let shape = SharedShape::cuboid(scale.x/2.0, scale.y/2.0, scale.z/2.0);
                c.set_shape(shape);
            }
            _ => todo!() // not important for the moment
        }
    }

    pub fn step(&mut self, distance: f32) {
        let gravity = vector![0.0, -9.81, 0.0];

        self.integration_parameters.dt = distance;
        self.pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.bvh,
            &mut self.narrowP,
            &mut self.rigidBS,
            &mut self.coll,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
    pub fn insert_object(&mut self, obj: &obj::Object, key: ObjectKey, collider: ColliderOptions) -> Option<ObjectHandle> {
        match collider.0{
            InnerColliderOptions::None => {
                None
            }
            InnerColliderOptions::Static=> {
                Some(self.static_collider_builder(obj, key))
            }
            InnerColliderOptions::Dynamic { .. } =>{
                Some(self.dynamic_collider_builder(obj,key,collider))
            }
        }
    }

    fn dynamic_collider_builder(&mut self, obj: &obj::Object, key: ObjectKey, options: ColliderOptions) -> ObjectHandle {

        let (gravity_scale, friction_val, restitution_val, density_val) = match options.0 {
            InnerColliderOptions::Dynamic { gravity_scale, friction, restitution, density } =>
            (gravity_scale,friction,restitution,density),
            _ => unreachable!()
        };

        let t: Transforms<'_> = extract_object_transforms(obj);

        // 2. Build the Collider (Shape)
        let mut collider = match obj {
            obj::Object::Cube(_) => {
                ColliderBuilder::cuboid(t.scale.x / 2.0, t.scale.y / 2.0, t.scale.z / 2.0)
                    .friction(friction_val)
                    .restitution(restitution_val)
                    .density(density_val)
                    .build()
            },
            obj::Object::Mesh(mesh_wrapper) => {
                 // ... (Mesh processing logic remains the same) ...
                let vertices: Vec<rapier3d::na::Point3<f32>> = mesh_wrapper.mesh.vertices
                    .iter()
                    .map(|v| {
                        rapier3d::na::Point3::new(
                            v.position.x * mesh_wrapper.scale.x,
                            v.position.y * mesh_wrapper.scale.y,
                            v.position.z * mesh_wrapper.scale.z,
                        )
                    })
                    .collect();

                let indices: Vec<[u32; 3]> = mesh_wrapper.mesh.indices
                    .chunks_exact(3)
                    .map(|chunk| [chunk[0] as u32, chunk[1] as u32, chunk[2] as u32])
                    .collect();

                ColliderBuilder::trimesh(vertices, indices)
                    .expect("Could not build dynamic mesh collider")
                    .friction(friction_val)
                    .restitution(restitution_val)
                    .density(density_val)
                    .build()
            },
            _ => todo!("Other object types not implemented for dynamic colliders"),
        };

        // 3. Build the RigidBody
        // CHANGE HERE: Added .user_data()
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![t.pos.x, t.pos.y, t.pos.z])
            .rotation(rapier3d::na::Vector3::new(t.rot.x, t.rot.y, t.rot.z))
            .gravity_scale(gravity_scale)
            .linear_damping(0.0) 
            .angular_damping(0.0)
            .user_data(key_to_u128(key))
            .build();

        // 4. Link User Data to Collider (You already had this)
        collider.user_data = key_to_u128(key);

        // 5. Insert into Sets
        let rigid_body_handle = self.rigidBS.insert(rigid_body);
        
        let collider_handle = self.coll.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigidBS
        );

        ObjectHandle { rigid_body_handle, collider_handle }
    }
    fn static_collider_builder(&mut self, obj: &obj::Object, key: ObjectKey)-> ObjectHandle{

        let t: Transforms<'_>  = extract_object_transforms(obj);

        let mut collider =  match obj{
            obj::Object::Cube(_) => {
                ColliderBuilder::cuboid(t.scale.x / 2.0, t.scale.y / 2.0, t.scale.z / 2.0)
                    .sensor(true)
                    .restitution(0.7)
                    .density(1.0)
                    .active_collision_types(
                        ActiveCollisionTypes::DYNAMIC_DYNAMIC
                        | ActiveCollisionTypes::DYNAMIC_KINEMATIC
                        | ActiveCollisionTypes::DYNAMIC_FIXED
                        | ActiveCollisionTypes::KINEMATIC_KINEMATIC 
                        | ActiveCollisionTypes::KINEMATIC_FIXED
                    )
                    .build()
            },
            obj::Object::Mesh(mesh_wrapper) => {
                // ... (Mesh processing logic remains the same) ...
                let vertices: Vec<rapier3d::na::Point3<f32>> = mesh_wrapper.mesh.vertices
                    .iter()
                    .map(|v| {
                        rapier3d::na::Point3::new(
                            v.position.x * mesh_wrapper.scale.x,
                            v.position.y * mesh_wrapper.scale.y,
                            v.position.z * mesh_wrapper.scale.z,
                        )
                    })
                    .collect();
            
                let indices: Vec<[u32; 3]> = mesh_wrapper.mesh.indices
                    .chunks_exact(3)
                    .map(|chunk| [chunk[0] as u32, chunk[1] as u32, chunk[2] as u32])
                    .collect();
            
                ColliderBuilder::trimesh(vertices, indices)
                    .expect("Could not build mesh collider from raw vertecies")
                    .sensor(true) 
                    .restitution(0.7)
                    .density(1.0)
                    .active_collision_types(
                        ActiveCollisionTypes::DYNAMIC_DYNAMIC
                        | ActiveCollisionTypes::DYNAMIC_KINEMATIC
                        | ActiveCollisionTypes::DYNAMIC_FIXED
                        | ActiveCollisionTypes::KINEMATIC_KINEMATIC 
                        | ActiveCollisionTypes::KINEMATIC_FIXED
                    )
                    .build()
            },
            _ => todo!()
        };

        // CHANGE HERE: Added .user_data()
        let rigid_body = RigidBodyBuilder::kinematic_position_based()
            .translation(vector![t.pos.x,t.pos.y,t.pos.z])
            .rotation(rapier3d::na::Vector3::new(t.rot.x, t.rot.y, t.rot.z))
            .user_data(key_to_u128(key)) // <--- ADDED THIS
            .build();
        
        // Link User Data to Collider (You already had this)
        collider.user_data = key_to_u128(key);
        
        let rigid_body_handle = self.rigidBS.insert(rigid_body);
        
        let collider_handle = self.coll.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigidBS
        );
    
        ObjectHandle { rigid_body_handle, collider_handle }
    }
    
    pub fn remove_object(&mut self, handle: ObjectHandle) {
        self.coll.remove(
            handle.collider_handle, 
            &mut self.islands,
            &mut self.rigidBS, 
            true
        );
        self.rigidBS.remove(
            handle.rigid_body_handle, 
            &mut self.islands, 
            &mut self.coll, 
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            true
        );
    }

    pub fn has_collision(&mut self, collider_handle: ColliderHandle) -> bool {
        
        let has_contacts = self.narrowP
            .contact_pairs_with(collider_handle)
            .next() 
            .is_some();
        
        let has_intersections = self.narrowP
            .intersection_pairs_with(collider_handle)
            .next()
            .is_some();
        
        has_contacts || has_intersections
    }


    pub fn get_collided_keys(&self, handle: ColliderHandle) -> Vec<ObjectKey> {
    
        let pairs =  self.narrowP.contact_pairs_with(handle).filter_map(|collider|{
            if collider.has_any_active_contact{
                let other_handle = if collider.collider1 == handle { collider.collider2 } else { collider.collider1 };
                
                let h = self.coll.get(other_handle).expect("Collider handle was found, but no matching collider.");
                Some ( u128_to_key(h.user_data) )
            }
            else{ None }
        });

        let pairs2 =  self.narrowP.intersection_pairs_with(handle).filter_map(|(h1,h2,intersecting)|{
            if intersecting{
                let other_handle = if h1 == handle { h2 } else { h1 };

                let h = self.coll.get(other_handle).expect("Collider handle was found, but no matching collider.");
                Some (u128_to_key(h.user_data))
            }
            else{ None }
        });

        pairs.chain(pairs2).collect()
    }


    
    
}



pub fn u128_to_key(val: u128) -> ObjectKey {
    let as_u64 = val as u64;
    KeyData::from_ffi(as_u64).into()
}

pub fn key_to_u128(key: ObjectKey) -> u128 {
    let data = key.data();
    let as_u64 = data.as_ffi();
    as_u64 as u128
}

pub fn extract_object_transforms(obj: &obj::Object)-> Transforms<'_>{
    match obj{
        obj::Object::Cube(cube) => Transforms { pos: &cube.position, rot: &cube.rotation, scale: &cube.scale },
        obj::Object::Mesh(mesh) => Transforms { pos: &mesh.position, rot: &mesh.rotation, scale: &mesh.scale},
        obj::Object::Sphere(sphere)=> Transforms { pos: &sphere.position, rot: &sphere.rotation, scale: &sphere.rotation },
        obj::Object::Pill(o)=> Transforms { pos: &o.position, rot: &o.rotation, scale: &o.rotation },
        obj::Object::Cylinder(o)=> Transforms { pos: &o.position, rot: &o.rotation, scale: &o.rotation }
    }
}