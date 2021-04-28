#![allow(dead_code)]

// mod screen;
// use screen::Screen;

mod window;
pub use window::WindowBuilder;
pub use window::color::Color;
use window::Window;

use std::collections::HashMap;

mod renderer;
use renderer::{Transform, Renderer, RenderMode};
pub use renderer::linearalgebra::{Vector2D, Vector3D};

pub mod time;
use time::{Time, Instant, Duration};

// used for ID generation
use std::sync::atomic::{AtomicUsize, Ordering};

static OBJECTID: AtomicUsize = AtomicUsize::new(0);

//#region Object and Vertex
pub struct Object {
    pub transform: Transform,
    verticies: Vec<Vertex>,
    frame_color: Color,
    id: usize
}

impl Object {
    fn new_id() -> usize {
        let id = OBJECTID.fetch_add(1, Ordering::SeqCst);
        if id == usize::MAX {
            OBJECTID.store(0, Ordering::SeqCst);
        }
        id
    }

    pub fn new() -> Self {
        Self {
            id: Object::new_id(),
            transform: Transform::new(),
            verticies: Vec::new(),
            frame_color: Color::BLACK,
        }
    }
    pub fn new_cube() -> Self {
        Self {
            id: Object::new_id(),
            transform: Transform::new(),
            verticies: vec![
                Vertex { // ltb
                    rel_pos: Vector3D { x: -1.0, y: -1.0, z: -1.0 },
                    connects: vec![1, 2, 4]
                },
                Vertex { // rtb
                    rel_pos: Vector3D { x: 1.0, y: -1.0, z: -1.0 },
                    connects: vec![0, 3, 5]
                },
                Vertex { // ltf
                    rel_pos: Vector3D { x: -1.0, y: -1.0, z: 1.0 },
                    connects: vec![0, 3, 6]
                },
                Vertex { // rtf
                    rel_pos: Vector3D { x: 1.0, y: -1.0, z: 1.0 },
                    connects: vec![1, 2, 7]
                },
                Vertex { // lbb
                    rel_pos: Vector3D { x: -1.0, y: 1.0, z: -1.0 },
                    connects: vec![5, 6]
                },
                Vertex { // rbb
                    rel_pos: Vector3D { x: 1.0, y: 1.0, z: -1.0 },
                    connects: vec![4, 7]
                },
                Vertex { // lbf
                    rel_pos: Vector3D { x: -1.0, y: 1.0, z: 1.0 },
                    connects: vec![4, 7]
                },
                Vertex { // rbf
                    rel_pos: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
                    connects: vec![5, 6]
                },
            ],
            frame_color: Color::BLACK,
        }
    }

    pub fn register(self, registrar: &mut Simulator) -> usize {
        let id = self.id;
        registrar.add_object(self);
        id
    }

    pub fn set_position(mut self, x: f32, y: f32, z: f32) -> Self{
        self.transform.set_position(x, y, z);
        self
    }

    pub fn set_rotation(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform.set_rotation(x, y, z);
        self
    }

    pub fn set_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform.set_scale(x, y, z);
        self
    }
}

pub struct Vertex {
    rel_pos: Vector3D,
    connects: Vec<usize>
}

impl Vertex {
    pub fn new(rel_pos: Vector3D) -> Self {
        Self {
            rel_pos,
            connects: Vec::new()
        }
    }
    pub fn add_connection(&mut self, index: usize) -> &mut Self {
        self.connects.push(index);
        self
    }
    pub fn add_connections(&mut self, indexs: Vec<usize>) -> &mut Self {
        for index in indexs {
            self.add_connection(index);
        }
        self
    }
}
//#endregion

//#region Simulator
pub struct Simulator {
    objects: HashMap<usize, Object>,
    renderer: Renderer,
    window: Window,
    pub time: Time,
    restrict_frame_rate: bool,
    frame_delay: Duration,
    last_frame_start: Instant
}

