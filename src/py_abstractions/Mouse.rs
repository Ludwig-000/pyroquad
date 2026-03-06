use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use crate::engine::PChannel::PChannel;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use std::collections::HashSet;
use crate::py_abstractions::MouseButton::MouseButton;

#[gen_stub_pyfunction]
#[pyfunction]
pub fn set_cursor_grab(option: bool)  {
    COMMAND_QUEUE.push(Command::SetCursorGrab(option));
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn show_mouse(option: bool) {
    COMMAND_QUEUE.push(Command::ShowMouse(option));
}



#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_buttons_down() -> HashSet<MouseButton> {

    use crate::engine::FrameInfo as fi;
    fi::MOUSE_BUTTON_DOWN.lock().unwrap()
        .iter()
        .map(|key| (*key).into())
        .collect()
}
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_buttons_pressed() -> HashSet<MouseButton> {

    use crate::engine::FrameInfo as fi;
    fi::MOUSE_BUTTON_PRESSED.lock().unwrap()
        .iter()
        .map(|key| (*key).into())
        .collect()
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_buttons_released() -> HashSet<MouseButton> {

    use crate::engine::FrameInfo as fi;
    fi::MOUSE_BUTTON_RELEASED.lock().unwrap()
        .iter()
        .map(|key| (*key).into())
        .collect()
}





#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_wheel() -> Vec2 {
    use crate::engine::FrameInfo as fi;
    let wheel = *fi::MOUSE_WHEEL.lock().unwrap();
    Vec2::const_new(wheel.0, wheel.1)
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_position() -> Vec2 {
    use crate::engine::FrameInfo as fi;
    let mouse =*fi::MOUSE_POSITION.lock().unwrap();
    Vec2::const_new(mouse.0, mouse.1)
}

/// Return mouse position in range [-1; 1].
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_position_local() -> Vec2 {
    use crate::engine::FrameInfo as fi;
    let mouse = *fi::MOUSE_POSITION_LOCAL.lock().unwrap();
    mouse.into()
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_mouse_delta_position() -> Vec2 {
    use crate::engine::FrameInfo as fi;
    let mouse = *fi::MOUSE_DELTA_POSITION.lock().unwrap();
    mouse.into()
}

/// Clears input queue
#[gen_stub_pyfunction]
#[pyfunction]
pub fn clear_input_queue() {
    COMMAND_QUEUE.push(  Command::ClearInputQueue );
}

/// Clears input queue
#[gen_stub_pyfunction]
#[pyfunction]
pub fn is_simulating_mouse_with_touch()-> PyResult<bool> {
    let (sender, receiver) = PChannel::sync_channel(1);
    COMMAND_QUEUE.push(  Command::IsSimulatingMouseWithTouch(sender));
    Ok(receiver.recv()?)
}
