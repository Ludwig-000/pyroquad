// python abstractions for both macroquad functions and custom functions.
// All functions are either 
// 1) executed directly, if no engine context is needed
// 2) will be pushed into COMMAND_QUEUE to be executed by macroquad
//
// also, any conversion between my abstracted pyclasses and the structs used in macroquad is being done here.
// ( example:  Color -> mq::Color )

use crate::py_abstractions::Loading::Loading::PC_ASSET_FOLDER;
use crate::py_abstractions::Text::Font;
use crate::py_abstractions::Text::TextDimensions;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage;
use crate::py_abstractions::Textures_and_Images::*;
use crate::py_abstractions::structs::TwoDObjects::collision::Shape;
use macroquad::prelude as mq;

use pyo3::prelude::*;

use pyo3_stub_gen::{derive::gen_stub_pyfunction};

use crate::engine::PChannel;

use std::collections::HashSet;

use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::Color::*;
use crate::py_abstractions::KeyCode::*;
use crate::py_abstractions::Config::Config;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::panic::{self, AssertUnwindSafe};



pub static ENGINE_CURRENTLY_ACTIVE: AtomicBool = AtomicBool::new(false);


/// [!] This should generally be the first function call.
///
/// Turns on the pyroquad engine, creates an open-gl window and allows for engine-calls to be processed.
///
/// Note that calling functions of the engine before this call is undefined behavious.
/// Some things, like Vector-maths will run fine, some functions like 'get_keys_pressed' will return a default value, 
/// but other functions may result in a deadlock.
/// 
/// The engine is built, assuming none of it's library calls are ever executed without the engine being active.
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (conf = None))] // overloads activate_engine with config
pub fn activate_engine( conf: Option<Config>) -> PyResult<()>{

    
    let conf = conf.unwrap_or_default();
    let macroConf =  Config::to_window_config(conf.clone());

    #[allow(clippy::disallowed_methods)]
    let (tx, rx) = std::sync::mpsc::sync_channel(1);

    ENGINE_CURRENTLY_ACTIVE.store(true, Ordering::SeqCst);
    std::thread::spawn(move || {
        let panic_catcher = panic::catch_unwind(AssertUnwindSafe(|| {

            macroquad::Window::from_config(macroConf, async move  {
                
                crate::engine::EngineSetup::setup_engine();
                crate::engine::FrameInfo::update_frame_info();
                // we make sure frame info is updated, so that statics are initialized.
                let _ = tx.send(());
                
                crate::engine::CoreLoop::proccess_commands_loop().await;
                
            });
    
            ENGINE_CURRENTLY_ACTIVE.store(false, Ordering::SeqCst);
    
            if conf.stop_python_when_closing_window{
                println!("Pyroquad window closed. Exiting process.");
                process::exit(0);
            }
            
        }));


    });

    
    rx.recv().map_err(|_| {
        ENGINE_CURRENTLY_ACTIVE.store(false, Ordering::SeqCst);
        pyo3::exceptions::PyRuntimeError::new_err("Engine failed to initialize")
    })
}





#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_all_objects() {
    COMMAND_QUEUE.push( Command::DrawAll3DObjects() );
}



