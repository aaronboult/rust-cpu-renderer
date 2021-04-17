#![allow(dead_code)]

extern crate sdl2;

pub mod drawing;
use drawing::{Drawable, Color};

pub mod events;
use events::{MouseMoveEvent, MouseInputEvent, MouseWheelEvent, KeyboardEvent, MouseInputType, KeyboardEventType};

// sdl2
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode};

// threading
use std::{thread, time};
use time::{Instant, Duration};

// thread data sharing 
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::marker::Send;

pub type HandlerResult = Result<(), ()>;

// calls all handlers in $handlers until one returns an err
// which tells the caller to stop propogating the event
macro_rules! call_event_handlers {
    ($handlers:ident, $event:ident) => (
        let handlers_lock = $handlers.lock().unwrap();
        for i in 0..handlers_lock.len() {
            // break if true is returned, indicating propogation should halt
            if handlers_lock[i](&$event).is_err() {
                break;
            }
        }
    )
}

// the screen struct - holds all information necessary to
// draw the window and dispatch events
struct Screen<T> where T: Drawable + Send + Sync + 'static {
    open: Arc<AtomicBool>,
    width: Arc<AtomicU32>,
    height: Arc<AtomicU32>,
    handle: Option<thread::JoinHandle<()>>,
    title: Arc<String>,
    frame_delay: Arc<Mutex<Duration>>,
    show_fps: Arc<AtomicBool>,
    draw_handler: Arc<Mutex<T>>,
    mouse_move_handlers: Arc<Mutex<Vec<fn(e: &MouseMoveEvent) -> HandlerResult>>>,
    mouse_input_handlers: Arc<Mutex<Vec<fn(e: &MouseInputEvent) -> HandlerResult>>>,
    mouse_wheel_handlers: Arc<Mutex<Vec<fn(e: &MouseWheelEvent) -> HandlerResult>>>,
    keyboard_handlers: Arc<Mutex<Vec<fn(e: &KeyboardEvent) -> HandlerResult>>>
}

impl<T> Screen<T> where T: Drawable + Send + Sync + 'static {
    fn show(&mut self) {

        // clone all Arc references to be used within the mainloop thread
        let open = self.open.clone();
        let width = self.width.clone();
        let height = self.height.clone();
        let title = self.title.clone();
        let frame_delay = self.frame_delay.clone();
        let show_fps = self.show_fps.clone();

        // clone handler vector references
        let draw_handler = self.draw_handler.clone();
        let mouse_move_handlers = self.mouse_move_handlers.clone();
        let mouse_input_handlers = self.mouse_input_handlers.clone();
        let mouse_wheel_handlers = self.mouse_wheel_handlers.clone();
        let keyboard_handlers = self.keyboard_handlers.clone();

        self.handle = Some(
            thread::spawn(
                move || {
                    let context = sdl2::init().unwrap();
                    let video = context.video().unwrap();
                    let window = video.window(&title, width.load(Ordering::SeqCst), height.load(Ordering::SeqCst))
                        .position_centered()
                        .resizable()
                        .build()
                        .unwrap();
                    let mut canvas = window.into_canvas().build().unwrap();
                    let mut event_pump = context.event_pump().unwrap();

                    let mut frame_start_time = Instant::now();

                    // set up values to maintain frame rate
                    let mut frame_count: u64 = 0;
                    let mut fps_timer = Instant::now();
                    const SECOND_DURATION: Duration = Duration::from_secs(1);

                    'main: loop {
                        if !open.load(Ordering::SeqCst) {
                            break 'main;
                        }

                        // only draw the frame if enough time has passed to maintain the frame rate
                        if frame_start_time.elapsed() >= {*frame_delay.lock().unwrap()} {
                            let mut handler_guard = draw_handler.lock().unwrap();

                            canvas.set_draw_color(handler_guard.get_bg_color());
                            canvas.clear();

                            frame_start_time = Instant::now();

                            frame_count += 1;

                            handler_guard.draw(&mut canvas);

                            canvas.present();
                        }

                        if fps_timer.elapsed() >= SECOND_DURATION && show_fps.load(Ordering::SeqCst) {
                            println!("Frames elapsed: {}", frame_count);
                            frame_count = 0;
                            fps_timer = Instant::now();
                        }

                        // dispatch events
                        for event in event_pump.poll_iter() {
                            match event {
                                Event::Quit {..} |
                                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                                    open.store(false, Ordering::SeqCst);
                                },

                                Event::Window { win_event, .. } => {
                                    match win_event {
                                        WindowEvent::Resized(new_width, new_height) => {
                                            width.store(new_width as u32, Ordering::SeqCst);
                                            height.store(new_height as u32, Ordering::SeqCst);
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
                                    call_event_handlers!(keyboard_handlers, keydown_event);
                                },

                                Event::KeyUp { keycode, scancode, keymod, repeat, .. } => {
                                    let keyboard_event_type: KeyboardEventType = if repeat {
                                        KeyboardEventType::KEYUP
                                    }
                                    else{
                                        KeyboardEventType::KEYHELD 
                                    };
                                    let keydown_event = KeyboardEvent::new(keycode.unwrap(), scancode.unwrap(), keymod, keyboard_event_type);
                                    call_event_handlers!(keyboard_handlers, keydown_event);
                                },

                                Event::MouseMotion { x, y, xrel, yrel, mousestate, .. } => {
                                    let mouse_move_event = MouseMoveEvent::new(x, y, xrel, yrel, mousestate);
                                    call_event_handlers!(mouse_move_handlers, mouse_move_event);
                                },

                                Event::MouseButtonUp { x, y, clicks, mouse_btn, .. } => {
                                    let mouse_input_event = MouseInputEvent::new(x, y, clicks, mouse_btn, MouseInputType::MOUSEUP);
                                    call_event_handlers!(mouse_input_handlers, mouse_input_event);
                                },

                                Event::MouseButtonDown { x, y, clicks, mouse_btn, .. } => {
                                    let mouse_input_event = MouseInputEvent::new(x, y, clicks, mouse_btn, MouseInputType::MOUSEDOWN);
                                    call_event_handlers!(mouse_input_handlers, mouse_input_event);
                                },

                                Event::MouseWheel { x, y, direction, .. } => {
                                    let mouse_wheel_event = MouseWheelEvent::new(x, y, direction);
                                    call_event_handlers!(mouse_wheel_handlers, mouse_wheel_event);
                                },

                                _ => {}
                            }
                        }
                    }
                }
            )
        );
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width.load(Ordering::SeqCst), self.height.load(Ordering::SeqCst))
    }
}

