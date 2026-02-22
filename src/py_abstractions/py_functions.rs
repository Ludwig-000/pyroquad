// python abstractions for both macroquad functions and custom functions.
// All functions are either 
// 1) executed directly, if no engine context is needed
// 2) will be pushed into COMMAND_QUEUE to be executed by macroquad
//
// also, any conversion between my abstracted pyclasses and the structs used in macroquad is being done here.
// ( example:  Color -> mq::Color )

use crate::py_abstractions::MouseButton::MouseButton;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage;
use crate::py_abstractions::Textures_and_Images::*;
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
use pyo3::exceptions::PyRuntimeError;
use std::panic::{self, AssertUnwindSafe};
use std::thread;



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

    ENGINE_CURRENTLY_ACTIVE.store(true, Ordering::SeqCst);
    std::thread::spawn(move || {
        let panic_catcher = panic::catch_unwind(AssertUnwindSafe(|| {

            macroquad::Window::from_config(macroConf, async move  {
                
                crate::engine::EngineSetup::setup_engine();
                crate::engine::CoreLoop::proccess_commands_loop().await;
    
            });
    
            ENGINE_CURRENTLY_ACTIVE.store(false, Ordering::SeqCst);
    
            if conf.stop_pyton_when_closing_window{
                println!("Pyquad window closed. Exiting process.");
                process::exit(0);
            }
            
        }));

        // check if the engine paniced. if yes, run cleanup.
        if let Err(cause) = panic_catcher {
            
        }
        
    });

    Ok(())
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
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawRect { x, y, w, h,color:c});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_affine_parallelepiped(offset: Vec3, e1: Vec3,e2: Vec3,e3: Vec3,texture: Option<Texture2D>,color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawAfflineParallelpiped { offset: offset.into(), e1: e1.into(), e2: e2.into(), e3: e3.into(), texture: texture.map(Into::into), color: color.into() });
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
    
    COMMAND_QUEUE.push(Command::DrawCylinder {position: position.into(), radius_top: radius_top, radius_bottom: radius_bottom, 
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
    
    COMMAND_QUEUE.push(Command::DrawCylinderWires {position: position.into(), radius_top: radius_top, radius_bottom: radius_bottom, 
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



/// draws a basic grid in 3d space.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_grid(slices: u32, spacing: f32, axes_color: Color, other_color: Color) {
    let c1 = mq::Color::new(axes_color.r,axes_color.g,axes_color.b,axes_color.a);
    let c2 = mq::Color::new(other_color.r,other_color.g,other_color.b,other_color.a);
    let c =Command::DrawGrid { slices, spacing, axes_color: c1, other_color: c2 };

    COMMAND_QUEUE.push(c );
}

/// draws a flat plane in 3d space.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_plane(center: Vec3, size: Vec2, color: Color, texture: Option<Texture2D>)  {
    let col = mq::Color::new(color.r,color.g,color.b,color.a);
    let cen = mq::vec3( center.x,center.y,center.z);
    let siz = mq::vec2(size.x,size.y);
    let tex = texture.map(|t| t.into());

    let c =Command::DrawPlane { center:cen,size:siz,color: col,texture: tex};

    COMMAND_QUEUE.push(c );
}

/// draws a basic 3d cube.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cube(position: Vec3, size: Vec3, color: Color) {
    let col = mq::Color{r: color.r,g: color.g,b :color.b,a: color.a};
    let pos = mq::vec3(position.x,position.y,position.z);
    let siz = mq::vec3(size.x,size.y,size.z);
    let texture = None; // for now
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(  Command::DrawCube{pos: pos, size: siz, texture, color: c} );
}

/// fills the entire screen with a single color.
/// this is usually used at the start of a frame.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn clear_background(color: Color) {
    let col = mq::Color{r: color.r,g: color.g,b: color.b,a: color.a};
    COMMAND_QUEUE.push(Command::ClearBackground { color: col});
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
pub fn draw_text(text: String, x: f32, y: f32, font_size: f32, color: Color) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawText {text, x, y, font_size, color:c});
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
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawPoly{ x, y, sides:20, radius:r, rotation:0.0, color:c});
}

/// draws n-sided polygon in 2d space. 
/// increasing the polygon count will simply make it a circle.
/// requires a 2d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawPoly{ x, y, sides, radius, rotation, color: c});
}

/// draws a texture in 2d space.
/// requires a 2d-camera to be seen.
/// 
/// a texture gets created by calling `Texture2D.from_image( image )`
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_texture(texture: Texture2D,x: f32, y: f32, color: Color ) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    let innerTexture: mq::Texture2D  = texture.into();
    COMMAND_QUEUE.push( Command::DrawTexture{ texture: innerTexture, x, y, color: c   }  );
   
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




//none yet here

/*
list of macroquad::prelude functions



    mq::build_textures_atlas
    mq::camera_font_scale
    mq::cartesian_to_polar
    mq::clear_input_queue

    mq::draw_line_3d
    mq::draw_mesh
    mq::draw_multiline_text
    mq::draw_plane
    mq::draw_poly_lines
    mq::draw_rectangle
    mq::draw_rectangle_ex
    mq::draw_rectangle_lines
    mq::draw_rectangle_lines_ex
    mq::draw_sphere
    mq::draw_sphere_ex
    mq::draw_sphere_wires

    mq::draw_text
    mq::draw_text_ex

    mq::draw_texture
    mq::draw_texture_ex
    mq::draw_triangle
    mq::draw_triangle_lines
    mq::get_char_pressed
    mq::get_dropped_files
    mq::get_fps
    mq::get_frame_time
    mq::get_internal_gl
    mq::get_keys_down
    mq::get_keys_pressed
    mq::get_keys_released
    mq::get_last_key_pressed
    mq::get_screen_data
    mq::get_text_center
    mq::get_time
    mq::gl_use_default_material
    mq::gl_use_material
    mq::is_key_down
    mq::is_key_pressed
    mq::is_key_released
    mq::is_mouse_button_down
    mq::is_mouse_button_pressed
    mq::is_mouse_button_released
    mq::is_quit_requested
    mq::is_simulating_mouse_with_touch
    mq::load_material
    mq::load_string
    mq::load_ttf_font_from_bytes
    mq::measure_text
    mq::mouse_delta_position
    mq::mouse_position
    mq::mouse_position_local
    mq::mouse_wheel
    mq::polar_to_cartesian
    mq::pop_camera_state
    mq::prevent_quit
    mq::push_camera_state
    mq::quat
    mq::render_target
    mq::render_target_ex
    mq::render_target_msaa
    mq::request_new_screen_size
    mq::screen_dpi_scale
    mq::screen_height
    mq::screen_width
    mq::set_cursor_grab
    mq::set_default_filter_mode
    mq::set_fullscreen
    mq::set_panic_handler
    mq::set_pc_assets_folder
    mq::show_mouse
    mq::simulate_mouse_with_touch
    mq::touches
    mq::touches_local


*/