/// draws a rectangle with a given color.
/// viewing the rectangle required a 2D Camera ( default )
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
    COMMAND_QUEUE.push(Command::DrawRect { x, y, w, h,color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_rectangle_lines(x: f32, y: f32, w: f32, h: f32,thickness: f32,  color: Color) {
    COMMAND_QUEUE.push(Command::DrawRectLines { x, y, w, h,thickness,color: color.into()});
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_triangle(v1: Vec2, v2: Vec2, v3: Vec2, color: Color) {
    COMMAND_QUEUE.push(Command::DrawTriangle { v1: v1.into(), v2: v2.into(), v3: v3.into(), color: color.into() });
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_triangle_lines(v1: Vec2, v2: Vec2, v3: Vec2, thickness: f32, color: Color) {
    COMMAND_QUEUE.push(Command::DrawTriangleLines { v1: v1.into(), v2: v2.into(), v3: v3.into(), thickness, color: color.into() });
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_affine_parallelepiped(offset: Vec3, e1: Vec3,e2: Vec3,e3: Vec3,texture: Option<Texture2D>,color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawAfflineParallelpiped { offset: offset.into(), e1: e1.into(), e2: e2.into(), e3: e3.into(), texture: texture.map(Into::into), color: color.into() });
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_affine_parallelogram(offset: Vec3, e1: Vec3,e2: Vec3,texture: Option<Texture2D>,color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawAfflineParallogram { offset: offset.into(), e1: e1.into(), 
        e2: e2.into(), texture: texture.map(Into::into), color: color.into() });
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn step_physics(distance: f32) {
    
    COMMAND_QUEUE.push(Command::ManuallyStepPhysics(distance));
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_arc( x: f32,
    y: f32,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    arc: f32,
    color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawArc { x, y, sides, radius, rotation, thickness, arc, color: color.into() });
}



#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cube_wires( position: Vec3, size: Vec3, color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawCubeWires {position: position.into(),size: size.into(),color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cylinder( position: Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<Texture2D>,
    color: Color ) {
    
    COMMAND_QUEUE.push(Command::DrawCylinder {position: position.into(), radius_top, radius_bottom, 
        height, texture: texture.map(Into::into),color: color.into()});
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cylinder_wires( position: Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<Texture2D>,
    color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawCylinderWires {position: position.into(), radius_top, radius_bottom, 
        height, texture: texture.map(Into::into),color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_ellipse( x: f32, y: f32, w: f32, h: f32, rotation: f32, color: Color){
    COMMAND_QUEUE.push(Command::DrawEllipse { x, y, w, h, rotation, color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_ellipse_lines( x: f32,
    y: f32,
    w: f32,
    h: f32,
    rotation: f32,
    thickness: f32,
    color: Color){
    COMMAND_QUEUE.push(Command::DrawEllipseLines { x, y, w, h, rotation, thickness, color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_hexagon( x: f32,
    y: f32,
    size: f32,
    border: f32,
    vertical: bool,
    border_color: Color,
    fill_color: Color){
    COMMAND_QUEUE.push(Command::DrawHexagon { x, y, size, border, vertical, border_color: border_color.into(), fill_color: fill_color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_line_3d( start: Vec3, end: Vec3, color: Color){
    COMMAND_QUEUE.push(Command::DrawLine3D { start: start.into(), end: end.into(), color: color.into()});
}
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color){
    COMMAND_QUEUE.push(Command::DrawLine { x1, y1, x2, y2, thickness, color: color.into() });
}


/// draws a basic grid in 3d space.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_grid(slices: u32, spacing: f32, axes_color: Color, other_color: Color) {
    let c =Command::DrawGrid { slices, spacing, axes_color: axes_color.into(), other_color: other_color.into() };
    COMMAND_QUEUE.push(c);
}

/// draws a flat plane in 3d space.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_plane(center: Vec3, size: Vec2, color: Color, texture: Option<Texture2D>)  {
    let cen = mq::vec3( center.x,center.y,center.z);
    let siz = mq::vec2(size.x,size.y);
    let tex = texture.map(|t| t.into());

    let c = Command::DrawPlane { center:cen,size:siz,color: color.into(),texture: tex};

    COMMAND_QUEUE.push(c);
}

/// draws a basic 3d cube.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cube(position: Vec3, size: Vec3, color: Color, texture: Option<Texture2D>) {
    let texture = texture.map( Into::into );
    COMMAND_QUEUE.push(  Command::DrawCube{pos: position.into(), size: size.into(), texture, color: color.into()} );
}

/// fills the entire screen with a single color.
/// this is usually used at the start of a frame.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn clear_background(color: Color) {
    COMMAND_QUEUE.push(Command::ClearBackground { color: color.into()});
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn screen_dpi_scale() -> PyResult<f32>{
    let (sender, receiver) = PChannel::PChannel::sync_channel(1);
    COMMAND_QUEUE.push(Command::ScreenDpiScale(sender));
    Ok(receiver.recv()?)
}

/// processes all drawing commands that have accumulated.
/// blocks until the frame has been drawn.
///
/// also, this function cleans up dropped memory such as Texture2D
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (physics_step = Some(0.0)))] 
pub fn next_frame(py: Python<'_>, physics_step: Option<f32>) -> PyResult<()>{
    {
        let fn_storage = ObjectFunctionStorage::get_fun_storage();
        fn_storage.execute_all(py)?;
    }
    let (sender, receiver) = PChannel::PChannel::sync_channel(1);
    COMMAND_QUEUE.push(Command::NextFrame { physics_step, sender });

    receiver.recv()?;
    Ok(())
}

/// draws a text in 2d space.
/// requires a 2d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (text, x, y, color = Color::WHITE(), font_size  = 20,
    font = None,  font_scale = 1.0, font_scale_aspect =  1.0, rotation = 0.0))] 
pub fn draw_text(text: String, x: f32, y: f32, color: Color, font_size: u16,
    font: Option<Font>, font_scale: f32, font_scale_aspect: f32, rotation: f32) -> PyResult<TextDimensions>{

    let (sender, receiver) = PChannel::PChannel::sync_channel(1);
    COMMAND_QUEUE.push(Command::DrawText { 
        text, 
        x, 
        y, 
        color: color.into(), 
        font: font.map(Into::into), font_size, font_scale, font_scale_aspect, rotation, sender });
    Ok(receiver.recv()?.into())
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_text_center(text: String, font: Option<Font>, font_size: u16, font_scale: f32, rotation: f32) -> PyResult<Vec2>{
    let (sender, receiver) = PChannel::PChannel::sync_channel(1);
    COMMAND_QUEUE.push(Command::GetTextCenter { text, font: font.map(Into::into), font_size, font_scale, rotation, sender });

    Ok(receiver.recv()?.into())
}


#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (text, x,y,font_size = 20, color = Color::WHITE(), font = None, rotation=0.0, line_distance_factor=None, font_scale=1.0, font_scale_aspect=1.0))]
pub fn draw_multiline_text(text: String, x: f32, y: f32, font_size: u16, color: Color, font: Option<Font>, rotation: f32, line_distance_factor: Option<f32>, 
    font_scale: f32, font_scale_aspect: f32) {

    COMMAND_QUEUE.push(Command::DrawMultilineText { text, x, y, 
        font_size, line_distance_factor, color: color.into(), font_scale, 
        font_scale_aspect, rotation, font: font.map(Into::into) });
}

/// draws very basic circle in 2d space.
/// requires a 2d camera to be seen.
///
/// note that this function simply draws a 20-sided polygon.
/// for a more "round" circle, simply call `draw_poly()` with a greater ammount of sides.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_circle(x: f32, y: f32, r: f32, color: Color) {
    COMMAND_QUEUE.push(Command::DrawPoly{ x, y, sides:20, radius:r, rotation:0.0, color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_circle_lines(x: f32, y: f32, r: f32, thickness: f32, color: Color) {
    COMMAND_QUEUE.push(Command::DrawPolyLines{ x, y, sides:20, radius:r, rotation:0.0,thickness, color: color.into()});
}

/// draws n-sided polygon in 2d space. 
/// increasing the polygon count will simply make it a circle.
/// requires a 2d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color) {
    COMMAND_QUEUE.push(Command::DrawPoly{ x, y, sides, radius, rotation, color: color.into()});
}
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_poly_lines(x: f32, y: f32, sides: u8, radius: f32, rotation: f32,thickness: f32, color: Color) {
    COMMAND_QUEUE.push(Command::DrawPolyLines { x, y, sides, radius, rotation, thickness, color: color.into() });
}

/// draws a texture in 2d space.
/// requires a 2d-camera to be seen.
/// 
/// a texture gets created by calling `Texture2D.from_image( image )`
///
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (texture, x=0.0, y=0.0, color=Color::WHITE()))]
pub fn draw_texture(texture: Texture2D,x: f32, y: f32, color: Color ) {
    let innerTexture: mq::Texture2D  = texture.into();
    COMMAND_QUEUE.push( Command::DrawTexture{ texture: innerTexture, x, y, color: color.into()   }  );
}

/// returns the current frames per second
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_fps() -> i32 {
    use crate::engine::FrameInfo::*;
    FPS.load(Ordering::Relaxed)
}

/// Returns duration in seconds of the last frame drawn.
/// This is useful for F.E. animations, that have to keep the same pace,
/// independent of the frame rate.
/// 
///Example:
///```
///>>>rect_x  = 0
///>>>while True:
///...  delta_time = get_delta_time()
///...  rect_x += (2.0*delta_time)
///...
///...  draw_rectangle(x=rect_x, y=50, w=50, h=50, color=Color.WHITE())
///...
///...  next_frame()
/// ```
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_delta_time() -> f32 {
    use crate::engine::FrameInfo as fi;
    *fi::DELTA_TIME.lock().unwrap()
}




#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_screen_data() -> PyResult<Image> {
    let (tx, rx) = PChannel::PChannel::sync_channel(1);
    COMMAND_QUEUE.push( Command::GetScreenData { sender: tx } );

    let res = rx.recv()?;
    Ok(
        Image { bytes: res.bytes, width: res.width, height: res.height }
    )
}


/// returns an list of all keys that have been pressed since the last check.
/// pressed = key down + key up
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_keys_pressed() -> HashSet<KeyCode> {
    
    use crate::engine::FrameInfo as fi;
    let keyset = fi::KEYS_PRESSED.lock().unwrap().clone();


    let converted_keys: HashSet<KeyCode> = keyset
        .into_iter()
        .map(KeyCode::from)
        .collect();

    converted_keys

}


/// returns an list of all keys that have been released since the last check.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_keys_released() -> HashSet<KeyCode> {
    
    use crate::engine::FrameInfo as fi;
    let keyset = fi::KEYS_RELEASED.lock().unwrap().clone();


    let converted_keys: HashSet<KeyCode> = keyset
        .into_iter()
        .map(KeyCode::from)
        .collect();

    converted_keys

}



/// returns an list of all keys that are currently in the process of being pressed.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_keys_down() -> HashSet<KeyCode> {

    use crate::engine::FrameInfo as fi;
    let keyset = fi::KEYS_DOWN.lock().unwrap().clone();


    let converted_keys: HashSet<KeyCode> = keyset
        .into_iter()
        .map(KeyCode::from)
        .collect();

    converted_keys
}

/// This function is useful in combination with 'prevent_quit()', 
///     to run some cleanup logic before closing the window and terminating the process.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn is_quit_requested() -> PyResult<bool> {
    let (sender, receiver) = PChannel::PChannel::sync_channel(1);
    COMMAND_QUEUE.push( Command::IsQuitRequested(sender));
    Ok(receiver.recv()?)
}

/// Prevents clowsing the window via 'Alt + F4', clicking 'X' on the window or similar.
/// Instead, the 'is_quit_requested()' flag will be toggled, and python is expected to close the window.
/// This is useful, if some cleanup HAS to be done, before the window can be safely closed, and the process terminated.
/// 
/// Once called, this flag will last the entire program.
/// 
#[gen_stub_pyfunction]
#[pyfunction]
pub fn prevent_quit() {
    COMMAND_QUEUE.push( Command::PreventQuit);
}
#[gen_stub_pyfunction]
#[pyfunction]
pub fn set_fullscreen(fullscreen: bool) {
    COMMAND_QUEUE.push( Command::SetFullscreen(fullscreen));
}



/// Return the last pressed key.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_last_key_pressed() -> Option<KeyCode> {
    use crate::engine::FrameInfo as fi;
    let keyset = *fi::LASK_KEY_PRESSED.lock().unwrap();
    keyset.map(|key| key.into() )
}


/// Return the last pressed char.
/// Each "get_char_pressed" call will consume a character from the input queue.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_char_pressed() -> Option<char> {

    use crate::engine::FrameInfo as fi;
    *fi::CHAR_PRESSED.lock().unwrap()
}

/// TODO: appears to not update when resizing the window?
/// need to investigate
#[gen_stub_pyfunction]
#[pyfunction]
pub fn screen_width() -> f32 {

    use crate::engine::FrameInfo as fi;
    *fi::SCREEN_WIDTH.lock().unwrap()
}

/// Request the window size to be the given value. 
/// This takes DPI into account.
/// Note that the OS might decide to give a different size. 
/// Additionally, the size won't be updated until the next next_frame()
#[gen_stub_pyfunction]
#[pyfunction]
pub fn request_new_screen_size(width: f32, height: f32) {
    COMMAND_QUEUE.push( Command::RequestNewScreenSize { width, height });
}

/// TODO: appears to not update when resizing the window?
/// need to investigate
#[gen_stub_pyfunction]
#[pyfunction]
pub fn screen_height() -> f32 {

    use crate::engine::FrameInfo as fi;
    *fi::SCREEN_HEIGHT.lock().unwrap()
}

/**
 * !Requires an active 3D camera.
 * 
 * 'draw_skybox' is being drawn with depth-test disabled.
 * This means, just like 'clear_background()', it is intended to be the first draw-call right after a 3D camera has been set.
 * 
 * 
 * SKYBOX TEXTURE REQUIREMENTS:
 * ----------------------------
 * Type:       Equirectangular Environment Map (Lat-Long)
 * Format:     LDR / Tonemapped (sRGB)
 * Projection: 360° Horizontal x 180° Vertical
 * Aspect:     2:1 (e.g., 2048x1024)
 * Note: If using an HDR/EXR file (like from Poly Haven), it MUST be 
 * tonemapped to a standard 8-bit image (JPG/PNG) before use.
 * Reference Examples: https://polyhaven.com/hdris
 * 
 * Also, you might need to mirror the Image vertically before converting it to a Texture2D.
 */
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (texture, tint = Color::WHITE()))] 
pub fn draw_skybox(texture: Texture2D, tint: Color) {
    let texture_unpacked = texture.into();
    COMMAND_QUEUE.push(  Command::DrawSkyBox{
        texture: Some(texture_unpacked), tint: tint.into()} );

}


