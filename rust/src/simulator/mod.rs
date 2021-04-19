#![allow(dead_code)]

mod display;
use display::Screen;

use std::collections::HashMap;

mod renderer;
use renderer::{Transform, Renderer};
pub use renderer::geometry::{Vector2D, Vector3D};

pub mod time;
use time::Time;

// used for ID generation
use std::sync::atomic::{AtomicUsize, Ordering};

static OBJECTID: AtomicUsize = AtomicUsize::new(0);

//#region Object and Vertex
pub struct Object {
    pub transform: Transform,
    verticies: Vec<Vertex>,
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
            verticies: Vec::new()
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
            ]
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
    screen: Screen,
    pub time: Time
}

impl Simulator {
    pub fn new() -> Self {
        let screen = Screen::new()
            .set_size(512, 512)
            .use_pixel_buffer()
            .build();
        Self {
            objects: HashMap::new(),
            renderer: Renderer::new(renderer::RenderMode::R3D),
            time: Time::new(),
            screen
        }
    }

    pub fn start(&mut self) {
        self.screen.open();
    }

    pub fn update(&mut self) -> Result<usize, ()> {

        if !self.screen.is_open() {
            return Err(());
        }

        let delta = self.time.update();

        self.screen.clear();

        for obj in self.objects.values_mut() {

            let mut projected_vertexs = vec![(-1, -1); 8];

            for i in 0..obj.verticies.len() {
                let projected_coords = self.renderer.project_to_screen(&obj.transform, &obj.verticies[i].rel_pos, self.screen.get_window_size());
                projected_vertexs[i] = projected_coords;
            }
            
            for i in 0..projected_vertexs.len() {
                self.screen.draw_point(
                    (projected_vertexs[i].0, projected_vertexs[i].1)
                );
                for o in obj.verticies[i].connects.iter() {
                    self.screen.draw_line(
                        (projected_vertexs[i].0, projected_vertexs[i].1), 
                        (
                            projected_vertexs[*o].0,
                            projected_vertexs[*o].1
                        )
                    );
                }
            }

        }
        
        self.screen.refresh();

        Ok(delta)
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