// alot of Python Constants are defined via function, so this prevents compiler spam.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#![allow(unused)] // temp

#![warn(clippy::large_enum_variant)]

#![deny(clippy::disallowed_methods)]

#![allow(clippy::excessive_precision)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::style)]


#[cfg(feature = "use-real")]
pub extern crate pyo3_stub_gen_real as pyo3_stub_gen;
#[cfg(not(feature = "use-real"))]
pub use ::pyo3_stub_gen;


use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

mod engine;
mod py_abstractions;


#[pymodule]
#[pyo3(gil_used = false)]
pub fn _pyroquad( m: &Bound<'_, PyModule>) -> PyResult<()> {


    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::activate_engine, m)?)?;
    
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_all_objects, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_rectangle, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_rectangle_lines, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_triangle, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_triangle_lines, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_poly, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_poly_lines, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_circle, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_circle_lines, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_affine_parallelepiped, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_affine_parallelogram, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_arc, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cube_wires, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cylinder, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cylinder_wires, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_ellipse, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_ellipse_lines, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_hexagon, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_line, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_line_3d, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::step_physics, m)?)?;


    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::next_frame, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::clear_background, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_multiline_text, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_text, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_text_center, m)?)?;
    

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
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::clear_input_queue, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::is_simulating_mouse_with_touch, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::simulate_mouse_with_touch, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::touches, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Mouse::touches_local, m)?)?;
    
    m.add_class::<crate::py_abstractions::Mouse::Touch>()?;
    m.add_class::<crate::py_abstractions::Mouse::TouchPhase>()?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_screen_data, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::screen_height, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::screen_width, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::request_new_screen_size, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::screen_dpi_scale, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::get_last_key_pressed, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_grid, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_texture, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_plane, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_cube, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::polar_to_cartesian, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::cartesian_to_polar, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::set_pc_assets_folder, m)?)?;
    
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::draw_skybox, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::set_fullscreen, m)?)?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::Camera::set_default_camera, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Camera::push_camera_state, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Camera::pop_camera_state, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Camera::camera_font_scale, m)?)?;

    m.add_class::<crate::py_abstractions::Text::Font>()?;
    m.add_class::<crate::py_abstractions::Text::TextDimensions>()?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Text::measure_text, m)?)?;
    
    m.add_class::<crate::py_abstractions::Color::Color>()?;

    m.add_class::<crate::py_abstractions::Loading::ThreadedLoading::Loading>()?;
    m.add_class::<crate::py_abstractions::Loading::FileData::FileData>()?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::load_file, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::load_file_future, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::download_file, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::download_file_future, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Loading::Loading::write_to_file, m)?)?;

    m.add_class::<crate::py_abstractions::GL::Vertex>()?;

    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::is_quit_requested, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::py_functions::prevent_quit, m)?)?;
    
    m.add_class::<crate::py_abstractions::GL::InternalGL>()?;
    m.add_class::<crate::py_abstractions::RenderTarget::RenderTarget>()?;
    m.add_class::<crate::py_abstractions::RenderTarget::RenderTargetParams>()?;
    m.add_class::<crate::py_abstractions::Textures_and_Images::Texture2D>()?;
    m.add_class::<crate::py_abstractions::Textures_and_Images::FilterMode>()?;
    m.add_class::<crate::py_abstractions::Textures_and_Images::Image>()?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::Textures_and_Images::build_texture_atlas, m)?)?;
    m.add_class::<crate::py_abstractions::Camera::Camera2D>()?;
    m.add_class::<crate::py_abstractions::Camera::Camera3D>()?;
    m.add_class::<crate::py_abstractions::Camera::Projection>()?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::RenderTarget::render_target_msaa, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::RenderTarget::render_target, m)?)?;



    m.add_class::<crate::py_abstractions::Audio::PlaySoundParams>()?;
    m.add_class::<crate::py_abstractions::Audio::Sound>()?;

    m.add_class::<crate::py_abstractions::Config::Config>()?;

    m.add_class::<crate::py_abstractions::structs::GLAM::BVec2::BVec2>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::BVec3::BVec3>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::Vec3::Vec3>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::Vec4::Vec4>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::Vec2::Vec2>()?;
    m.add_class::<crate::py_abstractions::structs::GLAM::Mat4::Mat4>()?;


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
    m.add_class::<crate::py_abstractions::Shader::ShaderSource>()?;
    m.add_class::<crate::py_abstractions::KeyCode::KeyCode>()?;

    m.add_class::<crate::py_abstractions::MouseButton::MouseButton>()?;

    m.add_class::<crate::py_abstractions::UniformType::UniformType>()?;
    m.add_class::<crate::py_abstractions::UniformType::EulerRot>()?;
    m.add_class::<crate::py_abstractions::UniformType::Comparison>()?;


    m.add_class::<crate::py_abstractions::PFuture::ImageFuture>()?;
    m.add_class::<crate::py_abstractions::PFuture::FileDataFuture>()?;
    m.add_class::<crate::py_abstractions::PFuture::Future>()?;
    //m.add_class::<crate::py_abstractions::PFuture::Timeout>()?;
    //m.add_class::<crate::py_abstractions::PFuture::EmptyFuture>()?;
    
    Ok(())
}

define_stub_info_gatherer!(stub_info);


/*
list of macroquad::prelude functions

    mq::get_dropped_files
    mq::gl_use_default_material
    mq::gl_use_material
    mq::load_material
*/