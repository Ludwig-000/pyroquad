use std::panic;

use std::sync::Arc;
use std::any::Any;
use std::sync::LazyLock;
use macroquad::color::BLACK;

use crossbeam::queue::SegQueue;

use macroquad::input::Touch;
use macroquad::miniquad;
use macroquad::models::DrawSphereParams;
use macroquad::prelude as mq;
use macroquad::audio as au;
use macroquad::text::TextDimensions;
use macroquad::window::get_internal_gl;
use crate::engine::CameraManager;
use crate::engine::CameraManager::CamMemory;
use crate::engine::CameraManager::Camera;
use crate::engine::CameraManager::clone_camera3d;
use crate::engine::CameraManager::set_camera;
use crate::engine::CameraManager::set_default_camera;
use crate::engine::MouseInsideScreen;
use crate::engine::Objects::Cylinder::Cylinder;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;
use crate::engine::Objects::Mesh::Mesh;
use crate::engine::Objects::Pill::Pill;
use crate::engine::Objects::Sphere::Sphere;
use crate::engine::Objects::TwoDObjects::draw_circle;
use crate::engine::Objects::TwoDObjects::draw_rect;
use crate::engine::PChannel::PSender;
use crate::engine::SHADERS::shader_manager as sm;
use crate::engine::PError::PError;
use crate::engine::PArc::PArc;
use crate::engine::Objects::ObjectManagement::ObjectStorage::*;
use crate::py_abstractions::GL::GlEnum;
use crate::py_abstractions::GL::implement_GlEnum;
use crate::py_abstractions::Textures_and_Images::EngineTexImgEnum;
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
    CreateMeshFromBytes{
        data: Vec<u8>,
        texture: Option<mq::Texture2D>,
        sender: PChannel::PSender<Result<Mesh, PError>>,
    },
    GetCustomMouseState{
        sender: PSender<(f32,f32,bool)>,
    },
    Camera2DWorldToScreen{cam: mq::Camera2D, point: mq::Vec2, sender: PSender<mq::Vec2>},
    Camera2DScreenToWorld{cam: mq::Camera2D, point: mq::Vec2, sender: PSender<mq::Vec2>},
    Camera2DToMatrix{cam: mq::Camera2D, sender: PSender<mq::Mat4>},
    MeasureText{text: String, font: Option<mq::Font>, font_size: u16, font_scale: f32, sender: PSender<TextDimensions>},
    BuildTextureAtlas,
    SetPcAssetFolder(String),
    Touches(PSender<Vec<Touch>>),
    TouchesLocal(PSender<Vec<Touch>>),
    SimulateMouseWithTouch(bool),
    LoadTTFFOnt{
        path: String,
        sender: PSender<Result<mq::Font, PError>>
    },
    PopulateFontCache{
        font: mq::Font, characters: Vec<char>, size: u16
    },
    SetFontFilter{
        font: mq::Font,
        filter: mq::FilterMode
    },
    LoadTTFFontFromBytes{
        bytes: Vec<u8>,
        sender: PSender<Result<mq::Font, PError>>
    },
    RequestNewScreenSize{
        width: f32,
        height: f32
    },
    IsQuitRequested(PSender<bool>),
    PreventQuit,
    SetFullscreen(bool),

    TexImEnum(EngineTexImgEnum),
    GlEnum(GlEnum),
    DrawRectangleFromPyClass(Rectangle),
    DrawCircleFromPyClass(Circle),
    PhysicsEnum(PhysicsEnum, ObjectKey),
    ManuallyStepPhysics(f32),
    SetCollisionForObject{key: ObjectKey, collider: ColliderOptions},
    GetColissionObjects{
        key: ObjectKey, sender: PChannel::PSender<Vec<Arc<Py<PyWeakref>>>>,
    },
    DrawAll3DObjects(),
    DrawObjectNow(ObjectKey),
    DoesObjectCollide{
        key: ObjectKey,
        sender: PSender<bool>,
    },
    SetDrawEachFrame{
        key: ObjectKey,
        set: bool,
    },
    DeleteObject{
        key: ObjectKey, 
    },
    GetObjectScale{ key: ObjectKey, sender: PChannel::PSender<mq::Vec3> },
    GetObjectPos{ key: ObjectKey, sender: PChannel::PSender<mq::Vec3> },
    GetObjectRotation{ key: ObjectKey, sender: PChannel::PSender<mq::Vec3> },

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
        sender: PChannel::PSender<ObjectKey>,
    },
    CreateSphere{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        texture: Option<mq::Texture2D>,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSender<ObjectKey>,
    },
    CreatePill{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        texture: Option<mq::Texture2D>,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSender<ObjectKey>,
    },
    CreateCylinder{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        texture: Option<mq::Texture2D>,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSender<ObjectKey>,
    },

    CreateMesh{
        mesh: Mesh,
        collider: ColliderOptions,
        weak_ref: Py<PyWeakref>,
        sender: PChannel::PSender<ObjectKey>,
    },

    DropThisItem(Arc<dyn Any + Send + Sync>), // drops it's item.  >_<

    LoadFile{ path: String, sender: PChannel::PSender<Result<Vec<u8>, PError>> },
    LoadSound{ path: String, sender: PChannel::PSender<Result<PArc<au::Sound>, PError>> },
    
    LoadSoundFromBytes{data: Vec<u8>, sender: PChannel::PSender<Result<PArc<au::Sound>, PError>> },

    PlaySound{ sound: au::Sound, params: au::PlaySoundParams },

    PlaySoundOnce{ sound: au::Sound },

    SetSoundVolume{ sound: au::Sound, volume: f32 },

    StopSound{ sound: au::Sound },

    RenderTargetMsaa{ width: u32, height: u32, sender: PChannel::PSender< PArc<mq::RenderTarget>  > },
    RenderTargetEx{ width: u32, height: u32, params: Option<mq::RenderTargetParams>, sender: PChannel::PSender<PArc<mq::RenderTarget> > },
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
    DrawLine{x1: f32,y1: f32,x2: f32,y2: f32,thickness: f32,color: mq::Color},


    DrawAfflineParallelpiped{offset: mq::Vec3, e1: mq::Vec3,e2: mq::Vec3,e3: mq::Vec3,texture: Option<mq::Texture2D>,color: mq::Color},
    DrawAfflineParallogram{offset: mq::Vec3,
        e1: mq::Vec3,
        e2: mq::Vec3,
        texture: Option<mq::Texture2D>,
        color: mq::Color},
    SetDefaultCamera(),

    DrawRect { x: f32, y: f32, w: f32, h: f32, color: mq::Color },
    DrawRectLines { x: f32, y: f32, w: f32, h: f32,thickness: f32, color: mq::Color },

    DrawTriangle { v1: mq::Vec2, v2: mq::Vec2, v3: mq::Vec2, color: mq::Color },
    DrawTriangleLines {v1: mq::Vec2,v2: mq::Vec2,v3: mq::Vec2, thickness: f32, color: mq::Color },


    DrawPlane { center: mq::Vec3, size: mq::Vec2, color: mq::Color, texture: Option<mq::Texture2D> },

    DrawGrid { slices: u32, spacing: f32, axes_color: mq::Color, other_color: mq::Color, center: mq::Vec3, rotation: mq::Quat },

    DrawCube { pos: mq::Vec3, size: mq::Vec3, texture: Option<mq::Texture2D>, color: mq::Color},

    DrawSkyBox {texture: Option<mq::Texture2D>, tint: mq::Color},

    DrawPoly{ x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: mq::Color},
    DrawPolyLines{ x: f32,y: f32,sides: u8,radius: f32,rotation: f32,thickness: f32,color: mq::Color},

    DrawText{
        text: String, 
        x: f32, 
        y: f32, 
        color: mq::Color, 
        font: Option<mq::Font>,
        font_size: u16, 
        font_scale: f32,
        font_scale_aspect: f32,
        rotation: f32,
        sender: PSender<mq::TextDimensions>},
    GetTextCenter{text: String,
        font: Option<mq::Font>,
        font_size: u16,
        font_scale: f32,
        rotation: f32,
        sender: PSender<mq::Vec2>
    },
    DrawMultilineText{text: String,
        x: f32,
        y: f32,
        font_size: u16,
        line_distance_factor: Option<f32>,
        color: mq::Color,
        font_scale:  f32,
        font_scale_aspect: f32,
        rotation: f32,
        font: Option<mq::Font>,
    },

    DrawTexture{ texture: mq::Texture2D, x: f32, y: f32, color: mq::Color   },
    
    ClearBackground { color: mq::Color },
    ScreenDpiScale(PSender<f32>),

    NextFrame{physics_step: Option<f32>, sender: PChannel::PSender<()>},

    LoadImage {
        path: String,
        sender: PChannel::PSender<Result<mq::Image, PError>>,
    },
    GetScreenData {
        sender: PChannel::PSender<mq::Image>,
    },

    SetCamera{camera_2d: Box<Option<mq::Camera2D>>, camera_3d: Box<Option<mq::Camera3D>>},

    SetCursorGrab ( bool ),

    ShowMouse(bool),
    ClearInputQueue,
    IsSimulatingMouseWithTouch(PSender<bool>),
    PushCameraState,
    PopCameraState,
    CameraFontScale{
        world_font_size: f32,
        sender: PSender<(u16, f32, f32)>,
    },
}

