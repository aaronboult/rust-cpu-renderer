#![allow(dead_code)]

use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use std::cmp::{max, min, Eq, PartialEq};
use std::ops::{Add, Sub};

//#region Color
// provides an API for color operations, and can convert
// from and to sdl2 colors
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0xff,
    };
    pub const WHITE: Color = Color {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    pub const RED: Color = Color {
        r: 0xff,
        g: 0,
        b: 0,
        a: 0xff,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 0xff,
        b: 0,
        a: 0xff,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 0xff,
        a: 0xff,
    };
    pub const ORANGE: Color = Color {
        r: 0xff,
        g: 0xa5,
        b: 0,
        a: 0xff,
    };
    pub const PINK: Color = Color {
        r: 0xff,
        g: 0xc0,
        b: 0xcb,
        a: 0xff,
    };
    pub const CYAN: Color = Color {
        r: 0,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    pub const PURPLE: Color = Color {
        r: 0x80,
        g: 0,
        b: 0x80,
        a: 0xff,
    };

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 0xff)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    fn bound_color_value(value: u8) -> u8 {
        max(min(value, 0), 0xff)
    }

    fn merge(&self, other: Self, op: fn(u8, u8) -> u8) -> Self {
        Self {
            r: Color::bound_color_value(op(self.r, other.r)),
            g: Color::bound_color_value(op(self.g, other.g)),
            b: Color::bound_color_value(op(self.b, other.b)),
            a: Color::bound_color_value(op(self.a, other.a)),
        }
    }

    // additive blending: takes the average of the sum
    pub fn blend_add(&self, other: Self) -> Self {
        self.merge(other, |a, b| (a + b) / 2)
    }

    // subtractive blending: takes the average of the difference
    pub fn blend_sub(&self, other: Self) -> Self {
        self.merge(other, |a, b| (a - b) / 2)
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::from((self.r, self.g, self.b, self.a))
    }
}

impl From<sdl2::pixels::Color> for Color {
    fn from(item: sdl2::pixels::Color) -> Self {
        Self::rgba(item.r, item.g, item.b, item.a)
    }
}

// implement basic additive coloring
impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.merge(other, |a, b| a + b)
    }
}

// implement basic subtractive coloring
impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.merge(other, |a, b| a - b)
    }
}
//#endregion

//#region Pixel
#[derive(Debug, Copy, Clone, Default)]
pub struct Pixel {
    color: Color,
    x: usize,
    y: usize,
}
//#endregion

//#region Drawer
#[derive(Copy, Clone, PartialEq)]
pub enum DrawMode {
    PIXELBUFFER,
    CANVAS,
}

pub struct Drawer {
    bg_color: Color,
    draw_color: Color,
    width: usize,
    height: usize,
    mode: DrawMode,
    pixel_contents: Vec<Pixel>,
    canvas_operations: OperationQueue,
}