/// Converts 2d polar coordinates to 2d cartesian coordinates.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn polar_to_cartesian(rho: f32, theta: f32) -> Vec2{
    mq::polar_to_cartesian(rho, theta).into()
}

/// Converts 2d polar coordinates to 2d cartesian coordinates.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn cartesian_to_polar(cartesian: Vec2) -> Vec2{
    mq::cartesian_to_polar(cartesian.into()).into()
}


/// There are super common project layout like this:
/// ```skip
///    .
///    ├── assets
///    ├── └── nice_texture.png
///    ├── src
///    ├── └── main.rs
///    └── Cargo.toml
/// ```
/// when such a project being run on desktop assets should be referenced as
/// "assets/nice_texture.png".
/// While on web or android it usually is just "nice_texture.png".
/// The reason: on PC assets are being referenced relative to current active directory/executable path. In most IDEs its the root of the project.
/// While on, say, android it is:
/// ```skip
/// [package.metadata.android]
/// assets = "assets"
/// ```
/// And therefore on android assets are referenced from the root of "assets" folder.
///
/// In the future there going to be some sort of meta-data file for PC as well.
/// But right now to resolve this situation and keep pathes consistent across platforms
/// `set_pc_assets_folder("assets");`call before first `load_file`/`load_texture` will allow using same pathes on PC and Android.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn set_pc_assets_folder(path: String){

    *PC_ASSET_FOLDER.lock().unwrap() = path.clone();

    COMMAND_QUEUE.push(Command::SetPcAssetFolder(path));
}




/// Batches the processing of many shape draw calls into a single native execution.
/// The list will be drawn left to right.
/// This is SIGNIFICANTLY faster than running a python for-loop. ( roughly a 2.3x performance improvement at 3000 ish elements. )
/// 
/// ```
/// # example
/// def draw(objects: list[Rectangle | Circle]):
///     for o in objects:
///         o.draw() # <- DO NOT DO THIS, if the list is large.
/// ```
/// 
/// ```
/// # example
/// def draw(objects: list[Rectangle | Circle]):
///     batch_draw_shapes(objects) <- do this instead.
/// ```
/// 
#[gen_stub_pyfunction]
#[pyfunction]
pub fn batch_draw_shapes<'py>(input: Vec<Shape<'py>>){
    for shape in input{
        match shape{
            Shape::Circ(c)=> {
                let c: PyRef<'_, super::structs::TwoDObjects::Circle::Circle> = c.borrow();
                c.draw();
            },
            Shape::Rect(r)=>{
                let r: PyRef<'_, super::structs::TwoDObjects::Rectangle::Rectangle> = r.borrow(); 
                r.draw();
            }
        }
    }
}