/// The engine's inbox.
///
/// Natively this is a plain queue that the engine thread drains. On Emscripten
/// there is no engine thread, so `push` also decides *when* a command runs:
/// frame-local commands wait for the next frame, everything else is executed
/// immediately, on the caller's stack. See [`crate::web`].
pub struct CommandQueue {
    inner: SegQueue<Command>,
}

impl CommandQueue {
    fn new() -> Self {
        Self { inner: SegQueue::new() }
    }

    pub fn push(&self, command: Command) {
        #[cfg(target_os = "emscripten")]
        if !command.is_frame_local()
            && crate::py_abstractions::py_functions::ENGINE_CURRENTLY_ACTIVE
                .load(std::sync::atomic::Ordering::Relaxed)
        {
            // Only once the engine exists - before that, dispatching would call
            // into macroquad without a context. Leaving the command in the queue
            // instead lets `PReceiver::recv` report the usual "you forgot
            // activate_engine()" error.
            crate::web::run_query_now(command);
            return;
        }

        self.inner.push(command);
    }

    pub fn pop(&self) -> Option<Command> {
        self.inner.pop()
    }
}

pub static COMMAND_QUEUE: LazyLock<CommandQueue> = LazyLock::new(CommandQueue::new);

impl Command {
    /// Whether this command has to run *inside* a macroquad frame.
    ///
    /// macroquad resets the draw list in `begin_frame()`, so anything that puts
    /// geometry on the screen - or changes render state those draws depend on,
    /// like the camera - only makes sense between a `begin_frame()` and the
    /// matching `end_frame()`. Everything else can run at any time.
    ///
    /// Natively the distinction is irrelevant: one thread drains one queue in
    /// order. In the browser the engine shares Python's thread, and a command
    /// that owes Python an answer cannot wait for a frame that will only happen
    /// once Python is unblocked - so the two kinds take different paths. See
    /// [`crate::web`].
    ///
    /// Written out exhaustively on purpose: a new [`Command`] should not compile
    /// until someone has decided which side it belongs on.
    #[cfg(target_os = "emscripten")]
    pub fn is_frame_local(&self) -> bool {
        match self {
            Command::GlEnum(..)
            | Command::DrawRectangleFromPyClass(..)
            | Command::DrawCircleFromPyClass(..)
            | Command::DrawAll3DObjects(..)
            | Command::DrawObjectNow(..)
            | Command::DrawArc { .. }
            | Command::DrawCubeWires { .. }
            | Command::DrawCylinder { .. }
            | Command::DrawCylinderWires { .. }
            | Command::DrawEllipse { .. }
            | Command::DrawEllipseLines { .. }
            | Command::DrawHexagon { .. }
            | Command::DrawLine3D { .. }
            | Command::DrawLine { .. }
            | Command::DrawAfflineParallelpiped { .. }
            | Command::DrawAfflineParallogram { .. }
            | Command::SetDefaultCamera(..)
            | Command::DrawRect { .. }
            | Command::DrawRectLines { .. }
            | Command::DrawTriangle { .. }
            | Command::DrawTriangleLines { .. }
            | Command::DrawPlane { .. }
            | Command::DrawGrid { .. }
            | Command::DrawCube { .. }
            | Command::DrawSkyBox { .. }
            | Command::DrawPoly { .. }
            | Command::DrawPolyLines { .. }
            | Command::DrawText { .. }
            | Command::DrawMultilineText { .. }
            | Command::DrawTexture { .. }
            | Command::ClearBackground { .. }
            | Command::NextFrame { .. }
            | Command::SetCamera { .. }
            | Command::PushCameraState
            | Command::PopCameraState => true,

            Command::CreateMeshFromBytes { .. }
            | Command::GetCustomMouseState { .. }
            | Command::Camera2DWorldToScreen { .. }
            | Command::Camera2DScreenToWorld { .. }
            | Command::Camera2DToMatrix { .. }
            | Command::MeasureText { .. }
            | Command::BuildTextureAtlas
            | Command::SetPcAssetFolder(..)
            | Command::Touches(..)
            | Command::TouchesLocal(..)
            | Command::SimulateMouseWithTouch(..)
            | Command::LoadTTFFOnt { .. }
            | Command::PopulateFontCache { .. }
            | Command::SetFontFilter { .. }
            | Command::LoadTTFFontFromBytes { .. }
            | Command::RequestNewScreenSize { .. }
            | Command::IsQuitRequested(..)
            | Command::PreventQuit
            | Command::SetFullscreen(..)
            | Command::TexImEnum(..)
            | Command::PhysicsEnum(..)
            | Command::ManuallyStepPhysics(..)
            | Command::SetCollisionForObject { .. }
            | Command::GetColissionObjects { .. }
            | Command::DoesObjectCollide { .. }
            | Command::SetDrawEachFrame { .. }
            | Command::DeleteObject { .. }
            | Command::GetObjectScale { .. }
            | Command::GetObjectPos { .. }
            | Command::GetObjectRotation { .. }
            | Command::SetObjectScale { .. }
            | Command::SetObjectPos { .. }
            | Command::SetObjectRotation { .. }
            | Command::CreateCube { .. }
            | Command::CreateSphere { .. }
            | Command::CreatePill { .. }
            | Command::CreateCylinder { .. }
            | Command::CreateMesh { .. }
            | Command::DropThisItem(..)
            | Command::LoadFile { .. }
            | Command::LoadSound { .. }
            | Command::LoadSoundFromBytes { .. }
            | Command::PlaySound { .. }
            | Command::PlaySoundOnce { .. }
            | Command::SetSoundVolume { .. }
            | Command::StopSound { .. }
            | Command::RenderTargetMsaa { .. }
            | Command::RenderTargetEx { .. }
            | Command::GetTextCenter { .. }
            | Command::ScreenDpiScale(..)
            | Command::LoadImage { .. }
            | Command::GetScreenData { .. }
            | Command::SetCursorGrab(..)
            | Command::ShowMouse(..)
            | Command::ClearInputQueue
            | Command::IsSimulatingMouseWithTouch(..)
            | Command::CameraFontScale { .. } => false,
        }
    }
}




