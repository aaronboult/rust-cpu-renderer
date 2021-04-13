use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect, Point};

use std::marker::Send;
use std::ops::{Add, Sub};
use std::cmp::{max, min, Eq, PartialEq};

//#region Color
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
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

    pub fn blend_add(&self, other: Self) -> Self {
        self.merge(other, |a,b| (a + b) / 2)
    }

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

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.merge(other, |a,b| a + b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.merge(other, |a,b| a - b)
    }
}
//#endregion

//#region Draw Traits
pub trait Drawable<T: Send + Sync = Self> {
    fn new(width: u32, height: u32, bg_color: Color) -> Self;
    fn get_bg_color(&self) -> Color;
    fn set_bg_color(&mut self, color: Color);
    fn draw(&mut self, _: &mut WindowCanvas);
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

impl Into<Point> for Pixel {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}
//#endregion

//#region PixelBuffer
pub struct PixelBufferDrawer {
    contents: Vec<Pixel>,
    bg_color: Color
}

impl Drawable for PixelBufferDrawer {
    fn new(width: u32, height: u32, bg_color: Color) -> Self {
        PixelBufferDrawer {
            contents: Vec::new(),
            bg_color
        }
    }

    fn get_bg_color(&self) -> Color {
        self.bg_color
    }

    fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        for index in 0..self.contents.len() {
            canvas.set_draw_color(self.contents[index].color);
            canvas.draw_point(
                self.contents[index]
            ).unwrap();
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
    FILL
}

pub struct CanvasDrawer {
    operations: OperationQueue,
    bg_color: Color
}

impl Drawable for CanvasDrawer{
    fn new(_: u32, _: u32, bg_color: Color) -> Self {
        CanvasDrawer{
            operations: OperationQueue::new(),
            bg_color
        }
    }

    fn get_bg_color(&self) -> Color {
        self.bg_color
    }

    fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    fn draw(&mut self, canvas: &mut WindowCanvas){
        while self.operations.len() > 0 {
            let operation = self.operations.pop();
            println!("{:?}", operation.get_type());
            operation.execute(canvas);
        }
    }

    fn fill(&mut self, color: Color){
        self.bg_color = color;
        self.operations.clear();
    }
}

//#region Canvas Operations
struct FillOperation {
    optype: CanvasOperationType,
    color: Color
}
impl FillOperation {
    fn new(color: Color) -> Self {
        Self {
            optype: CanvasOperationType::FILL,
            color
        }
    }
}
impl CanvasOperation for FillOperation {
    fn get_type(&self) -> CanvasOperationType {
        self.optype
    }
    fn execute(&self, canvas: &mut WindowCanvas) {
        let size = canvas.output_size().unwrap();
        canvas.set_draw_color(self.color);
        canvas.fill_rect(
            Rect::new(0, 0, size.0, size.1)
        ).unwrap();
    }
}
//#endregion

//#endregion