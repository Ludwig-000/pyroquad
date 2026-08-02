use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;
use pyo3_stub_gen::derive::gen_stub_pyfunction;
use pyo3_stub_gen::derive::gen_stub_pymethods;

use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use crate::engine::FrameInfo::clear_keys_pressed;
use crate::engine::PChannel::PChannel;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use std::collections::HashSet;
use crate::py_abstractions::MouseButton::MouseButton;


#[gen_stub_pyfunction]
#[pyfunction]
pub fn mouse_inside_window() -> PyResult<bool>  {
    let (sender, receiver) = PChannel::sync_channel(1);
    COMMAND_QUEUE.push(  Command::GetCustomMouseState { sender });
    Ok(receiver.recv()?.2)
}

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
    // TOOD: make sure the internal macroquad buffer stays synced with our custom buffer!!!!!!
    clear_keys_pressed();
    COMMAND_QUEUE.push(  Command::ClearInputQueue ); // THIS DOES NOT PROPERLY CLEAR THE INPUT QUEUE!!!
}


/// This is set to true by default, meaning touches will raise mouse events in addition to raising touch events.
/// If set to false, touches won't affect mouse events.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn is_simulating_mouse_with_touch()-> PyResult<bool> {
    let (sender, receiver) = PChannel::sync_channel(1);
    COMMAND_QUEUE.push(  Command::IsSimulatingMouseWithTouch(sender));
    Ok(receiver.recv()?)
}



/// This is set to true by default, meaning touches will raise mouse events in addition to raising touch events.
/// If set to false, touches won't affect mouse events.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn simulate_mouse_with_touch(option: bool) {
    COMMAND_QUEUE.push( Command::SimulateMouseWithTouch(option));
}


/// Return touches with positions in pixels.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn touches() -> PyResult<Vec<Touch>> {
    let (sender, receiver) = PChannel::sync_channel(1);
    COMMAND_QUEUE.push(  Command::Touches(sender));
    let touches = receiver.recv()?;
    Ok(touches.into_iter().map( Into::into ).collect())
}


/// Return touches with positions in range [-1; 1].
#[gen_stub_pyfunction]
#[pyfunction]
pub fn touches_local() -> PyResult<Vec<Touch>> {
    let (sender, receiver) = PChannel::sync_channel(1);
    COMMAND_QUEUE.push(  Command::TouchesLocal(sender));
    let touches = receiver.recv()?;
    Ok(
        touches.into_iter().map( Into::into ).collect()
    )
}


#[gen_stub_pyclass_enum]
#[pyclass(from_py_object)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchPhase{
    Started,
    Stationary,
    Moved,
    Ended,
    Cancelled,
}
impl From<macroquad::prelude::TouchPhase> for TouchPhase{
    fn from(value: macroquad::prelude::TouchPhase) -> Self {
        match value{
            macroquad::input::TouchPhase::Cancelled => TouchPhase::Cancelled,
            macroquad::input::TouchPhase::Ended => TouchPhase::Cancelled,
            macroquad::input::TouchPhase::Moved => TouchPhase::Moved,
            macroquad::input::TouchPhase::Started => TouchPhase::Started,
            macroquad::input::TouchPhase::Stationary => TouchPhase::Stationary,
        }
    }
}
impl From<TouchPhase> for macroquad::prelude::TouchPhase{
    fn from(value: TouchPhase) -> Self {
        match value{
            TouchPhase::Cancelled => macroquad::input::TouchPhase::Cancelled,
            TouchPhase::Ended => macroquad::input::TouchPhase::Ended,
            TouchPhase::Moved => macroquad::input::TouchPhase::Moved,
            TouchPhase::Started => macroquad::input::TouchPhase::Started,
            TouchPhase::Stationary => macroquad::input::TouchPhase::Stationary,
        }
    }
}


#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy)]
pub struct Touch{
    #[pyo3(get,set)]
    pub id: u64,
    #[pyo3(get,set)]
    pub phase: TouchPhase,
    #[pyo3(get,set)]
    pub position: Vec2,
}

#[gen_stub_pymethods]
#[pymethods]
impl Touch{
    #[new]
    pub fn new(id: u64, phase: TouchPhase, position: Vec2) -> Touch{
        Touch{id, phase, position}
    }
}


impl From<macroquad::prelude::Touch> for Touch{
    fn from(value: macroquad::prelude::Touch) -> Self {
        Touch{
            id: value.id,
            phase: value.phase.into(),
            position: value.position.into(),
        }
    }
}
impl From<Touch> for macroquad::prelude::Touch{
    fn from(value: Touch) -> Self {
        macroquad::prelude::Touch{
            id: value.id,
            phase: value.phase.into(),
            position: value.position.into(),
        }
    }
}