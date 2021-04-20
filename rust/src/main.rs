mod simulator;
use simulator::Object;

extern crate rand;
use rand::prelude::*;

fn main() -> Result<(), ()> {

    let mut rng = rand::thread_rng();

    const SPAWNRANGE: std::ops::Range<f32> = -1.0..1.0;

    const ROTATERANGE: std::ops::Range<f32> = -60.0..60.0;

    let mut sim = simulator::SimulationBuilder::new()
        .use_3d()
        .set_size(600, 800)
        .lock_frame_rate(60)
        .build();

    let mut cube_ids: Vec<usize> = Vec::new();

    for _ in 0..1 {
        cube_ids.push(
            Object::new_cube()
                .set_position(
                    rng.gen_range(SPAWNRANGE),
                    rng.gen_range(SPAWNRANGE),
                    rng.gen_range(SPAWNRANGE) - 20.0
                )
                .set_scale(
                    rng.gen_range(SPAWNRANGE) * 4.0,
                    rng.gen_range(SPAWNRANGE) * 4.0,
                    rng.gen_range(SPAWNRANGE) * 4.0
                )
                .register(&mut sim)
        );
    }

    sim.start();

    while sim.update().is_ok() {
        let delta = sim.time.get_delta_time();
        println!("{}", delta);
        for id in cube_ids.iter() {
            let cube = sim.get_object_by_id(*id);
            cube.transform.rotation.x += rng.gen_range(ROTATERANGE) * delta;
            cube.transform.rotation.y += rng.gen_range(ROTATERANGE) * delta;
            cube.transform.rotation.z += rng.gen_range(ROTATERANGE) * delta;
            // cube.transform.position.x += rng.gen_range(RANGE) / 20.0;
            // cube.transform.position.y += rng.gen_range(RANGE) / 20.0;
            // cube.transform.position.z += rng.gen_range(RANGE) / 20.0;
        }
    }

    Ok(())

}