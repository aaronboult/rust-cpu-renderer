mod display;
use display::{Display};

use std::{thread, time};

mod renderer;
pub use renderer::geometry::{Vector2D, Vector3D};

pub fn make() {

    let renderer = renderer::Renderer::new(renderer::RenderMode::R3D);

    let cube_vertexs: [Vector3D; 8] = [        
        Vector3D::new(-1.0, -1.0, -1.0), Vector3D::new(1.0, -1.0, -1.0), // ltb, rtb
        Vector3D::new(-1.0, -1.0, 1.0), Vector3D::new(1.0, -1.0, 1.0), // ltf, rtf
        Vector3D::new(-1.0, 1.0, -1.0), Vector3D::new(1.0, 1.0, -1.0), // lbb, rbb
        Vector3D::new(-1.0, 1.0, 1.0), Vector3D::new(1.0, 1.0, 1.0) // lbf, rbf
    ];

    let vertex_connections: [[usize; 3]; 8] = [
        [1, 2, 4], [0, 3, 5], // ltb, rtb
        [0, 3, 6], [1, 2, 7], // ltf, rtf
        [0, 5, 6], [1, 4, 7], // lbb, rbb
        [2, 4, 7], [3, 5, 6] // lbf, rbf
    ];

    let mut projected_vertexs: [(i32, i32); 8] = [(-1, -1); 8];

    let mut cube_transform = renderer::Transform::new();
    cube_transform.position.z = -20.0;

    let mut d = Display::new()
        .set_size(512, 512)
        .use_pixel_buffer()
        .build();

    d.open();

    while d.is_open() {

        d.clear();

        for i in 0..cube_vertexs.len() {
            let projected_coords = renderer.project_to_screen(&cube_transform, &cube_vertexs[i], d.get_window_size());
            projected_vertexs[i] = projected_coords;
        }
    
        for i in 0..projected_vertexs.len() {
            let current_vertex = (projected_vertexs[i].0 as u32, projected_vertexs[i].1 as u32);
            d.draw_point(current_vertex);
            for o in vertex_connections[i].iter() {
                d.draw_line(
                    current_vertex, 
                    (
                        projected_vertexs[*o].0 as u32,
                        projected_vertexs[*o].1 as u32
                    )
                );
            }
        }

        cube_transform.rotation.x += 1.0;
        cube_transform.rotation.y += 1.0;
        cube_transform.rotation.z += 1.0;

        thread::sleep(time::Duration::from_millis(20));

    }

    d.await_close();

}