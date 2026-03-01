use crate::engine::Objects::ObjectManagement::{ ObjectStorage::ObjectStorage};
use crate::engine::Objects::ObjectManagement::ObjectStorage::*;
use macroquad::prelude as mq;

/// idk make this unsafe threaded at some point.
/// this function takes a considerable ammount of time to run, when working with many objects.
pub fn sync_transforms_inner(storage : &mut ObjectStorage) {
    use crate::engine::Objects::PhysicsWorld::Rapier::*;
    let objects_ptr = storage as *mut ObjectStorage;



    for (_, rb) in storage.physics_world.rigidBS.iter() {
        if !rb.is_dynamic() || rb.is_sleeping() { continue; }
        
        let key = u128_to_key(rb.user_data);

        let r_pos = rb.translation();
        let pos = mq::Vec3::new(r_pos.x, r_pos.y, r_pos.z);
        let (rx, ry, rz) = rb.rotation().euler_angles();
        let rot = mq::Vec3::new(rx, ry, rz);

        unsafe {
            let item  = (*objects_ptr).get_mut(key);
            match item{
                Object::Cube(cube)=> {
                    if cube.rotation != rot {
                        cube.mesh.recalculate_rot(cube.position, cube.rotation, rot);
                        cube.rotation = rot;
                    }
                    if cube.position != pos {
                        cube.mesh.recalculate_pos(cube.position, pos);
                        cube.position = pos;
                    }

                }
                Object::Sphere(sphere)=>{
                    if sphere.rotation != rot {
                        sphere.mesh.recalculate_rot(sphere.position, sphere.rotation, rot);
                        sphere.rotation = rot;
                    }
                    if sphere.position != pos {
                        sphere.mesh.recalculate_pos(sphere.position, pos);
                        sphere.position = pos;
                    }
                }
                Object::Mesh(mesh)=> {
                    if mesh.rotation != rot {
                        mesh.recalculate_rot(mesh.position, mesh.rotation, rot);
                        mesh.rotation = rot;
                    }
                    if mesh.position != pos {
                        mesh.recalculate_pos(mesh.position, pos);
                        mesh.position = pos;
                    }
                }
                Object::Pill(pill)=> {
                    if pill.rotation != rot {
                        pill.mesh.recalculate_rot(pill.position, pill.rotation, rot);
                        pill.rotation = rot;
                    }
                    if pill.position != pos {
                        pill.mesh.recalculate_pos(pill.position, pos);
                        pill.position = pos;
                    }
                }
                Object::Cylinder(cylinder)=> {
                    if cylinder.rotation != rot {
                        cylinder.mesh.recalculate_rot(cylinder.position, cylinder.rotation, rot);
                        cylinder.rotation = rot;
                    }
                    if cylinder.position != pos {
                        cylinder.mesh.recalculate_pos(cylinder.position, pos);
                        cylinder.position = pos;
                    }
                }
            }
            
        }
    }
}