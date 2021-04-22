mod simulator;
use simulator::Object;

extern crate rand;
use rand::prelude::*;

fn main() -> Result<(), ()> {

    let mut rng = rand::thread_rng();

    let mut sim = simulator::SimulationBuilder::new()
        .use_3d()
        .set_size(600, 800)
        .use_pixel_buffer()
        .build();
    
    // sim.restrict_frame_rate().set_target_frame_rate(60);

    sim.show_fps(true);

    let mut cube_ids: Vec<usize> = Vec::new();

    sim.start();

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
            let cube = sim.get_object_by_id(*id);
            cube.transform.rotation.x += rng.gen_range(0.0..60.0) * delta;
            cube.transform.rotation.y += rng.gen_range(0.0..60.0) * delta;
            cube.transform.rotation.z += rng.gen_range(0.0..60.0) * delta;
        }
    }

    Ok(())

}