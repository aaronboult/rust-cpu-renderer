pub mod geometry;
use geometry::{Vector, Matrix, Vector2D, Vector3D};

use super::{Transform};

pub enum RenderMode {
    R_2D,
    R_3D
}

pub struct Camera<T> where T: Transform {
    transform: T,
    view_plane_top_left: Vector2D,
    view_plane_bottom_right: Vector2D,
    view_plane_z: f32
}

impl<T> Camera<T> where T: Transform {
    pub fn new() -> Self {
        Self {
            transform: T::default(),
            view_plane_top_left: Vector2D::new(-1.0, 1.0),
            view_plane_bottom_right: Vector2D::new(1.0, -1.0),
            view_plane_z: -1.0
        }
    }
}

pub struct Renderer<T> where T: Transform {
    mode: RenderMode,
    camera: Camera<T>,
    scale: f32
}

impl<T> Renderer<T> where T: Transform {
    pub fn new(mode: RenderMode) -> Self {
        Self {
            mode,
            camera: Camera::new(),
            scale: 1.0
        }
    }

    pub fn project_to_screen<U>(&self, transform: T, point: U) -> Vector2D where T: Transform, U: Vector {
        match self.mode {
            RenderMode::R_2D => {
                Vector2D::ZERO
            },
            RenderMode::R_3D => {
                self.calculate_3d_projection(transform, point)
            }
        }
    }

    fn calculate_3d_projection<U>(&self, transform: T, point: U) -> Vector2D where T: Transform, U: Vector {

        let point = Vector3D::new(
            point.get_x(),
            point.get_y(),
            point.get_z()
        );

        let rotation = transform.get_rotation();
    
        // scale the initial point
        let projected_point = Matrix::from(point * transform.get_scale());

        let x_rotation_matrix = Matrix::from_vec(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, rotation.x.cos(), -rotation.y.cos(),
            0.0, rotation.x.sin(), rotation.x.cos()
        ]);

        let y_rotation_matrix = Matrix::from_vec(3, 3, vec![
            rotation.y.cos(), 0.0, -rotation.y.sin(),
            0.0, 1.0, 0.0,
            rotation.y.sin(), 0.0, rotation.y.cos()
        ]);

        let z_rotation_matrix = Matrix::from_vec(3, 3, vec![
            rotation.z.cos(), -rotation.z.sin(), 0.0,
            rotation.z.sin(), rotation.z.cos(), 0.0,
            0.0, 0.0, 1.0
        ]);

        let rotation_matrix = x_rotation_matrix * y_rotation_matrix * z_rotation_matrix;

        // multiplying rotation matrix with 3d column vector 
        // produces another 3d column vector
        let rotated_projection = Vector3D::from(rotation_matrix * projected_point)
            + transform.get_position()
            - self.camera.transform.get_position();

        let recording_screen_size = self.camera.view_plane_bottom_right
            - self.camera.view_plane_top_left;
        
        if rotated_projection.z > 0.0 {
            let resolution_x = 512.0;
            let resolution_y = 512.0;
            return Vector2D::new(
                (rotated_projection.x * resolution_x) / (rotated_projection.z / recording_screen_size.x) * self.camera.view_plane_z,
                (rotated_projection.y * resolution_y) / (rotated_projection.z / recording_screen_size.y) * self.camera.view_plane_z,
            );
        }

        -Vector2D::ONE
    
    }
}