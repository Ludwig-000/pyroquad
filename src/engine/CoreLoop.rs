use std::panic;

use std::sync::Arc;
use std::any::Any;
use std::time::Instant;
use macroquad::color::BLACK;
use lazy_static::*;

use crossbeam::queue::SegQueue;

use macroquad::prelude as mq;
use macroquad::audio as au;
use macroquad::window::get_internal_gl;
use crate::engine::Objects::Cylinder::Cylinder;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;
use crate::engine::Objects::Mesh::Mesh;
use crate::engine::Objects::PhysicsWorld::ApplyPhysics;
use crate::engine::Objects::Pill::Pill;
use crate::engine::Objects::Sphere::Sphere;
use crate::engine::Objects::TwoDObjects::draw_circle;
use crate::engine::Objects::TwoDObjects::draw_rect;
use crate::engine::SHADERS::shader_manager as sm;
use crate::engine::PError::PError;
use crate::engine::PArc::PArc;
use crate::engine::Objects::ObjectManagement::ObjectStorage::*;
use crate::py_abstractions::GL::GLENUM;
use crate::py_abstractions::GL::implement_GlEnum;
use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::ColliderOptions;
use crate::py_abstractions::structs::ThreeDObjects::PhysicsHandle::PhysicsEnum;
use crate::py_abstractions::structs::TwoDObjects::Circle::Circle;
use crate::py_abstractions::structs::TwoDObjects::Rectangle::Rectangle;
use pyo3::{Py};
use pyo3::types::{PyWeakref};
use crate::engine::PChannel;
use crate::engine::Objects::Cube::*;
use crate::engine::Objects::ObjectManagement::ObjectStorage;
use crate::engine::Objects::ObjectManagement::ObjectManagement;


pub enum Command {
    GLENUM(GLENUM),
    DrawRectangleFromPyClass(Rectangle),
    DrawCircleFromPyClass(Circle),
    PhysicsEnum(PhysicsEnum, ObjectKey),
    ManuallyStepPhysics(f32),
    SetCollisionForObject{key: ObjectKey, collider: ColliderOptions},
    GetColissionObjects{
        key: ObjectKey, sender: PChannel::PSyncSender<Vec<Arc<Py<PyWeakref>>>>,
    },
    DrawAll3DObjects(),

    DeleteObject{
        key: ObjectKey, 
    },
    GetObjectScale{ key: ObjectKey, sender: PChannel::PSyncSender<mq::Vec3> },
    GetObjectPos{ key: ObjectKey, sender: PChannel::PSyncSender<mq::Vec3> },
    GetObjectRotation{ key: ObjectKey, sender: PChannel::PSyncSender<mq::Vec3> },

    SetObjectScale{ key: ObjectKey, scale: mq::Vec3 },
    SetObjectPos{ key: ObjectKey, position: mq::Vec3 },
    SetObjectRotation{ key: ObjectKey, rotation: mq::Vec3 },

    CreateCube{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        texture: Option<mq::Texture2D>,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSyncSender<ObjectKey>,
    },
    CreateSphere{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        texture: Option<mq::Texture2D>,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSyncSender<ObjectKey>,
    },
    CreatePill{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        texture: Option<mq::Texture2D>,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSyncSender<ObjectKey>,
    },
    CreateCylinder{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        texture: Option<mq::Texture2D>,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSyncSender<ObjectKey>,
    },

    CreateMesh{
        mesh: Mesh,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSyncSender<ObjectKey>,
    },

    DropThisItem(Arc<dyn Any + Send + Sync>), // drops it's item.  >_<

    LoadFile{ path: String, sender: PChannel::PSyncSender<Result<Vec<u8>, PError>> },
    LoadSound{ path: String, sender: PChannel::PSyncSender<Result<PArc<au::Sound>, PError>> },
    
    LoadSoundFromBytes{data: Vec<u8>, sender: PChannel::PSyncSender<Result<PArc<au::Sound>, PError>> },

    PlaySound{ sound: au::Sound, params: au::PlaySoundParams },

    PlaySoundOnce{ sound: au::Sound },

    SetSoundVolume{ sound: au::Sound, volume: f32 },

    StopSound{ sound: au::Sound },

