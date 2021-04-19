mod simulator;
use simulator::Object;

extern crate rand;
use rand::prelude::*;

fn main() -> Result<(), ()> {

    let mut rng = rand::thread_rng();

    const RANGE: std::ops::Range<f32> = -1.0..1.0;

    let mut sim = simulator::Simulator::new();

    let mut cube_ids: Vec<usize> = Vec::new();

    for _ in 0..100 {
        cube_ids.push(
            Object::new_cube()
                .set_position(
                    rng.gen_range(RANGE),
                    rng.gen_range(RANGE),
                    rng.gen_range(RANGE) - 20.0
                )
                .set_scale(
                    rng.gen_range(RANGE) * 4.0,
                    rng.gen_range(RANGE) * 4.0,
                    rng.gen_range(RANGE) * 4.0
                )
                .register(&mut sim)
        );
    }

    sim.start();

    while sim.update().is_ok() {
        let delta = sim.time.get_delta_time() as f32;
        for id in cube_ids.iter() {
            let cube = sim.get_object_by_id(*id);
            cube.transform.rotation.x += rng.gen_range(RANGE) * 0.1 * delta;
            cube.transform.rotation.y += rng.gen_range(RANGE) * 0.1 * delta;
            cube.transform.rotation.z += rng.gen_range(RANGE) * 0.1 * delta;
            // cube.transform.position.x += rng.gen_range(RANGE) / 20.0;
            // cube.transform.position.y += rng.gen_range(RANGE) / 20.0;
            // cube.transform.position.z += rng.gen_range(RANGE) / 20.0;
        }
    }

    Ok(())

}