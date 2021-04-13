use sdl2::render::WindowCanvas;
use sdl2::rect::Point;

use std::marker::Send;
use std::ops::{Add, Sub};
use std::cmp::{max, min};

//#region Color
#[derive(Debug, Copy, Clone, Default)]
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

    pub fn from_sdl_color(color: sdl2::pixels::Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a
        }
    }

    pub fn to_sdl_color(&self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::from((self.r, self.g, self.b, self.a))
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
    fn new(width: u32, height: u32) -> Self;
    fn draw(&self, _: &mut WindowCanvas){}
    fn fill(&mut self, _: Color);
}
//#endregion

//#region Drawable Structs
pub struct PixelBufferDrawer {
    contents: Vec<Vec<Color>>
}

impl Drawable for PixelBufferDrawer {
    fn new(width: u32, height: u32) -> Self {
        PixelBufferDrawer {
            contents: vec![vec![Color::default(); width as usize]; height as usize]
        }
    }

    fn draw(&self, canvas: &mut WindowCanvas) {
        for y in 0..self.contents.len() {
            for x in 0..self.contents[y].len() {
                canvas.set_draw_color(self.contents[y][x].to_sdl_color());
                canvas.draw_point(
                    Point::new(x as i32, y as i32)
                ).unwrap();
            }
        }
    }

    fn fill(&mut self, color: Color) {
        for y in 0..self.contents.len() {
            for x in 0..self.contents[y].len() {
                self.contents[y][x] = color;
            }
        }
    }
}

// Unit struct and empty impl of Drawable as no work needs to be done
pub struct CanvasDrawer;

impl Drawable for CanvasDrawer{
    fn new(_: u32, _: u32) -> Self {CanvasDrawer{}}
    fn fill(&mut self, _: Color){}
}
//#endregion