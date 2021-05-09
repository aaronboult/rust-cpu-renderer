mod simulator;
use simulator::{Color, OriginPosition};
use simulator::objects::{Object, Cube, Spot};

extern crate rand;
use rand::Rng;

fn main() -> Result<(), String> {
    if cfg!(feature="r3d") {
        test_3d()
    }
    else if cfg!(feature="r2d") {
        test_2d()
    }
    else {
        Err(String::from("Neither 3D or 2D was provided as a compiler feature"))
    }
}


fn test_3d() -> Result<(), String> {
    let mut rng = rand::thread_rng();

    let window = simulator::WindowBuilder::new()
        .set_size(600, 800)
        .set_background_color(Color::GREY)
        .set_title("3D Test")
        .show_frame_rate();

    let mut sim = simulator::SimulationBuilder::new()
        .use_3d()
        .use_object_clearing()
        .build(window);
    
    sim.paint_background();

    let mut cube_ids: Vec<usize> = Vec::new();

    for _ in 0..50 {
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
            cube.transform_mut().rotation.x += scaler * 200.0 * delta;
            cube.transform_mut().rotation.y += scaler * 500.0 * delta;
            cube.transform_mut().rotation.z += scaler * 200.0 * delta;
        }
    }
    Ok(())
}

fn test_2d() -> Result<(), String> {
    let mut rng = rand::thread_rng();

    let window = simulator::WindowBuilder::new()
        .set_size(600, 800)
        .set_background_color(Color::GREY)
        .set_title("2D Test")
        .show_frame_rate();

    let mut sim = simulator::SimulationBuilder::new()
        .use_2d()
        .set_origin(OriginPosition::TOPLEFT)
        .build(window);

    sim.paint_background();

    const NUMBER_OF_SPOTS: u32 = 100_000;

    for _ in 0..NUMBER_OF_SPOTS {
        let obj = Spot::new(Color::RED)
            .set_position(rng.gen::<f32>() * 600.0, rng.gen::<f32>() * 800.0);
        obj.register(&mut sim);
    }

    while sim.update().is_ok() {
    }

    Ok(())
}