    RenderTargetMsaa{ width: u32, height: u32, sender: PChannel::PSyncSender< PArc<mq::RenderTarget>  > },
    RenderTargetEx{ width: u32, height: u32, params: Option<mq::RenderTargetParams>, sender: PChannel::PSyncSender<PArc<mq::RenderTarget> > },
    DrawArc{ x: f32,
        y: f32,
        sides: u8,
        radius: f32,
        rotation: f32,
        thickness: f32,
        arc: f32,
        color: mq::Color,},
    DrawCubeWires{position: mq::Vec3, size: mq::Vec3, color: mq::Color},
    DrawCylinder{ position: mq::Vec3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        texture: Option<mq::Texture2D>,
        color: mq::Color,},
    DrawCylinderWires{ position: mq::Vec3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        texture: Option<mq::Texture2D>,
        color: mq::Color,},

    DrawEllipse{x: f32, y: f32, w: f32, h: f32, rotation: f32, color: mq::Color},
    DrawEllipseLines{x: f32,
            y: f32,
            w: f32,
            h: f32,
            rotation: f32,
            thickness: f32,
            color: mq::Color,},
    DrawHexagon{x: f32,
                y: f32,
                size: f32,
                border: f32,
                vertical: bool,
                border_color: mq::Color,
                fill_color: mq::Color,},
    DrawLine3D{start: mq::Vec3, end: mq::Vec3, color: mq::Color},


    DrawAfflineParallelpiped{offset: mq::Vec3, e1: mq::Vec3,e2: mq::Vec3,e3: mq::Vec3,texture: Option<mq::Texture2D>,color: mq::Color},
    SetDefaultCamera(),

    DrawRect { x: f32, y: f32, w: f32, h: f32, color: mq::Color },

    DrawPlane { center: mq::Vec3, size: mq::Vec2, color: mq::Color, texture: Option<mq::Texture2D> },

    DrawGrid { slices: u32, spacing: f32, axes_color: mq::Color, other_color: mq::Color },

    DrawCube { pos: mq::Vec3, size: mq::Vec3, texture: Option<mq::Texture2D>, color: mq::Color},

    DrawCubemap { pos: mq::Vec3, size: mq::Vec3, texture: Option<mq::Texture2D>, color: mq::Color},

    DrawPoly{ x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: mq::Color},

    DrawText{text: String, x: f32, y: f32, font_size: f32, color: mq::Color},

    DrawTexture{ texture: mq::Texture2D, x: f32, y: f32, color: mq::Color   },
    
    ClearBackground { color: mq::Color },


    NextFrame{physics_step: Option<f32>, sender: PChannel::PSyncSender<()>},

    ImgToTexture {
        image: Arc<mq::Image>,
        sender: PChannel::PSyncSender<PArc<mq::Texture2D>>,
    },

    LoadImage {
        path: String,
        sender: PChannel::PSyncSender<Result<mq::Image, PError>>,
    },
    GetScreenData {
        sender: PChannel::PSyncSender<mq::Image>,
    },

    SetCamera{camera_2d: Option<mq::Camera2D>, camera_3d: Option<mq::Camera3D>},

    SetCursorGrab ( bool ),

    ShowMouse(bool),
    
}

lazy_static! {
    pub static ref COMMAND_QUEUE: Arc<SegQueue<Command>> = Arc::new(SegQueue::new());
    
}


