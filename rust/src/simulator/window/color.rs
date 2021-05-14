use std::cmp::{Eq, PartialEq};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ColorMode {
    RGB,
    BGR
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    mode: ColorMode
}

macro_rules! const_color {
    ($name:ident, $r:literal, $g:literal, $b:literal) => (pub const $name: Color = Color { r: $r, g: $g, b: $b, a: 0xff, mode: ColorMode::RGB };);
    ($name:ident, $r:literal, $g:literal, $b:literal, $a:literal) => (pub const $name: Color = Color { r: $r, g: $g, b: $b, a: $a, mode: ColorMode::RGB };);
}

impl Color {
    const_color!(BLACK, 0, 0, 0);
    const_color!(WHITE, 0xff, 0xff, 0xff);
    const_color!(RED, 0xff, 0, 0);
    const_color!(GREEN, 0, 0xff, 0);
    const_color!(BLUE, 0, 0, 0xff);
    const_color!(ORANGE, 0xff, 0xa5, 0x00);
    const_color!(GREY, 0x80, 0x80, 0x80);
    const_color!(GRAY, 0x80, 0x80, 0x80);
    const_color!(TRANSPARENT, 0, 0, 0, 0);

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::rgba(r, g, b, 0xff)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a, mode: ColorMode::RGB }
    }

    pub fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Color::rgba(r, g, b, a)
    }

    pub fn bgr(b: u8, g: u8, r: u8) -> Self {
        Color::rgb(r, g, b)
    }

    pub fn abgr(a: u8, b: u8, g: u8, r: u8) -> Self {
        Color::rgba(r, g, b, a)
    }

    pub fn as_rgb(&mut self) -> &mut Self {
        self.mode = ColorMode::RGB;
        self
    }

    pub fn as_bgr(&mut self) -> &mut Self {
        self.mode = ColorMode::BGR;
        self
    }

    pub fn to_rgb(self) -> Self {
        Self { r: self.r, g: self.g, b: self.b, a: self.a, mode: ColorMode::RGB }
    }

    pub fn to_bgr(self) -> Self {
        Self { r: self.r, g: self.g, b: self.b, a: self.a, mode: ColorMode::BGR }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        #[allow(unreachable_patterns)]
        match color.mode {
            ColorMode::RGB => ((color.r as u32) << 16) | ((color.g as u32) << 8) | color.b as u32,
            ColorMode::BGR => ((color.b as u32) << 16) | ((color.g as u32) << 8) | color.r as u32,
            _ => unimplemented!("Unimplemented color mode")
        }
    }
}