impl Drawer {
    pub fn new(
        bg_color: Color,
        draw_color: Color,
        mode: DrawMode,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            bg_color,
            draw_color,
            mode,
            width: width as usize,
            height: height as usize,
            pixel_contents: Vec::new(), // * 4 for each value in rgba
            canvas_operations: OperationQueue::new(),
        }
    }

    pub fn get_canvas_operations(&self) -> Option<&OperationQueue> {
        if self.mode == DrawMode::CANVAS {
            return Some(&self.canvas_operations);
        }
        None
    }

    pub fn get_last_canvas_operation(&self) -> Option<&Box<dyn CanvasOperation>> {
        if self.mode == DrawMode::CANVAS {
            return Some(self.canvas_operations.peek(0));
        }
        None
    }

    pub fn get_bg_color(&self) -> Color {
        self.bg_color
    }

    pub fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    pub fn get_draw_color(&self) -> Color {
        self.draw_color
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width as usize;
        self.height = height as usize;
    }

    pub fn draw_point(&mut self, point: (i32, i32)) {
        match self.mode {
            DrawMode::PIXELBUFFER => {
                self.pixel_contents.push(
                    Pixel {
                        color: self.draw_color,
                        x: point.0 as usize,
                        y: point.1 as usize
                    }
                );
            }
            DrawMode::CANVAS => {
                unimplemented!("Draw point not implemented for canvas");
            }
        }
    }

    pub fn draw_line(&mut self, point_a: (i32, i32), point_b: (i32, i32)) {
        match self.mode {
            DrawMode::PIXELBUFFER => {
                let (point_a, point_b) = if point_a.0 > point_b.0 {
                    (point_b, point_a)
                } else {
                    (point_a, point_b)
                };
                // using y=mx+c
                // as well as the ratio dy:dx
                let dy = point_b.1 as f32 - point_a.1 as f32;
                let dx = point_b.0 as f32 - point_a.0 as f32;
                if dy != 0.0 && dx != 0.0 {
                    let m = dy / dx;
                    let m_round = if m > 0.0 {
                        m.ceil() as i32
                    } else {
                        m.floor() as i32
                    };
                    let c = point_a.1 as f32 - m * point_a.0 as f32;
                    for x in point_a.0..point_b.0 {
                        for y_diff in min(0, m_round)..max(0, m_round) {
                            let y = m * x as f32 + c;
                            self.draw_point((x, y_diff + y as i32));
                        }
                    }
                } else if dy == 0.0 {
                    // no change in y
                    let min_x = min(point_a.0, point_b.0);
                    let max_x = max(point_a.0, point_b.0);
                    for x in min_x..max_x {
                        self.draw_point((x, point_a.1));
                    }
                } else {
                    // no change in x
                    let min_y = min(point_a.1, point_b.1);
                    let max_y = max(point_a.1, point_b.1);
                    for y in min_y..max_y {
                        self.draw_point((point_a.0, y));
                    }
                }
            }
            DrawMode::CANVAS => {
                unimplemented!("Draw line not implemented for canvas");
            }
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.clear();
        self.set_bg_color(color);
    }

    pub fn clear(&mut self) {
        self.pixel_contents.clear();
        if self.mode == DrawMode::PIXELBUFFER {
            self.pixel_contents = Vec::new();
        }
        self.canvas_operations.clear();
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas, event_pump: &EventPump) {
        match self.mode {
            DrawMode::PIXELBUFFER => {
                let mut surface = canvas.window().surface(event_pump).unwrap();
                let format = surface.pixel_format_enum();
                let surface_pixels = surface.without_lock_mut().unwrap();
                println!("Draw:");
                let mut start = std::time::Instant::now();
                for y in 0..self.height {
                    for x in 0..self.width {
                        let index = 4 * (y * self.width + x); // color stored in format bgr
                        surface_pixels[index] = self.bg_color.b;
                        surface_pixels[index + 1] = self.bg_color.g;
                        surface_pixels[index + 2] = self.bg_color.r;
                    }
                }
                println!("\tClearing took: {:?}", start.elapsed());
                // println!("Info:\n\tPixels: {}\n\tWidth: {}\n\tHeight: {}\n\tLen: {}\n\tFormat: {:?}", self.width * self.height, self.width, self.height, self.pixel_contents.len(), format);
                start = std::time::Instant::now();
                for pixel in self.pixel_contents.iter() {
                    if pixel.y < self.height && pixel.x < self.width {
                        let index = 4 * (pixel.y * self.width + pixel.x); // color stored in format bgr
                        surface_pixels[index] = pixel.color.b;
                        surface_pixels[index + 1] = pixel.color.g;
                        surface_pixels[index + 2] = pixel.color.r;
                    }
                }
                println!("\tDrawing took: {:?}", start.elapsed());
                self.pixel_contents = Vec::new();
                surface.update_window().unwrap();
            }
            DrawMode::CANVAS => {
                canvas.set_draw_color(self.draw_color);
                while self.canvas_operations.len() > 0 {
                    let operation = self.canvas_operations.pop();
                    println!("{:?}", operation.get_type());
                    operation.execute(canvas);
                }
                canvas.present();
            }
        }
    }
}

//#endregion

//#region Canvas Operations
#[derive(Debug, Copy, Clone)]
pub enum CanvasOperationType {}

pub trait CanvasOperation {
    fn execute(&self, canvas: &mut WindowCanvas);
    fn get_type(&self) -> CanvasOperationType;
}

pub struct OperationQueue {
    queue: Vec<Box<dyn CanvasOperation>>,
}

impl OperationQueue {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn add(&mut self, item: Box<dyn CanvasOperation>) {
        self.queue.push(item);
    }

    fn pop(&mut self) -> Box<dyn CanvasOperation> {
        self.queue.remove(0)
    }

    fn peek(&self, index: usize) -> &Box<dyn CanvasOperation> {
        &self.queue[index]
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn clear(&mut self) {
        self.queue = Vec::new();
    }
}
//#endregion