/// State the engine owns for as long as the window lives.
///
/// Natively this is just a local of [`proccess_commands_loop`]. On Emscripten
/// two different call paths dispatch commands, so it has to be shared - see
/// [`crate::web`].
pub struct EngineState {
    pub object_storage: ObjectStorage::ObjectStorage,
    pub cam_memory: CamMemory,
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            object_storage: ObjectStorage::ObjectStorage::new(),
            cam_memory: CamMemory::new(),
        }
    }
}

impl Default for EngineState {
    fn default() -> Self {
        Self::new()
    }
}

/// Executes exactly one command.
///
/// Only a handful of arms ever `.await` - `NextFrame` and the asset loads.
/// Every other command completes on the first poll, which is what lets the
/// browser build run queries straight off the Python stack.
pub async fn dispatch(command: Command, st: &mut EngineState) {
            match command {
                Command::CreateMeshFromBytes{data, texture, sender}=>{
                    let mesh = Mesh::load_from_bytes(&data, texture.map(|t|t.into())).map_err(|e|{
                        PError::BasicErr(format!("invalid mesh: {e}"))
                    });
                    let _ = sender.send(mesh);
                }
                Command::GetCustomMouseState { sender }=>{
                    let (x,y,inside) = unsafe { MouseInsideScreen::get_mouse_state_info() };
                    sender.send((x,y,inside));
                }
                Command::Camera2DToMatrix { cam, sender } =>{
                    use macroquad::camera::Camera as mqCam;
                    let _ = sender.send( cam.matrix() );
                }
                Command::Camera2DWorldToScreen { cam, point, sender } =>{
                    let _ = sender.send(
                        cam.world_to_screen(point)
                    );
                }
                Command::Camera2DScreenToWorld { cam, point, sender } =>{
                    let _ = sender.send(
                        cam.screen_to_world(point)
                    );
                }
                Command::MeasureText { text, font, font_size, font_scale, sender }=>{
                    let _ = sender.send(
                        mq::measure_text(&text, font.as_ref(), font_size, font_scale)
                    );
                }
                Command::BuildTextureAtlas => {
                    mq::build_textures_atlas();
                },
                Command::SetPcAssetFolder(path)=> mq::set_pc_assets_folder(&path),
                Command::Touches(sender)=>{
                    let touches = mq::touches();
                    let _ = sender.send(touches);
                }
                Command::TouchesLocal(sender)=>{
                    let touches = mq::touches_local();
                    let _ = sender.send(touches);
                }
                Command::SimulateMouseWithTouch(option)=>{
                    mq::simulate_mouse_with_touch(option);
                }
                Command::LoadTTFFOnt { path, sender }=>{
                    let font  = mq::load_ttf_font(&path).await;
                    let _ = sender.send( font.map_err(Into::into ) );
                }
                Command::LoadTTFFontFromBytes { bytes, sender }=>{
                    let _ = sender.send( mq::load_ttf_font_from_bytes(&bytes).map_err(Into::into ));
                }
                Command::SetFontFilter { font, filter }=> {
                    let mut font = font;
                    font.set_filter(filter);
                }
                Command::PopulateFontCache { font, characters, size }
                    => font.populate_font_cache(&characters, size),
                Command::RequestNewScreenSize { width, height }
                    => mq::request_new_screen_size(width, height),
                Command::PreventQuit => mq::prevent_quit(),
                Command::SetFullscreen(fullscreen) => mq::set_fullscreen(fullscreen),
                Command::IsQuitRequested(sender)=>{
                    let is_quit_requested = mq::is_quit_requested();
                    let _ = sender.send(is_quit_requested);
                }
                Command::DrawPolyLines { x, y, sides, radius, rotation, thickness, color }
                    => {
                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        mq::draw_poly_lines(x, y, sides, radius, rotation, thickness, color)
                    }
                Command::DrawAfflineParallogram { offset, e1, e2, texture, color }
                    => {
                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        mq::draw_affine_parallelogram(offset, e1, e2, texture.as_ref(), color)}
                Command::DrawMultilineText { text, 
                    x, y, font_size, line_distance_factor, color, 
                    font_scale, font_scale_aspect, rotation, font } => {

                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        let params =  mq::TextParams{font: font.as_ref(), font_size, font_scale, font_scale_aspect, rotation, color};
                        let res = mq::draw_multiline_text_ex(&text, x, y,line_distance_factor, params );
                    }

                Command::ClearInputQueue => mq::clear_input_queue(),
                Command::IsSimulatingMouseWithTouch(sender)=>{
                    let _ = sender.send(mq::is_simulating_mouse_with_touch());
                }
                Command::DrawLine { x1, y1, x2, y2, thickness, color }
                    => {
                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        mq::draw_line(x1, y1, x2, y2, thickness, color)}
                Command::TexImEnum(en)=> en.execute(),
                Command::GlEnum(glenum)=> {
                    let gl  = unsafe {
                        get_internal_gl().quad_gl
                    };
                    implement_GlEnum(glenum, gl);
                }
                Command::DrawCircleFromPyClass(circle)=> {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    draw_circle(&circle)},
                Command::DrawRectangleFromPyClass(rect)=> {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    draw_rect(&rect)},
                Command::PhysicsEnum(phys,key )=> {
                    let handle = st.object_storage.get_handle(key).expect("No physics handle found, yet physics function was called.");
                    st.object_storage.apply_physics_enum(phys, &handle);
                }
                Command::DoesObjectCollide { key, sender }=>{
                    let mut res = false;
                    if let Some(handle) = st.object_storage.get_handle(key){
                        res = st.object_storage.physics_world.has_collision(handle.collider_handle);
                    }
                    let _ = sender.send(res);
                }
                Command::SetCollisionForObject{key, collider}=> {
                    st.object_storage.set_collision_for_object(key, collider);
                }
                Command::GetColissionObjects { key, sender }=>{
                    let keys = st.object_storage.collides_with(key);
                    let py_refs  = st.object_storage.keys_to_py(keys);
                    let _ = sender.send(py_refs);
                }
                Command::ManuallyStepPhysics(distance)=>{
                    st.object_storage.step_physics(distance);
                }
                Command::DrawAll3DObjects()=> {
                    let matrix;
                    unsafe {
                        let mat = mq::get_internal_gl();
                        matrix= mat.quad_gl.get_projection_matrix()
                    }
                    sm::switch_to_desired_shader(sm::ShaderKind::Basic, &None);
                    ObjectManagement::draw_all_Objects(&st.object_storage, matrix);
                }
                Command::DrawObjectNow(obj_key)=> {
                    let gl = unsafe {
                        mq::get_internal_gl().quad_gl
                    };
                    sm::switch_to_desired_shader(sm::ShaderKind::Basic, &None);
                    gl.draw_mode(mq::DrawMode::Triangles);
                    let obj=  st.object_storage.get(obj_key);
                    match obj {
                        Object::Cube(c)=> c.draw(gl),
                        Object::Cylinder(c)=> c.draw(gl),
                        Object::Mesh(m)=> m.draw(gl),
                        Object::Pill(p)=> p.draw(gl),
                        Object::Sphere(s)=> s.draw(gl),
                    }
                }
                Command::SetDrawEachFrame { key, set }=>{
                    let obj = unsafe {st.object_storage.get_mut(key)};
                    match obj{
                        Object::Cube(c)=> c.draw_each_frame = set,
                        Object::Cylinder(c)=> c.draw_each_frame = set,
                        Object::Mesh(c)=> c.draw_each_frame = set,
                        Object::Pill(c)=> c.draw_each_frame = set,
                        Object::Sphere(c)=> c.draw_each_frame = set,
                    }
                }
                Command::DeleteObject { key }=> {
                    st.object_storage.remove_object(key);
                }
                Command::GetObjectPos { key, sender } => {
                    let pos = match  st.object_storage.get(key){
                        Object::Cube(cube) => cube.position,
                        Object::Mesh(mesh) => mesh.position,
                        Object::Sphere(sphere)=> sphere.position,
                        Object::Cylinder(cyl)=> cyl.position,
                        Object::Pill(pill)=> pill.position,
                    };
                    let _ = sender.send(pos);
                }
                Command::GetObjectScale { key, sender } => {
                    let pos = match  st.object_storage.get(key){
                        Object::Cube(cube) => cube.scale,
                        Object::Mesh(mesh)=> mesh.scale,
                        Object::Sphere(sphere)=> sphere.scale,
                        Object::Cylinder(cyl)=> cyl.scale,
                        Object::Pill(pill)=> pill.scale,
                    };
                    let _ = sender.send(pos);
                }
                Command::GetObjectRotation { key, sender } => {
                    let pos = match  st.object_storage.get(key){
                        Object::Cube(cube) => cube.rotation,
                        Object::Mesh(mesh)=> mesh.rotation,
                        Object::Sphere(sphere)=> sphere.rotation,
                        Object::Cylinder(cyl)=> cyl.rotation,
                        Object::Pill(pill)=> pill.rotation,
                    };
                    let _ = sender.send(pos);
                }
                Command::SetObjectPos { key, position } => {
                    st.object_storage.change_obj_position(&position, key, 
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

                    st.object_storage.change_obj_scale(&scale, key, 
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
                    st.object_storage.change_obj_rotation(&rotation, key, 
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
                    st.object_storage.quick_push(collider,sender, weak_ref, 
                        move || {
                            let internal_pill = Pill::new(size, position, rotation, color,texture,);
                            Object::Pill(internal_pill)
                        });
                }
                Command::CreateCylinder { size, position, rotation, color,texture, collider, weak_ref, sender }=>{
                    st.object_storage.quick_push(collider,sender, weak_ref, 
                        move || {
                            let internal_cyl = Cylinder::new(size, position, rotation, color,texture);
                            Object::Cylinder(internal_cyl)
                        });
                }
                Command::CreateCube { size, position, rotation,color,texture,collider, weak_ref: pyAny, sender }=>{

                    st.object_storage.quick_push(collider,sender, pyAny, 
                        move || {
                            let internal_cube = Cube::new(size, position, rotation, color,texture);
                            Object::Cube(internal_cube)
                        });
                        
                }
                Command::CreateMesh { mesh,collider, weak_ref, sender }=>{

                    st.object_storage.quick_push(collider,sender, weak_ref, 
                        move || {
                            Object::Mesh(mesh)
                        });
                }
                Command::CreateSphere { size, position, rotation,color,texture, collider,weak_ref: pyAny, sender }=>{

                    st.object_storage.quick_push(collider,sender, pyAny, 
                        move || {
                            let internal_sphere = Sphere::new(size, position, rotation, color, texture);
                            Object::Sphere(internal_sphere)
                        });
                        
                }
                Command::DrawRectLines { x, y, w, h, thickness, color }
                    => {
                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        mq::draw_rectangle_lines(x, y, w, h, thickness, color)
                    },
                Command::DrawRect { x, y, w, h, color} => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_rectangle(
                        x,
                        y,
                        w,
                        h,
                        color,
                    );
                }
                Command::DrawTriangle { v1, v2, v3, color }
                    => {
                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        mq::draw_triangle(v1, v2, v3, color)
                    }
                Command::DrawTriangleLines { v1, v2, v3, thickness, color }
                    => {
                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        mq::draw_triangle_lines(v1, v2, v3, thickness, color)
                    }
                Command::DrawPlane { center, size, color, texture } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    let tex_ref = texture.as_ref();
                    mq::draw_plane(center,size,tex_ref,color);
                }
                Command::DrawGrid { slices, spacing, axes_color, other_color, center, rotation } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_grid_ex(slices, spacing, axes_color, other_color, center, rotation);
                }
                Command::DrawCube {pos,size,texture,color} => {
                    sm::switch_to_desired_shader(sm::ShaderKind::Basic, &None);
                    mq::draw_cube(pos,size,texture.as_ref(),color)
                }
                Command::DrawSkyBox {texture, tint} => {
                    
                    let cam  =match &st.cam_memory.current_cam{
                        Camera::Camera2D(_)=> panic!("should be 3d cam"),
                        Camera::Camera3D(_cam)=> clone_camera3d(_cam)
                    };
                    sm::switch_to_desired_shader(sm::ShaderKind::SkyBox, &Some(cam));
                    mq::draw_sphere_ex(mq::Vec3::ZERO, 10.0, texture.as_ref(), tint,DrawSphereParams { rings: 100, slices: 100, draw_mode: mq::DrawMode::Triangles });
                }
                Command::DrawAfflineParallelpiped { offset, e1, e2, e3, texture, color } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_affine_parallelepiped(offset, e1, e2, e3, texture.as_ref(), color);
                }
                Command::DrawArc { x, y, sides, radius, rotation, thickness, arc, color } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_arc(x, y, sides, radius, rotation, thickness, arc, color);
                }
        
                Command::DrawCubeWires { position, size, color } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_cube_wires(position, size, color);
                }
                Command::DrawCylinder { position, radius_top, radius_bottom, height, texture, color } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::Basic, &None);
                    mq::draw_cylinder(position, radius_top, radius_bottom, height, texture.as_ref(), color);
                }
        
                Command::DrawCylinderWires { position, radius_top, radius_bottom, height, texture, color } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_cylinder_wires(position, radius_top, radius_bottom, height, texture.as_ref(), color);
                }
        
                Command::DrawEllipse { x, y, w, h, rotation, color }    => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_ellipse(x, y, w, h, rotation, color);
                }
                
                Command::DrawEllipseLines { x, y, w, h, rotation, thickness, color }    => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_ellipse_lines(x, y, w, h, rotation, thickness, color);
                }
        
                Command::DrawHexagon { x, y, size, border, vertical, border_color, fill_color } => {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_hexagon(x, y, size, border, vertical, border_color, fill_color);
                }
        
                Command::DrawLine3D { start, end, color } =>{
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
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
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_poly(x,y,sides,radius,rotation,color  );
                }
                Command::SetDefaultCamera() =>{ 
                    set_default_camera(&mut st.cam_memory);
                }
        
                Command::DrawTexture { texture,x,y,color}=>
                {
                    sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                    mq::draw_texture(&texture,x,y,color)
                }
                
                Command::ClearBackground {color } => {
                    mq::clear_background(color);
                }
                Command::ScreenDpiScale(sender)=> {
                    let _ = sender.send(  mq::screen_dpi_scale() );
                }
        
                Command::NextFrame{physics_step, sender} => {
                    mq::next_frame().await;
                    crate::engine::SHADERS::shader_manager::new_frame_shader_update();
                    crate::engine::FrameInfo::update_frame_info();

                    let _ = sender.send(());
                    
                    if let Some(physics_step)  = physics_step{
                        st.object_storage.step_physics(physics_step);
                    }
                    mq::clear_background(BLACK); // 3d rendering is bugged if we don't clear.
                }
            
                Command::DrawText { 
                    text, 
                    x, 
                    y, 
                    font_size, 
                    color, 
                    font, 
                    font_scale, 
                    font_scale_aspect, 
                    rotation, 
                    sender } =>{
                        sm::switch_to_desired_shader(sm::ShaderKind::None, &None);
                        let font_options = mq::TextParams{
                            font: font.as_ref(),
                            font_size,
                            font_scale,
                            font_scale_aspect,
                            rotation,
                            color,
                        };

                        let res = mq::draw_text_ex(&text, x, y, font_options);
                        let _ = sender.send(res);
                    }
                Command::GetTextCenter { text, font, font_size, font_scale, rotation, sender }=>{
                    let _ = sender.send(
                        mq::get_text_center(&text, font.as_ref(), font_size, font_scale, rotation)
                    );
                }
                Command::SetCamera { camera_2d, camera_3d } => { // merged cam2d and 3d for simplicity.
                    match (*camera_2d, *camera_3d) {
                        (Some(cam), None) => {
                            set_camera(&mut st.cam_memory, Camera::Camera2D(cam));
                        },
                        (None, Some(cam)) => {
                            set_camera(&mut st.cam_memory, Camera::Camera3D(cam));
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
                    let result = result.map( PArc::new );
                
                    let _ = sender.send(result);
                }
                Command::LoadSoundFromBytes { data ,sender} => {
        
                    let result: Result<_, PError> = async {
                        //converts .mp3 to .wav
                        let secured_data  = crate::engine::AudioConverter::ensure_wav(data)?;
        
                        let sound = au::load_sound_from_bytes(&secured_data).await?;
                        Ok(sound)
                    }.await;
                
                    let result = result.map( PArc::new );
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
        
                Command::DropThisItem(_drop)=>{}
                Command::PopCameraState => CameraManager::pop_camera_state(),
                Command::PushCameraState => CameraManager:: push_camera_state(),
                Command::CameraFontScale { world_font_size, sender }=>{
                    let res  = CameraManager::camera_font_scale(world_font_size);
                    let _ = sender.send(res);
                }
        
                
            }
}

/// processes commands that rely on the macroquad engine
/// commands that do not rely on it's core (openGL) components ( or just the internal Core-Thread ) are found in pyabstractions.
#[cfg(not(target_os = "emscripten"))]
pub async fn proccess_commands_loop() {
    let mut st = EngineState::new();

    loop {
        while let Some(command) = COMMAND_QUEUE.pop() {
            dispatch(command, &mut st).await;
        }
    }
}

/// In the browser the engine shares one thread with CPython and the event loop,
/// so the loop needs a different shape entirely.
/// See [`crate::web::engine_loop`].
#[cfg(target_os = "emscripten")]
pub async fn proccess_commands_loop() {
    crate::web::engine_loop().await
}
