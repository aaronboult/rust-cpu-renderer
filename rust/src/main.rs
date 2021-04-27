mod window;
use window::color::Color;

fn main() {
    let mut wind = window::WindowBuilder::new()
        .set_width(600)
        .set_height(600)
        .set_title("Simulation Engine")
        .build();
    let mut col = 0;
    const COLRATE: i32 = 1;
    while wind.is_running() {
        wind.fill(
            if col < COLRATE {
                Color::RED
            }
            else if col < 2 * COLRATE {
                Color::BLUE
            }
            else if col < 3 * COLRATE {
                Color::GREEN
            }
            else {
                if col == 4 * COLRATE{
                    col = -1;
                }
                Color::ORANGE
            }
        );
        // wind.fill(Color::GREY);
        col += 1;
        wind.update();
    }
}