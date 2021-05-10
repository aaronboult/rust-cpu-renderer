pub mod linearalgebra;
use linearalgebra::{Matrix, Vector2D, Vector3D};

#[derive(Copy, Clone, Debug)]
pub enum OriginPosition {
    TOPLEFT,
    TOPRIGHT,
    BOTTOMLEFT,
    BOTTOMRIGHT,
    TOPMIDDLE,
    BOTTOMMIDDLE,
    MIDDLELEFT,
    MIDDLERIGHT,
    MIDDLEMIDDLE
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum RenderMode {
    R2D,
    R3D
}

pub struct Camera {
    transform: Transform,
    view_plane_top_left: Vector2D,
    view_plane_bottom_right: Vector2D,
    view_plane_z: f32
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            view_plane_top_left: Vector2D::new(-1.0, 1.0),
            view_plane_bottom_right: Vector2D::new(1.0, -1.0),
            view_plane_z: -1.0
        }
    }
}

pub struct Renderer {
    mode: RenderMode,
    camera: Camera,
    resolution_x: f32,
    resolution_y: f32,
    allow_3d_rotation: bool,
    origin: OriginPosition
}

impl Renderer {
    pub fn new(mode: RenderMode, origin: OriginPosition, allow_3d_rotation: bool) -> Self {
        Self {
            mode,
            camera: Camera::new(),
            resolution_x: 1920.0,
            resolution_y: 1080.0,
            allow_3d_rotation,
            origin
        }
    }

    pub fn get_resolution(&self) -> (f32, f32) {
        (self.resolution_x, self.resolution_y)
    }

    pub fn set_resolution(&mut self, resolution_x: f32, resolution_y: f32) {
        self.resolution_x = resolution_x;
        self.resolution_y = resolution_y;
    }

    pub fn allow_3d_rotation(&mut self) {
        self.allow_3d_rotation = true;
    }

    pub fn disable_3d_rotation(&mut self) {
        self.allow_3d_rotation = true;
    }

    fn get_origin(&self, window_size: (i32, i32)) -> (i32, i32) {
        match self.origin {
            OriginPosition::TOPLEFT => (0, 0),
            OriginPosition::TOPRIGHT => (window_size.0, 0),
            OriginPosition::BOTTOMLEFT => (0, window_size.1),
            OriginPosition::BOTTOMRIGHT => window_size,
            OriginPosition::TOPMIDDLE => (window_size.0 / 2, 0),
            OriginPosition::BOTTOMMIDDLE => (window_size.0 / 2, window_size.1),
            OriginPosition::MIDDLELEFT => (0, window_size.1 / 2),
            OriginPosition::MIDDLERIGHT => (window_size.0, window_size.1 / 2),
            OriginPosition::MIDDLEMIDDLE => (window_size.0 / 2, window_size.1 / 2),
        }
    }

    pub fn project_to_screen(&self, transform: &Transform, vertex: &Vector3D, window_size: (i32, i32)) -> (i32, i32) {
        match self.mode {
            RenderMode::R2D => {
                self.calculate_2d_projection(transform, vertex, self.get_origin(window_size))
            },
            RenderMode::R3D => {
                self.calculate_3d_projection(transform, vertex, self.get_origin(window_size))
            }
        }
    }

