mod simulator;
use simulator::{Color, OriginPosition};
use simulator::objects::*;

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
    let mut _rng = rand::thread_rng();

    let window = simulator::WindowBuilder::new()
        .set_size(600, 800)
        .set_background_color(Color::GREY)
        .set_title("2D Test")
        .show_frame_rate();

    let mut sim = simulator::SimulationBuilder::new()
        .use_2d()
        .set_origin(OriginPosition::MIDDLEMIDDLE)
        .disable_3d_rotation()
        .build(window);

    sim.paint_background();

    // const NUMBER_OF_SPOTS: usize = 1;

    // for _ in 0..NUMBER_OF_SPOTS {
    //     let obj = Spot::new(Color::RED)
    //         .set_position(_rng.gen::<f32>() * 600.0, _rng.gen::<f32>() * 800.0)
    //         .register(&mut sim);
    // }

    const NUMBER_OF_CIRCLES: usize = 1000;

    for _ in 0..NUMBER_OF_CIRCLES {
        Circle::new(Color::BLACK, Color::BLACK)
            .set_radius(100.0)
            .register(&mut sim);
    }

    const NUMBER_OF_RECTANGLES: usize = 0;

    for _ in 0..NUMBER_OF_RECTANGLES {
        Rectangle::new(250.0 * _rng.gen::<f32>(), 250.0 * _rng.gen::<f32>()).register(&mut sim);
    }

    const NUMBER_OF_SQUARES: usize = 0;

    for _ in 0..NUMBER_OF_SQUARES {
        Square::new(250.0 * _rng.gen::<f32>()).register(&mut sim);
    }

    const ANGLE_MIN: f32 = -0.0001;
    const ANGLE_MAX: f32 = 0.00001;

    const MOVE_MIN: f32 = -50.0;
    const MOVE_MAX: f32 = 50.0;

    while sim.update().is_ok() {
        let delta = sim.time.get_delta_time();
        let mut index_offset = 0;
        for i in 0..NUMBER_OF_CIRCLES {
            let obj = sim.get_object_by_id(&(i + index_offset)).unwrap();
            obj.transform_mut().rotate_2d(_rng.gen_range(ANGLE_MIN, ANGLE_MAX) * delta);
            let m1 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            let m2 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            let m3 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            obj.transform_mut().translate(m1, m2, m3);
        }
        index_offset += NUMBER_OF_CIRCLES;
        for i in 0..NUMBER_OF_RECTANGLES {
            let obj = sim.get_object_by_id(&(i + index_offset)).unwrap();
            obj.transform_mut().rotate_2d(_rng.gen_range(ANGLE_MIN, ANGLE_MAX) * delta);
            let m1 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            let m2 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            let m3 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            obj.transform_mut().translate(m1, m2, m3);
        }
        index_offset += NUMBER_OF_RECTANGLES;
        for i in 0..NUMBER_OF_SQUARES {
            let obj = sim.get_object_by_id(&(i + index_offset)).unwrap();
            obj.transform_mut().rotate_2d(_rng.gen_range(ANGLE_MIN, ANGLE_MAX) * delta);
            let m1 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            let m2 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            let m3 = _rng.gen_range(MOVE_MIN, MOVE_MAX) * delta;
            obj.transform_mut().translate(m1, m2, m3);
        }
    }

    Ok(())
}