use cgmath::*;
use game_loop::winit::event::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

// I should probably make a new() function for this...
pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);

        proj * view
    }

    pub fn build_view_orthographic_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = ortho(
            -self.fovy / 5.0,
            self.fovy / 5.0,
            -self.fovy / 5.0,
            self.fovy / 5.0,
            self.znear,
            self.zfar,
        );

        proj * view
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_position = camera.eye.to_homogeneous().into();
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

// The amount of time I've sunk into naming these variables has withered my soul.
//
pub struct CameraController {
    speed: f32,
    x_movement: f32,
    y_movement: f32,
    z_movement: f32,
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            x_movement: 0.0,
            y_movement: 0.0,
            z_movement: 0.0,
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let amount = if *state == ElementState::Pressed {
                    1.0
                } else {
                    0.0
                };
                match keycode {
                    VirtualKeyCode::D => {
                        self.x_movement = amount;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.x_movement = -amount;
                        true
                    }
                    VirtualKeyCode::Q => {
                        self.y_movement = amount;
                        true
                    }
                    VirtualKeyCode::E => {
                        self.y_movement = -amount;
                        true
                    }
                    VirtualKeyCode::W => {
                        self.z_movement = -amount;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.z_movement = amount;
                        true
                    }
                    VirtualKeyCode::L => {
                        self.x_rotation = amount;
                        true
                    }
                    VirtualKeyCode::J => {
                        self.x_rotation = -amount;
                        true
                    }
                    VirtualKeyCode::U => {
                        self.y_rotation = amount;
                        true
                    }
                    VirtualKeyCode::O => {
                        self.y_rotation = -amount;
                        true
                    }
                    VirtualKeyCode::I => {
                        self.z_rotation = -amount;
                        true
                    }
                    VirtualKeyCode::K => {
                        self.z_rotation = amount;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        let _print_camera_info = |camera: &mut Camera| {
            // Closures are hard.
            println!("\tX\tY\tZ");
            println!(
                "Eye:\t{:.2}\t{:.2}\t{:.2}",
                camera.eye.x, camera.eye.y, camera.eye.z
            );
            println!(
                "Target:\t{:.2}\t{:.2}\t{:.2}",
                camera.target.x, camera.target.y, camera.target.z
            );
            println!();
        };

        camera.eye += Vector3::new(self.x_movement, self.y_movement, self.z_movement) * self.speed;
        camera.target += Vector3::new(
            self.x_rotation + self.x_movement,
            self.y_movement + self.y_rotation,
            self.z_movement + self.z_rotation,
        ) * self.speed;
    }
}
