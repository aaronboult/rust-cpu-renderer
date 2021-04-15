use sdl2::mouse::{MouseButton, MouseState, MouseWheelDirection};
use sdl2::keyboard::{Keycode, Scancode, Mod};

pub struct MouseMoveEvent {
    pub x: i32,
    pub y: i32,
    pub x_direction: i32,
    pub y_direction: i32,
    pub mousestate: MouseState
}

impl MouseMoveEvent {
    pub fn new(x: i32, y: i32, x_direction: i32, y_direction: i32, mousestate: MouseState) -> Self {
        Self { x, y, x_direction, y_direction, mousestate }
    }
}

pub struct MouseInputEvent {
    pub x: i32,
    pub y: i32,
    pub clicks: u8,
    pub button: MouseButton,
    pub input_type: MouseInputType
}

impl MouseInputEvent {
    pub fn new(x: i32, y: i32, clicks: u8, button: MouseButton, input_type: MouseInputType) -> Self {
        Self { x, y, clicks, button, input_type }
    }
}

#[derive(Debug)]
pub enum MouseInputType {
    MOUSEUP,
    MOUSEDOWN
}

pub struct MouseWheelEvent {
    pub x: i32,
    pub y: i32,
    pub direction: MouseWheelDirection
}

impl MouseWheelEvent {
    pub fn new(x: i32, y: i32, direction: MouseWheelDirection) -> Self {
        Self { x, y, direction }
    }
}

pub struct KeyboardEvent {
    pub keycode: Keycode,
    pub scancode: Scancode,
    pub keymod: Mod,
    pub input_type: KeyboardEventType
}

impl KeyboardEvent {
    pub fn new(keycode: Keycode, scancode: Scancode, keymod: Mod, input_type: KeyboardEventType) -> Self{
        Self { keycode, scancode, keymod, input_type }
    }
}

#[derive(Debug)]
pub enum KeyboardEventType {
    KEYUP,
    KEYDOWN,
    KEYHELD
}