// Wraps the Screen struct and provides an API to interact with it
pub struct Display<T> where T: Drawable + Send + Sync + 'static {
    screen: Screen<T>
}

impl<T> Display<T> where T: Drawable + Send + Sync + 'static {
    pub fn new(width: u32, height: u32) -> Display<T>{
        return Display::<T> {
            screen: Screen::<T> {
                open: Arc::new(AtomicBool::new(false)),
                width: Arc::new(AtomicU32::new(width)),
                height: Arc::new(AtomicU32::new(height)),
                handle: None,
                title: Arc::new(String::from("Simulation Engine")),
                frame_delay: Arc::new(Mutex::new(Duration::from_millis(1000 / 60))),
                show_fps: Arc::new(AtomicBool::new(false)),
                draw_handler: Arc::new(Mutex::new(T::new(Color::rgb(255, 255, 255)))),
                mouse_move_handlers: Arc::new(Mutex::new(Vec::new())),
                mouse_input_handlers: Arc::new(Mutex::new(Vec::new())),
                mouse_wheel_handlers: Arc::new(Mutex::new(Vec::new())),
                keyboard_handlers: Arc::new(Mutex::new(Vec::new()))
            }
        };
    }

    pub fn clear(&self) {
        self.screen.draw_handler.lock().unwrap().clear();
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        self.screen.get_size()
    }

    pub fn show_frame_rate(&mut self, show: bool) {
        self.screen.show_fps.store(show, Ordering::SeqCst);
    }

    pub fn set_title(&mut self, new_title: &'static str) {
        self.screen.title = Arc::new(String::from(new_title));
    }

    pub fn set_refresh_rate(&mut self, new_rate: u64) {
        *self.screen.frame_delay.lock().unwrap() = Duration::from_millis(1000 / new_rate);
    }

    pub fn get_draw_color(&self) -> Color {
        self.screen.draw_handler.lock().unwrap().get_draw_color()
    }

    pub fn set_draw_color(&self, color: Color) {
        self.screen.draw_handler.lock().unwrap().set_draw_color(color);
    }

    pub fn draw_point(&self, point: (u32, u32)) {
        self.screen.draw_handler.lock().unwrap().draw_point(point);
    }

    pub fn draw_line(&self, point_a: (u32, u32), point_b: (u32, u32)) {
        self.screen.draw_handler.lock().unwrap().draw_line(point_a, point_b);
    }

    pub fn fill(&self, color: Color) {
        self.screen.draw_handler.lock().unwrap().fill(color);
    }

    pub fn add_mouse_move_handler(&mut self, handler: fn(e: &MouseMoveEvent) -> HandlerResult) -> Result<(), &'static str> {
        let mut mouse_move_handlers_lock = self.screen.mouse_move_handlers.lock().unwrap();
        mouse_move_handlers_lock.push(handler);
        Ok(())
    }

    pub fn add_mouse_input_handler(&mut self, handler: fn(e: &MouseInputEvent) -> HandlerResult) -> Result<(), &'static str> {
        let mut mouse_input_handlers_lock = self.screen.mouse_input_handlers.lock().unwrap();
        mouse_input_handlers_lock.push(handler);
        Ok(())
    }

    pub fn add_mouse_wheel_handler(&mut self, handler: fn(e: &MouseWheelEvent) -> HandlerResult) -> Result<(), &'static str> {
        let mut mouse_wheel_handlers_lock = self.screen.mouse_wheel_handlers.lock().unwrap();
        mouse_wheel_handlers_lock.push(handler);
        Ok(())
    }

    pub fn add_keyboard_handler(&mut self, handler: fn(e: &KeyboardEvent) -> HandlerResult) -> Result<(), &'static str> {
        let mut keyboard_handlers_lock = self.screen.keyboard_handlers.lock().unwrap();
        keyboard_handlers_lock.push(handler);
        Ok(())
    }

    pub fn open(&mut self) {
        self.screen.open.store(true, Ordering::SeqCst);
        self.screen.show();
    }

    pub fn is_open(&self) -> bool {
        self.screen.open.load(Ordering::SeqCst)
    }

    pub fn await_close(&mut self) {
        if self.screen.open.load(Ordering::SeqCst) {
            self.screen.open.store(true, Ordering::SeqCst);
            self.screen.handle.take().unwrap().join().unwrap();
        }
    }
}