impl Simulator {
    pub fn update(&mut self) -> Result<f32, ()> {
        if !self.window.is_running() {
            return Err(());
        }

        if self.restrict_frame_rate { // enter loop to maintain framerate if needed
            std::thread::sleep(self.frame_delay - self.last_frame_start.elapsed());
        }

        self.last_frame_start = Instant::now();

        let delta = self.time.update();
        
        self.window.fill(self.window.get_background_color());

        for obj in self.objects.values_mut() {
            let mut projected_vertexs: Vec<(i32, i32)> = Vec::new();

            for i in 0..obj.verticies.len() {
                projected_vertexs.push((-1, -1));
                let projected_coords = self.renderer.project_to_screen(&obj.transform, &obj.verticies[i].rel_pos, self.window.get_window_size());
                projected_vertexs[i] = projected_coords;
            }

            for i in 0..projected_vertexs.len() {
                self.window.draw_point(
                    projected_vertexs[i].0, projected_vertexs[i].1, obj.frame_color
                );
                for o in obj.verticies[i].connects.iter() {
                    self.window.draw_line(
                        (projected_vertexs[i].0, projected_vertexs[i].1), 
                        (
                            projected_vertexs[*o].0,
                            projected_vertexs[*o].1
                        ),
                        obj.frame_color
                    );
                }
            }

        }
        
        self.window.update();

        Ok(delta)
    }

    pub fn set_frame_rate_restriction(&mut self, restrict: bool) -> &mut Self {
        self.restrict_frame_rate = restrict;
        self
    }

    pub fn restrict_frame_rate(&mut self) -> &mut Self {
        self.set_frame_rate_restriction(true)
    }

    pub fn release_frame_rate(&mut self) -> &mut Self {
        self.set_frame_rate_restriction(false)
    }

    pub fn set_target_frame_rate(&mut self, target: u16) -> &mut Self {
        self.frame_delay = Duration::from_nanos(1_000_000_000 / target as u64);
        self
    }

    pub fn set_frame_rate_display(&mut self, show: bool) -> &mut Self {
        self.window.set_frame_rate_display(show);
        self
    }

    pub fn show_fps(&mut self) -> &mut Self{
        self.set_frame_rate_display(true)
    }

    pub fn hide_fps(&mut self) -> &mut Self {
        self.set_frame_rate_display(false)
    }

    pub fn add_object(&mut self, object: Object) -> usize {
        let id = object.id;
        self.objects.insert(id, object);
        id
    }

    pub fn get_object_by_id(&mut self, id: usize) -> &mut Object {
        self.objects.get_mut(&id).unwrap()
    }
}
//#endregion

//#region SimulationBuilder
pub struct SimulationBuilder {
    restrict_frame_rate: bool,
    target_frame_rate: u16,
    render_mode: RenderMode,
    width: u32,
    height: u32,
}

impl SimulationBuilder {
    pub fn new() -> Self {
        Self {
            restrict_frame_rate: false,
            target_frame_rate: 60,
            render_mode: RenderMode::R2D,
            width: 512,
            height: 512,
        }
    }

    pub fn restrict_frame_rate(&mut self) -> &mut Self {
        self.restrict_frame_rate = true;
        self
    }

    pub fn release_frame_rate(&mut self) -> &mut Self {
        self.restrict_frame_rate = false;
        self
    }

    pub fn set_target_frame_rate(&mut self, target: u16) -> &mut Self {
        self.target_frame_rate = target;
        self
    }

    pub fn lock_frame_rate(&mut self, target: u16) -> &mut Self {
        self.restrict_frame_rate = true;
        self.target_frame_rate = target;
        self
    }

    pub fn set_render_mode(&mut self, mode: RenderMode) -> &mut Self {
        self.render_mode = mode;
        self
    }

    pub fn use_3d(&mut self) -> &mut Self {
        self.render_mode = RenderMode::R3D;
        self
    }

    pub fn use_2d(&mut self) -> &mut Self {
        self.render_mode = RenderMode::R2D;
        self
    }

    // consume the windowbuilder used for constructing the window
    pub fn build(&self, window_builder: WindowBuilder) -> Simulator {
        Simulator {
            objects: HashMap::new(),
            renderer: Renderer::new(self.render_mode),
            time: Time::new(),
            window: window_builder.build(),
            restrict_frame_rate: self.restrict_frame_rate,
            frame_delay: Duration::from_nanos(1_000_000_000 / self.target_frame_rate as u64),
            last_frame_start: Instant::now()
        }
    }
}
//#endregion