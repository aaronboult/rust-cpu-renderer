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
    let window = simulator::WindowBuilder::new()
        .set_size(600, 800)
        .set_background_color(Color::GREY)
        .set_title("3D Test")
        .show_frame_rate();

    let mut sim = simulator::SimulationBuilder::new()
        .use_3d()
        .use_object_clearing()
        .set_mainloop(mainloop_3d)
        .build(window);
    
    sim.paint_background();

    for _ in 0..50 {
        Cube::new()
            .set_position(0.0, 0.0, -20.0)
            .register(&mut sim);
    }
    
    sim.start()
}

fn mainloop_3d(sim: &mut Simulator) -> Result<(), &'static str> {
    let delta = sim.time.get_delta_time();
    for i in 0..sim.object_count() {
        sim.clear_from_screen(&i);
        let obj = sim.get_object_by_id(&i).unwrap();
        obj.transform_mut().rotation.x += 200.0 * delta;
        obj.transform_mut().rotation.y += 500.0 * delta;
        obj.transform_mut().rotation.z += 200.0 * delta;
    }
    Ok(())
}

fn test_2d() -> Result<(), &'static str> {
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
        .set_mainloop(mainloop_2d)
        .build(window);

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

    sim.start()
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