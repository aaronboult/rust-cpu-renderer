mod simulator;
use simulator::Color;
use simulator::objects::{Object, Cube};

extern crate rand;
use rand::Rng;

fn main() -> Result<(), ()> {

    let mut rng = rand::thread_rng();

    let window = simulator::WindowBuilder::new()
        .set_size(600, 800)
        .set_background_color(Color::GREY)
        .set_title("Test title")
        .show_frame_rate();

    let mut sim = simulator::SimulationBuilder::new()
        .use_3d()
        .use_object_clearing()
        .build(window);
    
    sim.paint_background();

    let mut cube_ids: Vec<usize> = Vec::new();

    for _ in 0..1000 {
        cube_ids.push(
            Cube::new()
                .set_position(
                    0.0,
                    0.0,
                    -20.0
                )
                .register(&mut sim)
        );
    }
    while sim.update().is_ok() {
        let delta = sim.time.get_delta_time();
        for id in cube_ids.iter() {
            let scaler: f32 = rng.gen();
            sim.clear_from_screen(id);
            let cube = sim.get_object_by_id(id).unwrap();
            // cube.transform_mut().rotation.x += scaler * 200.0 * delta;
            cube.transform_mut().rotation.y += scaler * 500.0 * delta;
            // cube.transform_mut().rotation.z += scaler * 200.0 * delta;
        }
    }
    Ok(())
}