    pub fn calculate_2d_projection(&self, transform: &Transform, vertex: &Vector3D, origin_pos: (i32, i32)) -> (i32, i32) {
        if self.allow_3d_rotation {
            let point = Vector3D::new(
                vertex.x,
                vertex.y,
                vertex.z
            );
    
            let rotation = transform.rotation * (std::f32::consts::PI / 180.0);
        
            // scale the initial point
            let projected_point = Matrix::from(point * transform.scale);
    
            let x_rotation_matrix = Matrix::from_vec(3, 3, vec![
                1.0, 0.0, 0.0,
                0.0, rotation.x.cos(), -rotation.x.sin(),
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
                + transform.position
                - self.camera.transform.position;
            
            (
                (rotated_projection.x + transform.position.x) as i32 + origin_pos.0,
                (rotated_projection.y + transform.position.y) as i32 + origin_pos.1
            )
        }
        else {
            (
                (vertex.x + transform.position.x) as i32 + origin_pos.0,
                (vertex.y + transform.position.y) as i32 + origin_pos.1
            )
        }
    }

    pub fn calculate_3d_projection(&self, transform: &Transform, vertex: &Vector3D, origin_pos: (i32, i32)) -> (i32, i32) {
        #[cfg(feature="renderer_profile")]
        let projection_calculation_timer = Instant::now();

        let point = Vector3D::new(
            vertex.x,
            vertex.y,
            vertex.z
        );

        let rotation = transform.rotation * (std::f32::consts::PI / 180.0);
    
        // scale the initial point
        let projected_point = Matrix::from(point * transform.scale);

        let x_rotation_matrix = Matrix::from_vec(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, rotation.x.cos(), -rotation.x.sin(),
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
            + transform.position
            - self.camera.transform.position;

        let recording_screen_size = self.camera.view_plane_bottom_right
            - self.camera.view_plane_top_left;

        let mut x: f32 = -1.0;
        let mut y: f32 = -1.0;

        if rotated_projection.z != 0.0 {
            x = (rotated_projection.x * self.resolution_x) / (rotated_projection.z / recording_screen_size.x) * self.camera.view_plane_z;
            y = (rotated_projection.y * self.resolution_y) / (rotated_projection.z / recording_screen_size.y) * self.camera.view_plane_z;
        }

        #[cfg(feature="renderer_profile")]
        println!("Projection Calculation Time: {}ms", projection_calculation_timer.elapsed().as_millis());

        (
            x as i32 + origin_pos.0,
            y as i32 + origin_pos.1,
        )
    
    }
}

#[derive(Default, Copy, Clone)]
pub struct Transform {
    pub position: Vector3D,
    pub rotation: Vector3D,
    pub scale: Vector3D
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3D::ZERO,
            rotation: Vector3D::ZERO,
            scale: Vector3D::ONE
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation.x = x;
        self.rotation.y = y;
        self.rotation.z = z;
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale.x = x;
        self.scale.y = y;
        self.scale.z = z;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.translate_x(x);
        self.translate_y(y);
        self.translate_z(z);
    }

    pub fn translate_x(&mut self, x: f32) {
        self.position.x += x;
    }

    pub fn translate_y(&mut self, y: f32) {
        self.position.y += y;
    }

    pub fn translate_z(&mut self, z: f32) {
        self.position.z += z;
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotate_x(x);
        self.rotate_y(y);
        self.rotate_z(z);
    }

    pub fn rotate_x(&mut self, x: f32) {
        self.rotation.x += x;
        if self.rotation.x < 0.0 {
            self.rotation.x += 360.0;
        }
        else if self.rotation.x > 360.0 {
            self.rotation.x -= 360.0;
        }
    }

    pub fn rotate_y(&mut self, y: f32) {
        self.rotation.y += y;
        if self.rotation.y < 0.0 {
            self.rotation.y += 360.0;
        }
        else if self.rotation.y > 360.0 {
            self.rotation.y -= 360.0;
        }
    }

    pub fn rotate_z(&mut self, z: f32) {
        self.rotation.z += z;
        if self.rotation.z < 0.0 {
            self.rotation.z += 360.0;
        }
        else if self.rotation.z > 360.0 {
            self.rotation.z -= 360.0;
        }
    }

    pub fn rotate_2d(&mut self, angle: f32) {
        self.rotate_z(angle);
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale_x(x);
        self.scale_y(y);
        self.scale_z(z);
    }

    pub fn scale_x(&mut self, x: f32) {
        self.scale.x += x;
    }

    pub fn scale_y(&mut self, y: f32) {
        self.scale.y += y;
    }

    pub fn scale_z(&mut self, z: f32) {
        self.scale.z += z;
    }
}

impl std::fmt::Debug for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowBuilder")
            .field("Position", &format!("{}", self.position))
            .field("Rotation", &format!("{}", self.rotation))
            .field("Scale", &format!("{}", self.scale))
            .finish()
    }
}