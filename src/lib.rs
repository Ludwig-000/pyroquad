//#![allow(warnings)]
#![allow(non_snake_case)] // alot of Python Constants are defined via function, so this prevents compiler spam.
#![allow(unused_variables)] // for now.
#![allow(dead_code)] // for now.

#![allow(clippy::excessive_precision)]
#![warn(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_range_loop)]
#![deny(clippy::disallowed_methods)]

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

mod engine;
mod py_abstractions;



#[pymodule]
#[pyo3(gil_used = false)]
pub fn _pyroquad( py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::activate_engine, m)?)?;
    
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_all_objects, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_rectangle, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_poly, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_circle, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_affine_parallelepiped, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_arc, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cube_wires, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cylinder, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cylinder_wires, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_ellipse, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_ellipse_lines, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_hexagon, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_line_3d, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::step_physics, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::next_frame, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::clear_background, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_text, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_fps, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_delta_time, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_keys_pressed, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_keys_down, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_keys_released, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_char_pressed, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::get_mouse_buttons_down, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::get_mouse_buttons_pressed, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::get_mouse_buttons_released, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::get_mouse_delta_position, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::get_mouse_position_local, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::get_mouse_wheel, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::get_mouse_position, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::set_cursor_grab, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::show_mouse, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_screen_data, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_last_key_pressed, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_grid, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_texture, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_plane, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cube, m)?)?;

    m.add_function(wrap_pyfunction!(crate::engine::Cubemap::draw_cubemap, m)?)?;



    m.add_function(wrap_pyfunction!(crate::py_abstractions::Camera::set_default_camera, m)?)?;


    m.add_class::<crate::py_abstractions::Color::Color>()?;

    m.add_class::<crate::py_abstractions::Loading::ThreadedLoading::Loading>()?;
    m.add_class::<crate::py_abstractions::Loading::FileData::FileData>()?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::load_file, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::download_file, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::write_to_file, m)?)?;

    m.add_class::<crate::py_abstractions::GL::Vertex>()?;


    
    m.add_class::<crate::py_abstractions::GL::InternalGL>()?;


    //m.add_class::<crate::py_abstractions::structs::GL::Internal_GL>()?;

    m.add_class::<crate::py_abstractions::RenderTarget::RenderTarget>()?;
    m.add_class::<crate::py_abstractions::RenderTarget::RenderTargetParams>()?;
    m.add_class::<crate::py_abstractions::Textures_and_Images::Texture2D>()?;
    m.add_class::<crate::py_abstractions::Textures_and_Images::Image>()?;
    m.add_class::<crate::py_abstractions::Camera::Camera2D>()?;
    m.add_class::<crate::py_abstractions::Camera::Camera3D>()?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::RenderTarget::render_target_msaa, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::RenderTarget::render_target, m)?)?;



    m.add_class::<crate::py_abstractions::Audio::PlaySoundParams>()?;
    m.add_class::<crate::py_abstractions::Audio::Sound>()?;

    m.add_class::<crate::py_abstractions::Config::Config>()?;

    m.add_class::<crate::py_abstractions::structs::GLAM::BVec2::BVec2>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::BVec3::BVec3>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::Vec3::Vec3>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::Vec2::Vec2>()?;


    m.add_class::<crate::py_abstractions::structs::ThreeDObjects::Cube::Cube>()?;
    m.add_class::<crate::py_abstractions::structs::ThreeDObjects::Sphere::Sphere>()?;
    m.add_class::<crate::py_abstractions::structs::ThreeDObjects::Mesh::Mesh>()?;
    m.add_class::<crate::py_abstractions::structs::ThreeDObjects::Pill::Pill>()?;
    m.add_class::<crate::py_abstractions::structs::ThreeDObjects::Cylinder::Cylinder>()?;

    m.add_class::<crate::py_abstractions::structs::TwoDObjects::Circle::Circle>()?;
    m.add_class::<crate::py_abstractions::structs::TwoDObjects::Rectangle::Rectangle>()?;
    m.add_class::<crate::py_abstractions::structs::ThreeDObjects::PhysicsHandle::Physics>()?;
    m.add_class::<crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::ColliderOptions>()?;

    m.add_class::<crate::py_abstractions::Shader::Shader>()?;
    m.add_class::<crate::py_abstractions::KeyCode::KeyCode>()?;

    m.add_class::<crate::py_abstractions::MouseButton::MouseButton>()?;



    Ok(())
}

define_stub_info_gatherer!(stub_info);

/*

list of macroquad enums

   mq::Comparison
    mq::DrawMode
    mq::EulerRot
    mq::FilterMode
    mq::ImageFormat
    mq::MouseButton
    mq::Projection
    mq::ShaderError
    mq::ShaderSource
    mq::TouchPhase
    mq::UniformType




*/





/*
list of macroquad::prelude functions



    mq::build_textures_atlas
    mq::camera_font_scale
    mq::cartesian_to_polar
    mq::clamp
    mq::clear_background
    mq::clear_input_queue

    mq::draw_affine_parallelepiped
    mq::draw_affine_parallelogram
    mq::draw_arc
    mq::draw_circle
    mq::draw_circle_lines
    mq::draw_cube
    mq::draw_cube_wires
    mq::draw_cylinder
    mq::draw_cylinder_ex
    mq::draw_cylinder_wires
    mq::draw_ellipse
    mq::draw_ellipse_lines
    mq::draw_fps
    mq::draw_grid
    mq::draw_grid_ex
    mq::draw_hexagon
    mq::draw_line
    mq::draw_line_3d
    mq::draw_mesh
    mq::draw_multiline_text
    mq::draw_plane
    mq::draw_poly
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
    mq::load_file
    mq::load_image
    mq::load_material
    mq::load_string
    mq::load_texture
    mq::load_ttf_font_from_bytes
    mq::measure_text
    mq::mouse_delta_position
    mq::mouse_position
    mq::mouse_position_local
    mq::mouse_wheel
    mq::next_frame
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
    mq::set_camera
    mq::set_cursor_grab
    mq::set_default_camera
    mq::set_default_filter_mode
    mq::set_fullscreen
    mq::set_panic_handler
    mq::set_pc_assets_folder
    mq::show_mouse
    mq::simulate_mouse_with_touch
    mq::touches
    mq::touches_local


*/