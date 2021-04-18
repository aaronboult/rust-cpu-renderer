#![allow(dead_code)]

extern crate sdl2;

pub mod drawing;
use drawing::{Drawer, DrawMode, Color};

pub mod events;
use events::*;

// sdl2
use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode};

// timekeeping
use std::{time};
use time::{Instant, Duration};

pub type HandlerResult = Result<(), ()>;


// calls all handlers in $handlers until one returns an err
// which tells the caller to stop propogating the event
macro_rules! call_event_handlers {
    ($($handlers:ident).+, $event:ident) => (
        for i in 0..$($handlers).+.len() {
            // break if true is returned, indicating propogation should halt
            if $($handlers).+[i](&$event).is_err() {
                break;
            }
        }
    )
}

//#region Screen
// the screen struct - holds all information necessary to
// draw the window and dispatch events
pub struct Screen {
    open: bool,
    width: u32,
    height: u32,
    title: &'static str,
    frame_delay: Duration,
    show_fps: bool,
    draw_handler: Drawer,
    mouse_move_handlers: Vec<fn(e: &MouseMoveEvent) -> HandlerResult>,
    mouse_input_handlers: Vec<fn(e: &MouseInputEvent) -> HandlerResult>,
    mouse_wheel_handlers: Vec<fn(e: &MouseWheelEvent) -> HandlerResult>,
    keyboard_handlers: Vec<fn(e: &KeyboardEvent) -> HandlerResult>,
    canvas: WindowCanvas,
    event_pump: EventPump,
    frame_start_time: Option<Instant>,
    fps_timer: Option<Instant>,
    frame_count: u32
}

impl Screen {
    const SECOND_DURATION: Duration = Duration::from_secs(1);

    pub fn new() -> ScreenBuilder {
        ScreenBuilder::new()
    }

    pub fn refresh(&mut self) {
        if self.open {
    
            // set up values to maintain frame rate
            self.fps_timer = Some(Instant::now());
    
            // only draw the frame if enough time has passed to maintain the frame rate
            if self.frame_start_time.unwrap().elapsed() >= self.frame_delay {
                self.frame_start_time = Some(Instant::now());
                self.frame_count += 1;

                self.draw_handler.draw(&mut self.canvas);
            }

            if self.fps_timer.unwrap().elapsed() >= Screen::SECOND_DURATION && self.show_fps {
                println!("Frames elapsed: {}", self.frame_count);
                self.frame_count = 0;
                self.fps_timer = Some(Instant::now());
            }

            self.handle_events();

        }

    }

