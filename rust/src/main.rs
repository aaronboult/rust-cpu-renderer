mod simulator;
use simulator::Simulator;
use simulator::{Color, OriginPosition};
use simulator::event::EventFilter;
use simulator::objects::*;

extern crate rand;
use rand::Rng;

fn main() -> Result<(), &'static str> {
    if cfg!(feature="r3d") {
        test_3d()
    }
    else if cfg!(feature="r2d") {
        test_2d()
    }
    else {
        Err("Neither 3D or 2D was provided as a compiler feature")
    }
}


fn test_3d() -> Result<(), &'static str> {
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

fn test_2d() -> Result<(), &'static str> {
    let mut _rng = rand::thread_rng();

    let window = simulator::WindowBuilder::new()
        .set_size(600, 800)
        .set_background_color(Color::GREY)
        .set_title("2D Test");
        // .show_frame_rate();

    let mut sim = simulator::SimulationBuilder::new()
        .use_2d()
        .set_origin(OriginPosition::MIDDLEMIDDLE)
        .disable_3d_rotation()
        .build(window);
    
    sim.set_move_callback(mainloop_2d);

    sim.paint_background();

    const NUMBER_OF_SPOTS: usize = 0;

    for _ in 0..NUMBER_OF_SPOTS {
        let _obj = Spot::new(Color::RED)
            .set_position(_rng.gen::<f32>() * 600.0, _rng.gen::<f32>() * 800.0)
            .register(&mut sim);
    }

    const NUMBER_OF_CIRCLES: usize = 0;

    for _ in 0..NUMBER_OF_CIRCLES {
        Circle::new(Color::BLACK, Color::BLACK)
            .set_radius(100.0)
            .register(&mut sim);
    }

    const NUMBER_OF_RECTANGLES: usize = 0;

    for _ in 0..NUMBER_OF_RECTANGLES {
        Rectangle::new(250.0 * _rng.gen::<f32>(), 250.0 * _rng.gen::<f32>()).register(&mut sim);
    }

    const NUMBER_OF_SQUARES: usize = 1;

    for _ in 0..NUMBER_OF_SQUARES {
        Square::new(250.0 * _rng.gen::<f32>()).register(&mut sim);
    }

    while sim.update().is_ok() {
        mainloop_2d(&mut sim).unwrap();
    }

    Ok(())
}

fn mainloop_2d(sim: &mut Simulator) -> Result<(), &'static str> {
    let delta = sim.time.get_delta_time();
    for i in 0..sim.object_count() {
        sim.get_object_by_id(&i).unwrap().transform_mut().rotate_2d(5.0 * delta);
    }
    for event in sim.poll_events().filter(EventFilter::WINDOWEVENT) {
        println!("{:?}", event);
    }
    Ok(())
}