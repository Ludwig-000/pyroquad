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



use pyo3::prelude::*;


mod engine;
mod py_abstractions;


#[pyo3::pymodule]
pub mod _pyroquad {

    #[pyo3::pymodule]
    #[pyo3(gil_used = false)]
    pub mod InternalGL {
        #[pymodule_export]
        pub use crate::py_abstractions::GL::{
            DrawMode, Geometry, GlPipeline, Vertex,
        };

        #[pymodule_export]
        pub use crate::py_abstractions::GL::{
            clear_draw_calls, delete_pipeline, depth_test, draw_mode, geometry,
            get_viewport, get_viewport_matrix, is_depth_test_enabled, pipeline,
            pop_model_matrix, push_model_matrix, reset, scissor, set_texture,
            texture, viewport,
        };
    }

    #[pymodule_export]
    pub use crate::py_abstractions::py_functions::{
        activate_engine, batch_draw_shapes, cartesian_to_polar, clear_background, draw_affine_parallelepiped,
        draw_affine_parallelogram, draw_all_objects, draw_arc, draw_circle, draw_circle_lines, draw_cube,
        draw_cube_wires, draw_cylinder, draw_cylinder_wires, draw_ellipse, draw_ellipse_lines, draw_grid,
        draw_hexagon, draw_line, draw_line_3d, draw_multiline_text, draw_plane, draw_poly, draw_poly_lines,
        draw_rectangle, draw_rectangle_lines, draw_skybox, draw_text, draw_texture, draw_triangle,
        draw_triangle_lines, get_char_pressed, get_delta_time, get_fps, get_keys_down, get_keys_pressed,
        get_keys_released, get_last_key_pressed, get_screen_data, get_text_center, is_quit_requested, next_frame,
        polar_to_cartesian, prevent_quit, request_new_screen_size, screen_dpi_scale, screen_height, screen_width,
        set_fullscreen, set_pc_assets_folder, step_physics,
    };

    #[pymodule_export]
    pub use crate::py_abstractions::Mouse::{
        clear_input_queue, get_mouse_buttons_down, get_mouse_buttons_pressed, get_mouse_buttons_released,
        get_mouse_delta_position, get_mouse_position, get_mouse_position_local, get_mouse_wheel,
        is_simulating_mouse_with_touch, mouse_inside_window, set_cursor_grab, show_mouse, simulate_mouse_with_touch,
        touches, touches_local, Touch, TouchPhase,
    };

    #[pymodule_export]
    pub use crate::py_abstractions::Camera::{
        camera_font_scale, pop_camera_state, push_camera_state, set_default_camera, Camera2D, Camera3D, Projection,
    };

    #[pymodule_export]
    pub use crate::py_abstractions::Text::{
        measure_text, Font, TextDimensions,
    };

    #[pymodule_export]
    pub use crate::py_abstractions::Color::Color;

    #[pymodule_export]
    pub use crate::py_abstractions::Loading::ThreadedLoading::Loading;
    #[pymodule_export]
    pub use crate::py_abstractions::Loading::FileData::FileData;
    #[pymodule_export]
    pub use crate::py_abstractions::Loading::Loading::{
        download_file, download_file_future, load_file, load_file_future, write_to_file,
    };

    

    #[pymodule_export]
    pub use crate::py_abstractions::RenderTarget::{
        render_target, render_target_msaa, RenderTarget, RenderTargetParams,
    };

    #[pymodule_export]
    pub use crate::py_abstractions::Textures_and_Images::{
        build_texture_atlas, FilterMode, Image, Texture2D,
    };

    #[pymodule_export]
    pub use crate::py_abstractions::Audio::{PlaySoundParams, Sound};

    #[pymodule_export]
    pub use crate::py_abstractions::Config::Config;



    #[pymodule_export]
    pub use crate::py_abstractions::structs::GLAM::{
        BVec2::BVec2, BVec3::BVec3, Mat4::Mat4, Quat::Quat, Vec2::Vec2, Vec3::Vec3, Vec4::Vec4};


    #[pymodule_export]
    pub use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::ColliderOptions;
    #[pymodule_export]
    pub use crate::py_abstractions::structs::ThreeDObjects::{
        Cube::Cube, Cylinder::Cylinder, Mesh::Mesh, PhysicsHandle::Physics, Pill::Pill, Sphere::Sphere};


    #[pymodule_export]
    pub use crate::py_abstractions::structs::TwoDObjects::Circle::Circle;
    #[pymodule_export]
    pub use crate::py_abstractions::structs::TwoDObjects::Rectangle::Rectangle;

    #[pymodule_export]
    pub use crate::py_abstractions::Shader::{Shader, ShaderSource};

    #[pymodule_export]
    pub use crate::py_abstractions::KeyCode::KeyCode;
    #[pymodule_export]
    pub use crate::py_abstractions::MouseButton::MouseButton;

    #[pymodule_export]
    pub use crate::py_abstractions::UniformType::{Comparison, EulerRot, UniformType};

    #[pymodule_export]
    pub use crate::py_abstractions::PFuture::{FileDataFuture, Future, ImageFuture};
}




//m.add_class::<crate::py_abstractions::PFuture::Timeout>()?;
//m.add_class::<crate::py_abstractions::PFuture::EmptyFuture>()?;

/*
list of macroquad::prelude functions

    mq::get_dropped_files
    mq::gl_use_default_material
    mq::gl_use_material
    mq::load_material
*/