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
    fn get_vertices(&self) -> &Vec<Vertex>;
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

#[derive(Clone, Debug)]
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

//#region Cube
pub struct Cube {
    transform: Transform,
    cached_transform: Transform,
    vertices: Vec<Vertex>,
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
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
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
            vertices: vec![
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

    pub fn set_position(mut self, x: f32, y: f32, z: f32) -> Self {
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
//#endregion

//#region Spot
pub struct Spot {
    transform: Transform,
    cached_transform: Transform,
    vertex: Vec<Vertex>,
    color: Color,
    id: usize
}

impl Object for Spot {
    fn get_id(&self) -> usize {
        self.id
    }
    fn register(self, registrar: &mut Simulator) -> usize {
        registrar.add_object(Box::new(self))
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertex
    }
    fn get_frame_color(&self) -> Color {
        self.get_fill_color()
    }
    fn set_frame_color(&mut self, color: Color) {
        self.set_fill_color(color);
    }
    fn get_fill_color(&self) -> Color {
        self.color
    }
    fn set_fill_color(&mut self, color: Color) {
        self.color = color;
    }
    fn get_cached_transform(&self) -> &Transform {
        &self.cached_transform
    }
    fn cache_transform(&mut self) {
        self.cached_transform = self.transform;
    }
}

impl Spot {
    pub fn new(color: Color) -> Self {
        Self {
            transform: Transform::new(),
            cached_transform: Transform::new(),
            vertex: vec!{
                Vertex {
                    rel_pos: Vector3D::new(0.0, 0.0, 0.0),
                    connects: vec![]
                }
            },
            color,
            id: Object::new_id(),
        }
    }

    pub fn set_position(mut self, x: f32, y: f32) -> Self {
        self.transform.set_position(x, y, self.transform.position.z);
        self
    }

    pub fn set_rotation(mut self, x: f32, y: f32) -> Self {
        self.transform.set_rotation(x, y, self.transform.position.z);
        self
    }

    pub fn set_scale(mut self, x: f32, y: f32) -> Self {
        self.transform.set_scale(x, y, self.transform.position.z);
        self
    }
}
//#endregion

//#region
pub struct Circle {
    transform: Transform,
    cached_transform: Transform,
    frame_color: Color,
    fill_color: Color,
    radius: f32,
    vertices: Vec<Vertex>,
    id: usize
}

impl Object for Circle {
    fn get_id(&self) -> usize {
        self.id
    }
    fn register(self, registrar: &mut Simulator) -> usize {
        registrar.add_object(Box::new(self))
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
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
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    fn cache_transform(&mut self) {
        self.cached_transform = self.transform;
    }
    fn get_cached_transform(&self) -> &Transform {
        &self.cached_transform
    }
}

impl Circle {
    pub fn new(frame_color: Color, fill_color: Color) -> Self {
        let transform = Transform::new();
        Self {
            transform,
            cached_transform: transform,
            frame_color: frame_color,
            fill_color: fill_color,
            radius: 1.0,
            vertices: Circle::get_vertices(1.0),
            id: Object::new_id()
        }
    }

    pub fn set_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self.vertices = Circle::get_vertices(radius);
        self
    }

    pub fn set_position(mut self, x: f32, y: f32) -> Self {
        self.transform.set_position(x, y, self.transform.position.z);
        self
    }

    pub fn set_rotation(mut self, x: f32, y: f32) -> Self {
        self.transform.set_rotation(x, y, self.transform.position.z);
        self
    }

    pub fn set_scale(mut self, x: f32, y: f32) -> Self {
        self.transform.set_scale(x, y, self.transform.position.z);
        self
    }

    fn get_vertices(radius: f32) -> Vec<Vertex> {
        let step: f32 = (2.0 * std::f32::consts::PI) / 10_000.0;
        let mut angle: f32 = 0.0;
        let mut vertices = Vec::new();
        while angle < 2.0 * std::f32::consts::PI {
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let mut vertex = Vertex::new(Vector3D::new(x, y, 0.0));
            if vertices.len() != 0 {
                vertex.add_connection(vertices.len() - 1);
                // println!("Adding: {} to {:?}", vertices.len() - 1, vertex.get_rel_pos());
            }
            vertices.push(vertex);
            angle += step;
        }
        let number_of_vertices = vertices.len();
        vertices[number_of_vertices - 1].add_connection(0);
        vertices
    }
}
//#endregion

//#region Quad
pub struct Quad {
    transform: Transform,
    cached_transform: Transform,
    frame_color: Color,
    fill_color: Color,
    vertices: Vec<Vertex>,
    id: usize
}

impl Object for Quad {
    fn get_id(&self) -> usize {
        self.id
    }
    fn register(self, registrar: &mut Simulator) -> usize {
        registrar.add_object(Box::new(self))
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
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
    fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    fn cache_transform(&mut self) {
        self.cached_transform = self.transform;
    }
    fn get_cached_transform(&self) -> &Transform {
        &self.cached_transform
    }
}

impl Quad {
    pub fn new(top_left: Vertex, top_right: Vertex, bottom_left: Vertex, bottom_right: Vertex) -> Self {
        Self {
            transform: Transform::new(),
            cached_transform: Transform::new(),
            frame_color: Color::BLACK,
            fill_color: Color::BLACK,
            vertices: vec![top_left, top_right, bottom_left, bottom_right],
            id: Object::new_id()
        }
    }
    
    pub fn from_dimensions(width: f32, height: f32) -> Self {
        let mut top_left = Vertex::new(Vector3D::new(-width / 2.0, -height / 2.0, 0.0));
        let mut top_right = Vertex::new(Vector3D::new(-width / 2.0, height / 2.0, 0.0));
        let mut bottom_left = Vertex::new(Vector3D::new(width / 2.0, -height / 2.0, 0.0));
        let bottom_right = Vertex::new(Vector3D::new(width / 2.0, height / 2.0, 0.0));
        top_left.add_connections(vec![1, 2]);
        top_right.add_connections(vec![3]);
        bottom_left.add_connection(3);
        Quad::new(top_left, top_right, bottom_left, bottom_right)
    }
}
//#endregion

//#region Square
pub struct Square {}
impl Square {
    pub fn new(side_length: f32) -> Quad {
        Quad::from_dimensions(side_length, side_length)
    }
}
//#endregion

//#region Rectangle
pub struct Rectangle {}
impl Rectangle {
    pub fn new(width: f32, height: f32) -> Quad {
        Quad::from_dimensions(width, height)
    }
}
//#endregion