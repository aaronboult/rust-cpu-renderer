use super::renderer::{Transform};
use super::renderer::linearalgebra::{Vector3D};
use super::{Simulator, Color};

// used for ID generation
use std::sync::atomic::{AtomicUsize, Ordering};

static OBJECTID: AtomicUsize = AtomicUsize::new(0);

//#region Object and Vertex
pub trait Object {
    fn get_id(&self) -> usize;
    fn register(self, registrar: &mut Simulator) -> usize;
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
    fn get_verticies(&self) -> &Vec<Vertex>;
    fn get_frame_color(&self) -> Color;
    fn set_frame_color(&mut self, color: Color);
    fn get_fill_color(&self) -> Color;
    fn set_fill_color(&mut self, color: Color);
    fn get_cached_transform(&self) -> &Transform;
    fn cache_transform(&mut self);
}

impl dyn Object {
    fn new_id() -> usize {
        let id = OBJECTID.fetch_add(1, Ordering::SeqCst);
        if id == usize::MAX {
            OBJECTID.store(0, Ordering::SeqCst);
        }
        id
    }
}

pub struct Cube {
    transform: Transform,
    cached_transform: Transform,
    verticies: Vec<Vertex>,
    frame_color: Color,
    fill_color: Color,
    id: usize,
}

impl Object for Cube {
    fn get_id(&self) -> usize {
        self.id
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
    fn get_verticies(&self) -> &Vec<Vertex> {
        &self.verticies
    }
    fn get_frame_color(&self) -> Color {
        self.frame_color
    }
    fn set_frame_color(&mut self, color: Color) {
        self.frame_color = color;
    }
    fn get_fill_color(&self) -> Color {
        self.fill_color
    }
    fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color;
    }
    fn register(self, registrar: &mut Simulator) -> usize {
        registrar.add_object(Box::new(self))
    }
    fn get_cached_transform(&self) -> &Transform {
        &self.cached_transform
    }
    fn cache_transform(&mut self) {
        self.cached_transform = self.transform;
    }
}

impl Cube {
    pub fn new() -> Self {
        Self {
            id: Object::new_id(),
            transform: Transform::new(),
            cached_transform: Transform::new(),
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
            fill_color: Color::BLACK,
        }
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

    pub fn get_rel_pos(&self) -> Vector3D {
        self.rel_pos
    }

    pub fn get_connections(&self) -> &Vec<usize> {
        &self.connects
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