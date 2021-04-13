extern crate sdl2;

mod drawing;
pub use drawing::{Drawable, PixelBufferDrawer, CanvasDrawer, Color};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::{thread, time};
use time::{Instant, Duration};
use std::marker::Send;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

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
                draw_handler: Arc::new(Mutex::new(T::new(width, height, Color::rgb(255, 255, 255))))
            }
        };
    }

    pub fn set_title(&mut self, new_title: String) {
        self.screen.title = Arc::new(new_title);
    }

    pub fn set_refresh_rate(&mut self, new_rate: u64) {
        *self.screen.frame_delay.lock().unwrap() = Duration::from_millis(1000 / new_rate);
    }

    pub fn fill(&self, color: Color) {
        self.screen.draw_handler.lock().unwrap().fill(color);
    }

    pub fn open(&mut self) {
        self.screen.open.store(true, Ordering::SeqCst);
        self.screen.show();
    }

    pub fn await_close(&mut self) {
        self.screen.open.store(true, Ordering::SeqCst);
        self.screen.handle.take().unwrap().join().unwrap();
    }
}

struct Screen<T> where T: Drawable + Send + Sync + 'static {
    open: Arc<AtomicBool>,
    width: Arc<AtomicU32>,
    height: Arc<AtomicU32>,
    handle: Option<thread::JoinHandle<()>>,
    title: Arc<String>,
    frame_delay: Arc<Mutex<Duration>>,
    draw_handler: Arc<Mutex<T>>
}

impl<T> Screen<T> where T: Drawable + Send + Sync + 'static {
    fn show(&mut self) {
        let open = self.open.clone();
        let width = self.width.clone();
        let height = self.height.clone();
        let title = self.title.clone();
        let frame_delay = self.frame_delay.clone();

        let draw_handler = self.draw_handler.clone();

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

                    let mut frame_count: u64 = 0;
                    let mut fps_timer = Instant::now();
                    const SECOND_DURATION: Duration = Duration::from_secs(1);

                    'main: loop {
                        if !open.load(Ordering::SeqCst) {
                            break 'main;
                        }

                        if frame_start_time.elapsed() >= {*frame_delay.lock().unwrap()} {
                            let mut handler_guard = draw_handler.lock().unwrap();

                            canvas.set_draw_color(handler_guard.get_bg_color());
                            canvas.clear();

                            frame_start_time = Instant::now();

                            frame_count += 1;

                            handler_guard.draw(&mut canvas);

                            canvas.present();
                        }

                        if fps_timer.elapsed() >= SECOND_DURATION {
                            println!("Frames elapsed: {}", frame_count);
                            frame_count = 0;
                            fps_timer = Instant::now();
                        }

                        for event in event_pump.poll_iter() {
                            match event {
                                Event::Quit {..} |
                                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                                    open.store(false, Ordering::SeqCst);
                                },

                                _ => {}
                            }
                        }
                    }
                }
            )
        );
    }
}