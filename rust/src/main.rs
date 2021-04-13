mod display;
use display::{Display, PixelBufferDrawer, CanvasDrawer, Color};

use std::{thread, time};

fn main(){

    let mut d = Display::<PixelBufferDrawer>::new(512, 512);

    d.set_refresh_rate(100);

    d.open();

    thread::sleep(time::Duration::from_secs(20));

    d.fill(
        Color::from_sdl_color(sdl2::pixels::Color::BLUE)
    );

    d.set_refresh_rate(10);

    d.await_close();

}