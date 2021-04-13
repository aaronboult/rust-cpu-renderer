mod display;
use display::{Display, PixelBufferDrawer, CanvasDrawer, Color};

use std::{thread, time};

fn main(){

    let mut d = Display::<PixelBufferDrawer>::new(512, 512);

    d.set_refresh_rate(100);

    d.open();

    thread::sleep(time::Duration::from_secs(5));

    d.fill(
        Color::from(sdl2::pixels::Color::BLUE)
    );

    d.await_close();

}