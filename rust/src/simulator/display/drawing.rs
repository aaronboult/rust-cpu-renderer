#![allow(dead_code)]

use sdl2::render::WindowCanvas;
use sdl2::rect::Point;

use std::marker::Send;
use std::ops::{Add, Sub};
use std::cmp::{max, min, Eq, PartialEq};

//#region Color
// provides an API for color operations, and can convert
// from and to sdl2 colors
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 0xff };
    pub const WHITE: Color = Color { r: 0xff, g: 0xff, b: 0xff, a: 0xff };
    pub const RED: Color = Color { r: 0xff, g: 0, b: 0, a: 0xff };
    pub const GREEN: Color = Color { r: 0, g: 0xff, b: 0, a: 0xff };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 0xff, a: 0xff };
    pub const ORANGE: Color = Color { r: 0xff, g: 0xa5, b: 0, a: 0xff };
    pub const PINK: Color = Color { r: 0xff, g: 0xc0, b: 0xcb, a: 0xff };
    pub const CYAN: Color = Color { r: 0, g: 0xff, b: 0xff, a: 0xff };
    pub const PURPLE: Color = Color { r: 0x80, g: 0, b: 0x80, a: 0xff };

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
        self.merge(other, |a,b| (a + b) / 2)
    }

    // subtractive blending: takes the average of the difference
    pub fn blend_sub(&self, other: Self) -> Self {
        self.merge(other, |a,b| (a - b) / 2)
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
        self.merge(other, |a,b| a + b)
    }
}

// implement basic subtractive coloring
impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.merge(other, |a,b| a - b)
    }
}
//#endregion

//#region Draw Traits
// provide a trait that allows the implementation of a new drawing style
pub trait Drawable<T: Send + Sync = Self> {
    fn new(bg_color: Color) -> Self;
    fn get_bg_color(&self) -> Color;
    fn set_bg_color(&mut self, color: Color);
    fn get_draw_color(&self) -> Color;
    fn set_draw_color(&mut self, color: Color);
    fn draw(&mut self, _: &mut WindowCanvas);
    fn clear(&mut self);
    fn draw_point(&mut self, point: (u32, u32));
    fn draw_line(&mut self, point_a: (u32, u32), point_b: (u32, u32));
    fn fill(&mut self, _: Color);
}
//#endregion

//#region Pixel
#[derive(Debug, Copy, Clone, Default)]
pub struct Pixel {
    color: Color,
    x: i32,
    y: i32
}

// allow easy conversion from a pixel into a point
impl Into<Point> for Pixel {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}
//#endregion

//#region PixelBuffer
// buffer Pixels in specific locations and draw them to the screen at runtime
// the buffer should only contain non-background pixels to avoid performance issues
pub struct PixelBufferDrawer {
    contents: Vec<Pixel>,
    bg_color: Color,
    draw_color: Color
}

impl Drawable for PixelBufferDrawer {
    fn new(bg_color: Color) -> Self {
        PixelBufferDrawer {
            contents: Vec::new(),
            bg_color,
            draw_color: Color::BLACK
        }
    }

    fn clear(&mut self) {
        self.contents = Vec::new();
    }

    fn get_bg_color(&self) -> Color {
        self.bg_color
    }

    fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    fn get_draw_color(&self) -> Color {
        self.draw_color
    }

    fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        for index in 0..self.contents.len() {
            canvas.set_draw_color(self.contents[index].color);
            canvas.draw_point(
                self.contents[index]
            ).unwrap();
        }
    }

    fn draw_point(&mut self, point: (u32, u32)) {
        self.contents.push(
            Pixel {
                x: point.0 as i32,
                y: point.1 as i32,
                color: self.draw_color
            }
        );
    }

    fn draw_line(&mut self, point_a: (u32, u32), point_b: (u32, u32)) {
        let (point_a, point_b) = if point_a.0 > point_b.0 {
            (point_b, point_a)
        }
        else {
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
            }
            else {
                m.floor() as i32
            };
            let c = point_a.1 as f32 - m * point_a.0 as f32;
            for x in point_a.0..point_b.0 {
                for y_diff in min(0, m_round)..max(0, m_round) {
                    let y = m * x as f32 + c;
                    self.draw_point((x, (y_diff + y as i32) as u32));
                }
            }
        }
        else if dy == 0.0 { // no change in y
            let min_x = min(point_a.0, point_b.0);
            let max_x = max(point_a.0, point_b.0);
            for x in min_x..max_x {
                self.draw_point((x, point_a.1));
            }
        }
        else { // no change in x
            let min_y = min(point_a.1, point_b.1);
            let max_y = max(point_a.1, point_b.1);
            for y in min_y..max_y {
                self.draw_point((point_a.0, y));
            }
        }
    }

    fn fill(&mut self, color: Color) {
        self.contents = Vec::new();
        self.bg_color = color;
    }
}
//#endregion

//#region Canvas
trait CanvasOperation: Send + Sync {
    fn execute(&self, canvas: &mut WindowCanvas);
    fn get_type(&self) -> CanvasOperationType;
}

struct OperationQueue {
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

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn clear(&mut self) {
        self.queue = Vec::new();
    }
}

#[derive(Debug, Copy, Clone)]
enum CanvasOperationType {
}

pub struct CanvasDrawer {
    operations: OperationQueue,
    bg_color: Color,
    draw_color: Color
}

impl Drawable for CanvasDrawer{
    fn new(bg_color: Color) -> Self {
        CanvasDrawer{
            operations: OperationQueue::new(),
            bg_color,
            draw_color: Color::BLACK
        }
    }

    fn clear(&mut self) {
        unimplemented!("Unimplemented clear");
    }

    fn get_bg_color(&self) -> Color {
        self.bg_color
    }

    fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    fn get_draw_color(&self) -> Color {
        self.draw_color
    }

    fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
    }

    fn draw(&mut self, canvas: &mut WindowCanvas){
        canvas.set_draw_color(self.draw_color);
        while self.operations.len() > 0 {
            let operation = self.operations.pop();
            println!("{:?}", operation.get_type());
            operation.execute(canvas);
        }
    }

    fn draw_point(&mut self, point: (u32, u32)) {
        println!("Point: {:?}", point);
        unimplemented!("draw_point(..) is not yet implemented for canvas drawer");
    }

    fn draw_line(&mut self, point_a: (u32, u32), point_b: (u32, u32)) {
        println!("Point:\n\tA: {:?}, B: {:?}", point_a, point_b);
        unimplemented!("draw_line(..) is not yet implemented for canvas drawer");
    }

    fn fill(&mut self, color: Color){
        self.set_bg_color(color);
    }
}
//#endregion

//#region Canvas Operations
//#endregion