    fn handle_events(&mut self) {
        // dispatch events
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    self.open = false;
                },

                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(new_width, new_height) => {
                            println!("Resize");
                            self.width = new_width as u32;
                            self.height = new_height as u32;
                        },
                        _ => {}
                    }
                },

                Event::KeyDown { keycode, scancode, keymod, repeat, .. } => {
                    let keyboard_event_type: KeyboardEventType = if repeat {
                        KeyboardEventType::KEYDOWN
                    }
                    else{
                        KeyboardEventType::KEYHELD 
                    };
                    let keydown_event = KeyboardEvent::new(keycode.unwrap(), scancode.unwrap(), keymod, keyboard_event_type);
                    call_event_handlers!(self.keyboard_handlers, keydown_event);
                },

                Event::KeyUp { keycode, scancode, keymod, repeat, .. } => {
                    let keyboard_event_type: KeyboardEventType = if repeat {
                        KeyboardEventType::KEYUP
                    }
                    else{
                        KeyboardEventType::KEYHELD 
                    };
                    let keydown_event = KeyboardEvent::new(keycode.unwrap(), scancode.unwrap(), keymod, keyboard_event_type);
                    call_event_handlers!(self.keyboard_handlers, keydown_event);
                },

                Event::MouseMotion { x, y, xrel, yrel, mousestate, .. } => {
                    let mouse_move_event = MouseMoveEvent::new(x, y, xrel, yrel, mousestate);
                    call_event_handlers!(self.mouse_move_handlers, mouse_move_event);
                },

                Event::MouseButtonUp { x, y, clicks, mouse_btn, .. } => {
                    let mouse_input_event = MouseInputEvent::new(x, y, clicks, mouse_btn, MouseInputType::MOUSEUP);
                    call_event_handlers!(self.mouse_input_handlers, mouse_input_event);
                },

                Event::MouseButtonDown { x, y, clicks, mouse_btn, .. } => {
                    let mouse_input_event = MouseInputEvent::new(x, y, clicks, mouse_btn, MouseInputType::MOUSEDOWN);
                    call_event_handlers!(self.mouse_input_handlers, mouse_input_event);
                },

                Event::MouseWheel { x, y, direction, .. } => {
                    let mouse_wheel_event = MouseWheelEvent::new(x, y, direction);
                    call_event_handlers!(self.mouse_wheel_handlers, mouse_wheel_event);
                },

                _ => {}
            }
        }
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) {
        self.draw_handler.clear();
    }

    pub fn show_frame_rate(&mut self, show: bool) {
        self.show_fps = show;
    }

    pub fn set_title(&mut self, new_title: &'static str) {
        self.title = new_title;
    }

    pub fn set_refresh_rate(&mut self, new_rate: u64) {
        self.frame_delay = Duration::from_millis(1000 / new_rate);
    }

    pub fn get_draw_color(&self) -> Color {
        self.draw_handler.get_draw_color()
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.draw_handler.set_draw_color(color);
    }

    pub fn draw_point(&mut self, point: (i32, i32)) {
        self.draw_handler.draw_point(point);
    }

    pub fn draw_line(&mut self, point_a: (i32, i32), point_b: (i32, i32)) {
        self.draw_handler.draw_line(point_a, point_b);
    }

    pub fn fill(&mut self, color: Color) {
        self.draw_handler.fill(color);
    }

    pub fn add_mouse_move_handler(&mut self, handler: fn(e: &MouseMoveEvent) -> HandlerResult) -> Result<(), &'static str> {
        self.mouse_move_handlers.push(handler);
        Ok(())
    }

    pub fn add_mouse_input_handler(&mut self, handler: fn(e: &MouseInputEvent) -> HandlerResult) -> Result<(), &'static str> {
        self.mouse_input_handlers.push(handler);
        Ok(())
    }

    pub fn add_mouse_wheel_handler(&mut self, handler: fn(e: &MouseWheelEvent) -> HandlerResult) -> Result<(), &'static str> {
        self.mouse_wheel_handlers.push(handler);
        Ok(())
    }

    pub fn add_keyboard_handler(&mut self, handler: fn(e: &KeyboardEvent) -> HandlerResult) -> Result<(), &'static str> {
        self.keyboard_handlers.push(handler);
        Ok(())
    }

    pub fn open(&mut self) {
        self.frame_start_time = Some(Instant::now());
        self.fps_timer = Some(Instant::now());
        self.open = true;
        self.refresh();
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}
//#endregion

//#region ScreenBuilder
pub struct ScreenBuilder {
    pub width: u32,
    pub height: u32,
    pub mode: DrawMode,
    pub title: &'static str,
    pub refresh_rate: u16
}

impl ScreenBuilder {
    pub fn new() -> Self {
        Self {
            width: 512,
            height: 512,
            mode: DrawMode::CANVAS,
            title: "Simulation Engine",
            refresh_rate: 60
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn set_refresh_rate(&mut self, rate: u16) -> &mut Self {
        self.refresh_rate = rate;
        self
    }

    pub fn use_pixel_buffer(&mut self) -> &mut Self {
        self.mode = DrawMode::PIXELBUFFER;
        self
    }

    pub fn use_canvas_drawer(&mut self) -> &mut Self {
        self.mode = DrawMode::CANVAS;
        self
    }

    pub fn build(&self) -> Screen {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window(&self.title, self.width, self.height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = context.event_pump().unwrap();
        Screen {
            open: false,
            width: self.width,
            height: self.height,
            title: "Simulation Engine",
            frame_delay: Duration::from_millis(1000 / 60),
            show_fps: false,
            draw_handler: Drawer::new(Color::WHITE, self.mode),
            mouse_move_handlers: Vec::new(),
            mouse_input_handlers: Vec::new(),
            mouse_wheel_handlers: Vec::new(),
            keyboard_handlers: Vec::new(),
            canvas,
            event_pump,
            frame_start_time: None,
            fps_timer: None,
            frame_count: 0
        }
    }
}
//#endregion