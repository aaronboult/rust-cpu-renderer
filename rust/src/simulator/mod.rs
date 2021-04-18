#![allow(dead_code)]

mod display;
use display::Screen;

use std::{thread, time};

mod renderer;
use renderer::{Transform, Renderer};
pub use renderer::geometry::{Vector2D, Vector3D};

struct Object {
    transform: Transform,
    verticies: Vec<Vertex>
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            verticies: Vec::new()
        }
    }
    pub fn new_cube() -> Self {
        Self {
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

pub struct Simulator {
    objects: Vec<Object>,
    renderer: Renderer,
    screen: Screen
}

impl Simulator {
    pub fn new() -> Self {
        let screen = Screen::new()
            .set_size(512, 512)
            .use_pixel_buffer()
            .build();
        Self {
            objects: Vec::new(),
            renderer: Renderer::new(renderer::RenderMode::R3D),
            screen
        }
    }

    pub fn start(&mut self) {

        let mut cube1 = Object::new_cube();
        cube1.transform.position.z = -20.0;
        cube1.transform.position.x = -1.0;
        self.objects.push(cube1);

        self.screen.open();

        'main: loop {
            if !self.screen.is_open() {
                break 'main;
            }
        
            self.screen.clear();

            for obj in self.objects.iter_mut() {
                
                let mut projected_vertexs = vec![(-1, -1); 8];

                for i in 0..obj.verticies.len() {
                    let projected_coords = self.renderer.project_to_screen(&obj.transform, &obj.verticies[i].rel_pos, self.screen.get_window_size());
                    projected_vertexs[i] = projected_coords;
                }
                
                for i in 0..projected_vertexs.len() {
                    let current_vertex = (projected_vertexs[i].0 as u32, projected_vertexs[i].1 as u32);
                    self.screen.draw_point(current_vertex);
                    for o in obj.verticies[i].connects.iter() {
                        self.screen.draw_line(
                            current_vertex, 
                            (
                                projected_vertexs[*o].0 as u32,
                                projected_vertexs[*o].1 as u32
                            )
                        );
                    }
                }

                obj.transform.rotation.x += 1.0;
                obj.transform.rotation.y += 1.0;
                obj.transform.rotation.z += 1.0;
        
                thread::sleep(time::Duration::from_millis(20));
    
            }
            
            self.screen.refresh();
        }
    }
}