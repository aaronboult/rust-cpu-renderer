mod simulator;
use simulator::{Object, Color};

extern crate rand;
use rand::Rng;

fn main() -> Result<(), ()> {

    let mut rng = rand::thread_rng();

    let window = simulator::WindowBuilder::new()
        .set_size(600, 800)
        .set_background_color(Color::GREY)
        .set_min_size(1000, 1000)
        .set_max_size(900, 900)
        .start_maximized()
        .disable_resize()
        .show_frame_rate();

    let mut sim = simulator::SimulationBuilder::new()
        .use_3d()
        .build(window);
    
    // sim.restrict_frame_rate().set_target_frame_rate(60);

    let mut cube_ids: Vec<usize> = Vec::new();

    for _ in 0..1 {
        cube_ids.push(
            Object::new_cube()
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
            let cube = sim.get_object_by_id(*id);
            cube.transform.rotation.x += scaler * 500.0 * delta;
            cube.transform.rotation.y += scaler * 500.0 * delta;
            cube.transform.rotation.z += scaler * 500.0 * delta;
        }
    }

    Ok(())

}