/// processes commands that rely on the macroquad engine
/// commands that do not rely on it's core (openGL) components ( or just the internal Core-Thread ) are found in pyabstractions.
pub async fn proccess_commands_loop() {

    let mut object_storage = ObjectStorage::ObjectStorage::new();

    loop {
        while let Some(command) = COMMAND_QUEUE.pop() {
            
            match command {
                Command::GLENUM(glenum)=> {
                    let gl  = unsafe {
                        get_internal_gl().quad_gl
                    };
                    implement_GlEnum(glenum, gl);
                }
                Command::DrawCircleFromPyClass(circle)=> {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    draw_circle(&circle)},
                Command::DrawRectangleFromPyClass(rect)=> {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    draw_rect(&rect)},
                Command::PhysicsEnum(phys,key )=> {
                    let handle = object_storage.get_handle(key).expect("No physics handle found, yet physics function was called.");
                    object_storage.apply_physics_enum(phys, &handle);
                }
                Command::SetCollisionForObject{key, collider}=> {
                    object_storage.set_collision_for_object(key, collider);
                }
                Command::GetColissionObjects { key, sender }=>{
                    let keys = object_storage.collides_with(key);
                    let py_refs  = object_storage.keys_to_py(keys);
                    let _ = sender.send(py_refs);
                }
                Command::ManuallyStepPhysics(distance)=>{
                    object_storage.step_physics(distance);
                }
                Command::DrawAll3DObjects()=> {
                    let matrix;
                    unsafe {
                        let mat = mq::get_internal_gl();
                        matrix= mat.quad_gl.get_projection_matrix()
                    }
                    sm::switch_to_desired_shader(sm::ShaderKind::Basic);
                    ObjectManagement::draw_all_Objects(&object_storage, matrix);
                }
                Command::DeleteObject { key }=> {
                    object_storage.remove_object(key);
                }
                Command::GetObjectPos { key, sender } => {
                    let pos = match  object_storage.get(key){
                        Object::Cube(cube) => cube.position,
                        Object::Mesh(mesh) => mesh.position,
                        Object::Sphere(sphere)=> sphere.position,
                        Object::Cylinder(cyl)=> cyl.position,
                        Object::Pill(pill)=> pill.position,
                    };
                    let _ = sender.send(pos);
                }
                Command::GetObjectScale { key, sender } => {
                    let pos = match  object_storage.get(key){
                        Object::Cube(cube) => cube.scale,
                        Object::Mesh(mesh)=> mesh.scale,
                        Object::Sphere(sphere)=> sphere.scale,
                        Object::Cylinder(cyl)=> cyl.scale,
                        Object::Pill(pill)=> pill.scale,
                    };
                    let _ = sender.send(pos);
                }
                Command::GetObjectRotation { key, sender } => {
                    let pos = match  object_storage.get(key){
                        Object::Cube(cube) => cube.rotation,
                        Object::Mesh(mesh)=> mesh.rotation,
                        Object::Sphere(sphere)=> sphere.rotation,
                        Object::Cylinder(cyl)=> cyl.rotation,
                        Object::Pill(pill)=> pill.rotation,
                    };
                    let _ = sender.send(pos);
                }
                Command::SetObjectPos { key, position } => {
                    object_storage.change_obj_position(&position, key, 
                        move |obj|{
                        match obj{
                            Object::Cube(cube)=> {
                                cube.mesh.recalculate_pos(cube.position, position);
                                cube.position = position;
                            }
                            Object::Mesh(mesh) => {
                                mesh.recalculate_pos(mesh.position, position);
                                mesh.position =  position;
                            }
                            Object::Sphere(sphere)=>{
                                sphere.mesh.recalculate_pos(sphere.position, position);
                                sphere.position = position;
                            }
                            Object::Pill(pill)=> {
                                pill.mesh.recalculate_pos(pill.position, position);
                                pill.position = position;
                            }
                            Object::Cylinder(cyl)=> {
                                cyl.mesh.recalculate_pos(cyl.position, position);
                                cyl.position = position;
                            }
                        }
                    });
                }
                Command::SetObjectScale { key, scale } => {

                    object_storage.change_obj_scale(&scale, key, 
                        move |obj|{
                            match obj{
                                Object::Cube(cube)=> {
                                    cube.mesh.recalculate_scale(cube.position, cube.scale, scale);
                                    cube.scale = scale;
                                }
                                Object::Mesh(mesh) => {
                                    mesh.recalculate_scale(mesh.position,mesh.scale, scale);
                                    mesh.scale =  scale;
                                }
                                Object::Sphere(sphere)=>{
                                    sphere.mesh.recalculate_scale(sphere.position, sphere.scale, scale);
                                    sphere.scale = scale;
                                }
                                Object::Pill(pill)=> {
                                    pill.mesh.recalculate_scale(pill.position, pill.scale, scale);
                                    pill.scale = scale;
                                }
                                Object::Cylinder(cyl)=> {
                                    cyl.mesh.recalculate_scale(cyl.position, cyl.scale, scale);
                                    cyl.scale = scale;
                                }
                            }
                        });
                }
                Command::SetObjectRotation { key, rotation } => {
                    object_storage.change_obj_rotation(&rotation, key, 
                        move |obj|{
                            match obj{
                                Object::Cube(cube)=> {
                                    cube.mesh.recalculate_rot(cube.position, cube.rotation, rotation);
                                    cube.rotation = rotation;
                                }
                                Object::Mesh(mesh) => {
                                    mesh.recalculate_rot(mesh.position, mesh.rotation, rotation);
                                    mesh.rotation =  rotation;
                                }
                                Object::Sphere(sphere)=>{
                                    sphere.mesh.recalculate_rot(sphere.position, sphere.rotation, rotation);
                                    sphere.rotation = rotation;
                                }
                                Object::Pill(pill)=>{
                                    pill.mesh.recalculate_rot(pill.position, pill.rotation, rotation);
                                    pill.rotation =rotation;
                                }
                                Object::Cylinder(cyl)=>{
                                    cyl.mesh.recalculate_rot(cyl.position, cyl.rotation, rotation);
                                    cyl.rotation =rotation;
                                }
                            }
                        });
                    
                }
                Command::CreatePill { size, position, rotation, color,texture, collider, weak_ref, sender }=>{
                    object_storage.quick_push(collider,sender, weak_ref, 
                        move || {
                            let internal_pill = Pill::new(size, position, rotation, color,texture,);
                            Object::Pill(internal_pill)
                        });
                }
                Command::CreateCylinder { size, position, rotation, color,texture, collider, weak_ref, sender }=>{
                    object_storage.quick_push(collider,sender, weak_ref, 
                        move || {
                            let internal_cyl = Cylinder::new(size, position, rotation, color,texture);
                            Object::Cylinder(internal_cyl)
                        });
                }
                Command::CreateCube { size, position, rotation,color,texture,collider, weak_ref: pyAny, sender }=>{

                    object_storage.quick_push(collider,sender, pyAny, 
                        move || {
                            let internal_cube = Cube::new(size, position, rotation, color,texture);
                            Object::Cube(internal_cube)
                        });
                        
                }
                Command::CreateMesh { mesh,collider, weak_ref, sender }=>{

                    object_storage.quick_push(collider,sender, weak_ref, 
                        move || {
                            Object::Mesh(mesh)
                        });
                }
                Command::CreateSphere { size, position, rotation,color,texture, collider,weak_ref: pyAny, sender }=>{

                    object_storage.quick_push(collider,sender, pyAny, 
                        move || {
                            let internal_sphere = Sphere::new(size, position, rotation, color, texture);
                            Object::Sphere(internal_sphere)
                        });
                        
                }
                Command::DrawRect { x, y, w, h, color} => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    mq::draw_rectangle(
                        x,
                        y,
                        w,
                        h,
                        color,
                    );
                }
                Command::DrawPlane { center, size, color, texture } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    let tex_ref = texture.as_ref();
                    mq::draw_plane(center,size,tex_ref,color);
                }
                Command::DrawGrid { slices, spacing, axes_color, other_color } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    mq::draw_grid(slices, spacing, axes_color, other_color);
                }
                Command::DrawCube {pos,size,texture,color} => {
                    sm::switch_to_desired_shader(sm::ShaderKind::Basic);
                    mq::draw_cube(pos,size,texture.as_ref(),color)
                }
                Command::DrawCubemap {pos,size,texture,color} => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    mq::draw_cube(pos,size,texture.as_ref(),color);
                    crate::engine::Cubemap::draw_fullscreen_quad();
        
                }
        
                Command::DrawAfflineParallelpiped { offset, e1, e2, e3, texture, color } => {
                    mq::draw_affine_parallelepiped(offset, e1, e2, e3, texture.as_ref(), color);
                }
                Command::DrawArc { x, y, sides, radius, rotation, thickness, arc, color } => {
                    mq::draw_arc(x, y, sides, radius, rotation, thickness, arc, color);
                }
        
                Command::DrawCubeWires { position, size, color } => {
                    mq::draw_cube_wires(position, size, color);
                }
                Command::DrawCylinder { position, radius_top, radius_bottom, height, texture, color } => {
                    mq::draw_cylinder(position, radius_top, radius_bottom, height, texture.as_ref(), color);
                }
        
                Command::DrawCylinderWires { position, radius_top, radius_bottom, height, texture, color } => {
                    mq::draw_cylinder_wires(position, radius_top, radius_bottom, height, texture.as_ref(), color);
                }
        
                Command::DrawEllipse { x, y, w, h, rotation, color }    => {
                    mq::draw_ellipse(x, y, w, h, rotation, color);  
                }
                
                Command::DrawEllipseLines { x, y, w, h, rotation, thickness, color }    => {
                    mq::draw_ellipse_lines(x, y, w, h, rotation, thickness, color);
                }
        
                Command::DrawHexagon { x, y, size, border, vertical, border_color, fill_color } => {
                    mq::draw_hexagon(x, y, size, border, vertical, border_color, fill_color);
                }
        
                Command::DrawLine3D { start, end, color } =>{
                    mq::draw_line_3d(start, end, color);    
                }
                
        
        
                Command::LoadImage { path,sender} => {
                    let result = async {
                    let bytes = mq::load_file(&path).await?;
                    
                    let image = mq::Image::from_file_with_format(&bytes, None)?;
                    Ok(image)
                }.await;
        
                    let _ = sender.send(result);
                }
                Command::GetScreenData { sender }=>{
                    let res = mq::get_screen_data();
                    let _ = sender.send(res);
                }
                Command::DrawPoly { x, y, sides, radius, rotation, color }=>
                {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    mq::draw_poly(x,y,sides,radius,rotation,color  );
                }
                Command::SetDefaultCamera() =>{ 
                    mq::set_default_camera() 
                }
        
                Command::DrawTexture { texture,x,y,color}=>
                {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    mq::draw_texture(&texture,x,y,color)
                }
                
                Command::ClearBackground {color } => {
                    macroquad::prelude::clear_background(color);
                }
        
                Command::NextFrame{physics_step, sender} => {
                    let start=  Instant::now();
                    mq::next_frame().await;
                    crate::engine::SHADERS::shader_manager::new_frame_shader_update();
                    crate::engine::FrameInfo::update_frame_info();

                    let _ = sender.send(());
                    
                    if physics_step.is_some(){
                        object_storage.step_physics(physics_step.unwrap());
                    }


                    mq::clear_background(BLACK); // 3d rendering is bugged if we don't clear.
                }
            
                Command::DrawText { text, x, y, font_size, color }=>{
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                mq::draw_text(text.as_str(), x,y,font_size,color);
                }
                
                Command::ImgToTexture { image, sender }=>{
                    let tex: PArc<mq::Texture2D> = PArc::new(  mq::Texture2D::from_image(&image));
                    let _ = sender.send(tex);
                }
                Command::SetCamera { camera_2d, camera_3d } => { // merged cam2d and 3d for simplicity.
                    match (camera_2d, camera_3d) {
                        (Some(cam), None) => {
                            mq::set_camera(&cam);
                        },
                        (None, Some(cam)) => {
                            mq::set_camera(&cam);
                        },
                        _ => panic!("invalid cam pattern"),
        
                    }
                }
                Command::SetCursorGrab(i) =>{
                    mq::set_cursor_grab(i);
                }
                Command::ShowMouse(i) =>{
                    mq::show_mouse(i);
                }
                Command::RenderTargetMsaa{ width, height, sender } => {
                    let render_target = mq::render_target_msaa(width, height);
                    let _ = sender.send(PArc::new(render_target));
                }
                Command::RenderTargetEx{ width, height, params, sender } => {
                    match params{
                        Some(p) => {
                            let render_target = mq::render_target_ex(width, height, p);
                            let _ = sender.send(PArc::new(render_target));
                        }
                        None => {
                            let render_target = mq::render_target_ex(width, height, mq::RenderTargetParams::default());
                        let _ = sender.send(PArc::new(render_target));
                        }
                    }
        
                }
                Command::LoadSound { path ,sender} => {
                    let result = async {
                        let data = macroquad::prelude::load_file(&path).await?;
        
                        //converts .mp3 to .wav
                        let secured_data  = crate::engine::AudioConverter::ensure_wav(data).map_err(|e| {
                        e.with_context(format!(" path: {path}"))
                        }  )?;
        
                        let sound = au::load_sound_from_bytes(&secured_data).await?;
                        Ok(sound)
                    }.await;
                    let result = result.map(|op| PArc::new(op)  );
                
                    let _ = sender.send(result);
                }
                Command::LoadSoundFromBytes { data ,sender} => {
        
                    let result: Result<_, PError> = async {
                        //converts .mp3 to .wav
                        let secured_data  = crate::engine::AudioConverter::ensure_wav(data)?;
        
                        let sound = au::load_sound_from_bytes(&secured_data).await?;
                        Ok(sound)
                    }.await;
                
                    let result = result.map(|op| PArc::new(op)  );
                    let _ = sender.send(result);
                }
                
                Command::PlaySound { sound, params } => {
                    au::play_sound(&sound, params);
                }
                Command::PlaySoundOnce { sound } => {
                    au::play_sound_once(&sound);
                }
                Command::StopSound { sound } => {
                    au::stop_sound(&sound);
                }
                Command::SetSoundVolume { sound, volume  } => {
                    au::set_sound_volume(&sound, volume);
                }
        
                Command::LoadFile { path ,sender} => {
        
                    let result = mq::load_file(&path).await.map_err(Into::into);
                    let _ = sender.send(result);
        
                }
        
                Command::DropThisItem(item_arc)=>{}
        
                
            }
        }


    }
}