use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};
use std::sync::atomic::{AtomicI32,Ordering};
use macroquad::input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released};
use macroquad::prelude as mq;
use macroquad::prelude::KeyCode;

use mq::Vec2;

use crate::engine::AtomicF32::AtomicF32;


/// Some information about the state of the engine, that is only updates once a frame,
/// meaning it can be stored safely inside statics.
pub static DELTA_TIME: AtomicF32  = AtomicF32::new(0.);
pub static FPS: AtomicI32 =  AtomicI32::new(0);

pub static MOUSE_POSITION: Mutex<(f32,f32)> = Mutex::new((0.,0.));
pub static MOUSE_DELTA_POSITION: Mutex<Vec2> = Mutex::new(Vec2::new(0.,0.));
pub static MOUSE_POSITION_LOCAL: Mutex<Vec2> = Mutex::new(Vec2::new(0.,0.));
pub static MOUSE_WHEEL: Mutex<(f32,f32)> = Mutex::new((0.,0.));

pub static SCREEN_HEIGHT: AtomicF32 = AtomicF32::new(0.);
pub static SCREEN_WIDTH: AtomicF32 = AtomicF32::new(0.);



pub static LASK_KEY_PRESSED: Mutex<Option<KeyCode>> = Mutex::new(None);
pub static CHAR_PRESSED: Mutex<Option<char>> = Mutex::new(None);

pub static KEYS_PRESSED: LazyLock<Mutex<HashSet<KeyCode>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub static KEYS_DOWN: LazyLock<Mutex<HashSet<KeyCode>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub static KEYS_RELEASED: LazyLock<Mutex<HashSet<KeyCode>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub static MOUSE_BUTTON_PRESSED: LazyLock<Mutex<HashSet<MouseButton>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub static MOUSE_BUTTON_DOWN: LazyLock<Mutex<HashSet<MouseButton>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub static MOUSE_BUTTON_RELEASED: LazyLock<Mutex<HashSet<MouseButton>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// this function should be run by 'next_frame'
pub fn update_frame_info(){
    let fps =  mq::get_fps();
    FPS.store(fps, Ordering::Relaxed);

    DELTA_TIME.store(mq::get_frame_time(), Ordering::Relaxed);
    {
        *KEYS_PRESSED.lock().unwrap() =   mq::get_keys_pressed();
        *KEYS_DOWN.lock().unwrap() = mq::get_keys_down();
        *KEYS_RELEASED.lock().unwrap() = mq::get_keys_released();
        *LASK_KEY_PRESSED.lock().unwrap() = mq::get_last_key_pressed();
        *CHAR_PRESSED.lock().unwrap() = mq::get_char_pressed();

        *MOUSE_POSITION.lock().unwrap() = mq::mouse_position();
        *MOUSE_DELTA_POSITION.lock().unwrap() = mq::mouse_delta_position();
        *MOUSE_POSITION_LOCAL.lock().unwrap() = mq::mouse_position_local();
        *MOUSE_WHEEL.lock().unwrap() = mq::mouse_wheel();

        SCREEN_HEIGHT.store(mq::screen_height(), Ordering::Relaxed);
        SCREEN_WIDTH.store(mq::screen_width(), Ordering::Relaxed);
        {
            let left = is_mouse_button_down(MouseButton::Left);
            let middle = is_mouse_button_down(MouseButton::Middle);
            let right = is_mouse_button_down(MouseButton::Right);
            let unknown = is_mouse_button_down(MouseButton::Unknown);

            let active_buttons: HashSet<MouseButton> = [
                (left, MouseButton::Left),
                (middle, MouseButton::Middle),
                (right, MouseButton::Right),
                (unknown, MouseButton::Unknown),
            ]
            .into_iter()
            .filter(|(is_down, _)| *is_down)
            .map(|(_, button)| button)
            .collect();

            *MOUSE_BUTTON_DOWN.lock().unwrap() = active_buttons;
        }
        {
            let left = is_mouse_button_pressed(MouseButton::Left);
            let middle = is_mouse_button_pressed(MouseButton::Middle);
            let right = is_mouse_button_pressed(MouseButton::Right);
            let unknown = is_mouse_button_pressed(MouseButton::Unknown);

            let active_buttons: HashSet<MouseButton> = [
                (left, MouseButton::Left),
                (middle, MouseButton::Middle),
                (right, MouseButton::Right),
                (unknown, MouseButton::Unknown),
            ]
            .into_iter()
            .filter(|(is_down, _)| *is_down)
            .map(|(_, button)| button)
            .collect();

            *MOUSE_BUTTON_PRESSED.lock().unwrap() = active_buttons;
        }
        {
            let left = is_mouse_button_released(MouseButton::Left);
            let middle = is_mouse_button_released(MouseButton::Middle);
            let right = is_mouse_button_released(MouseButton::Right);
            let unknown = is_mouse_button_released(MouseButton::Unknown);

            let active_buttons: HashSet<MouseButton> = [
                (left, MouseButton::Left),
                (middle, MouseButton::Middle),
                (right, MouseButton::Right),
                (unknown, MouseButton::Unknown),
            ]
            .into_iter()
            .filter(|(is_down, _)| *is_down)
            .map(|(_, button)| button)
            .collect();

            *MOUSE_BUTTON_RELEASED.lock().unwrap() = active_buttons;
        }
        
    }
}

pub fn clear_keys_pressed() {
    if let Ok(mut last_key) = LASK_KEY_PRESSED.lock() {
        *last_key = None;
    }
    
    if let Ok(mut char_pressed) = CHAR_PRESSED.lock() {
        *char_pressed = None;
    }

    if let Ok(mut keys_pressed) = KEYS_PRESSED.lock() {
        keys_pressed.clear();
    }

    if let Ok(mut mouse_pressed) = MOUSE_BUTTON_PRESSED.lock() {
        mouse_pressed.clear();
    }
}