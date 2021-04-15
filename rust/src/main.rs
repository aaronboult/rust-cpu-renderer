mod display;
use display::{Display};
use display::drawing::{PixelBufferDrawer, CanvasDrawer, Color};
use display::events::{MouseMoveEvent, MouseInputEvent, MouseWheelEvent, KeyboardEvent};

use std::{thread, time};

use sdl2::mouse::{MouseButton, MouseWheelDirection};

fn main(){

    let mut d = Display::<PixelBufferDrawer>::new(512, 512);

    d.set_title("Yo yo yo");

    d.set_refresh_rate(100);
    d.show_frame_rate(false);

    d.add_mouse_move_handler(t_mouse_move).unwrap();
    d.add_mouse_input_handler(t_mouse_input).unwrap();
    d.add_mouse_wheel_handler(t_mouse_wheel).unwrap();
    d.add_keyboard_handler(t_keyboard).unwrap();

    d.open();

    thread::sleep(time::Duration::from_secs(5));

    d.fill(
        Color::rgb(255, 100, 40)
    );

    d.await_close();

}

fn t_mouse_move(e: &MouseMoveEvent) -> Result<(), ()>{
    println!("Mouse Pos: ({}, {}), Rel: ({}, {})", e.x, e.y, e.x_direction, e.y_direction);
    Ok(())
}

fn t_mouse_input(e: &MouseInputEvent) -> Result<(), ()> {
    println!("{:?}", e.input_type);
    println!("\tPos: ({}, {}), Clicks: {}", e.x, e.y, e.clicks);
    match e.button {
        MouseButton::Left => {
            println!("\t\tLeft Button Clicked");
        },
        MouseButton::Right => {
            println!("\t\tRight Button Clicked");
        },
        MouseButton::Middle => {
            println!("\t\tMiddle Button Clicked");
        },
        MouseButton::X1 => {
            println!("\t\tX1 Button Clicked");
        },
        MouseButton::X2 => {
            println!("\t\tX2 Button Clicked");
        },
        MouseButton::Unknown => {
            println!("\t\tUnknown Clicked");
        },
        _ => {}
    }
    Ok(())
}

fn t_mouse_wheel(e: &MouseWheelEvent) -> Result<(), ()> {
    println!("Mouse Wheel, Pos: ({}, {})", e.x, e.y);
    match e.direction {
        MouseWheelDirection::Normal => {
            println!("\tDirection: Normal");
        },
        MouseWheelDirection::Flipped => {
            println!("\tDirection: Flipped")
        },
        MouseWheelDirection::Unknown(x) => {
            println!("\tDirection: Unknown: {}", x);
        },
        _ => {}
    }
    Ok(())
}

fn t_keyboard(e: &KeyboardEvent) -> Result<(), ()> {
    println!("Keycode: {}, Scancode: {}, Keymod: {}, InputType: {:?}", e.keycode, e.scancode, e.keymod, e.input_type);
    Ok(())
}