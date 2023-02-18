use cgmath::*;
use winit::event::*;

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
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

// The amount of time I've sunk into naming these variables has withered my soul.
pub struct CameraController {
    speed: f32,
    x_movement: f32,
    y_movement: f32,
    z_movement: f32,
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
    /*
        is_move_leftward_pressed: bool, // X
        is_move_rightward_pressed: bool,
        is_move_upward_pressed: bool, // Y
        is_move_downward_pressed: bool,
        is_move_frontward_pressed: bool, // Z
        is_move_backward_pressed: bool,
        is_increase_roll_pressed: bool, // Roll, X rotation
        is_decrease_roll_pressed: bool,
        is_increase_pitch_pressed: bool, // Pitch, Y rotation
        is_decrease_pitch_pressed: bool,
        is_increase_yaw_pressed: bool, // Yaw, Z rotation
        is_decrease_yaw_pressed: bool,
    */
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
            /*
            is_move_upward_pressed: false,
            is_move_downward_pressed: false,
            is_move_leftward_pressed: false,
            is_move_rightward_pressed: false,
            is_move_frontward_pressed: false,
            is_move_backward_pressed: false,
            is_increase_roll_pressed: false,
            is_decrease_roll_pressed: false,
            is_increase_pitch_pressed: false,
            is_decrease_pitch_pressed: false,
            is_increase_yaw_pressed: false,
            is_decrease_yaw_pressed: false,*/
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
        //let forward = camera.target - camera.eye;
        //let forward_normal = forward.normalize();
        //let forward_magnitude = forward.magnitude();
        //let right = Vector3::new(1.0, 0.0, 0.0); // To do: derive from the camera's up value.
                                                 /*let up = forward_normal.cross(right); // Optimize this utter garbage.
                                                         if self.is_upward_pressed {
                                                             camera.eye =
                                                                 camera.target - (forward + up * self.speed).normalize() * forward_magnitude;
                                                         }
                                                         if self.is_downward_pressed {
                                                             camera.eye =
                                                                 camera.target - (forward - up * self.speed).normalize() * forward_magnitude;
                                                             camera.target =
                                                                 camera.eye - (forward + up * self.speed).normalize() * forward_magnitude;
                                                         }

                                                         let forward = camera.target - camera.eye;
                                                         let forward_normal = forward.normalize();
                                                         // (Here took the place a magnitude calculation that was not needed.)
                                                 */
        let print_camera_info = |camera: &mut Camera| {
            // Closures are hard.
            return;
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
        camera.target += Vector3::new(self.x_rotation + self.x_movement, self.y_movement + self.y_rotation, self.z_movement + self.z_rotation) * self.speed;

        //let (sin_pitch, cos_pitch) = self.x_rotation.sin_cos();
        //let (sin_yaw, cos_yaw) = self.y_rotation.sin_cos();
        //let (sin_roll, cos_roll) = self.z_rotation.sin_cos();
        //camera.target += Matrix4::look_to_rh(self.position, Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch, sin_yaw).normalize(), Vector3::unit_y(),)

        /*if self.is_move_upward_pressed {
                    camera.eye += camera.up * self.speed;
                    camera.target += camera.up * self.speed;
                }
                if self.is_move_downward_pressed {
                    camera.eye -= camera.up * self.speed;
                    camera.target -= camera.up * self.speed;
                }
                if self.is_move_leftward_pressed {
                    camera.eye -= right * self.speed;
                    camera.target -= right * self.speed;
                }
                if self.is_move_rightward_pressed {
                    camera.eye += right * self.speed;
                    camera.target += right * self.speed;
                }
                if self.is_move_frontward_pressed {
                    camera.eye += forward * self.speed;
                }
                if self.is_move_backward_pressed {
                    camera.eye -= forward * self.speed;
                }
                if self.is_increase_roll_pressed {
                    camera.eye =
                        camera.target + (forward + right * self.speed).normalize() * forward_magnitude;
                }
                if self.is_decrease_roll_pressed {}
                if self.is_increase_pitch_pressed {}
                if self.is_decrease_pitch_pressed {}
                if self.is_increase_yaw_pressed {}
                if self.is_decrease_yaw_pressed {}
        */
        /*
        // I would make this 'left' instead, but I'm too lazy to do that.
        let right = forward_normal.cross(camera.up);

        // Redo calculations in case up or down is pressed.
        let forward = camera.target - camera.eye;
        let forward_magnitude = forward.magnitude();

        // Not sure why the value is getting normalized if it's going to end up the same. It might
        // come handy in the future, so it'll stay here for now.
        // (Would I be using the tangent of the camera if I didn't normalize? It looks the same...)
        if self.is_rightward_pressed {
            camera.eye =
                camera.target - (forward + right * self.speed).normalize() * forward_magnitude;
        }
        if self.is_leftward_pressed {
            camera.eye =
                camera.target - (forward - right * self.speed).normalize() * forward_magnitude;
        }*/
    }
}
/*
const SAFE_FRACT_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    yaw: Rad<f342>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> Matrix<f32> {  //I honestly have no idea how any of this works.
        let (pitch_sin, pitch_cos) = self.pitch.0.sin_cos();
        let (yaw_sin, yaw_cos) = self.yaw.0.sin_cos();

        Matrix4::look_at(
            self.position,
            Vector3::new(
                pitch_cos * yaw_cos,
                pitch_sin,
                pitch_cos * yaw_sin
            ).normalize(),
            Vector3::unit_y(),
        )
    }
}

pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar);
    }
}

#[derive(Debug)]
pub struct CameraController {
    upwards_speed: f32,
    downwards_speed: f32,
    leftwards_speed: f32,
    rightwards_speed: f32,
    forwards_speed: f32,
    backwards_speed: f32,
    horizontal_rotation: f32,
    vertical_rotation: f32,
    scroll: f32,
    speed_multiplier: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed_multiplier: f32, sensitivity: f32) -> Self {
        Self {
            upwards_speed: 0.0,
            downwards_speed: 0.0,
            leftwards_speed: 0.0,
            rightwards_speed: 0.0,
            forwards_speed: 0.0,
            backwards_speed: 0.0,
            horizontal_rotation: 0.0,
            vertical_rotation: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let speed = if state == Element::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::Space => {
                self.forwards_speed = speed;
                true
            }
            VirtualKeyCode::LShift => {
                self.downwards_speed = speed;
                true
            }
            VirtualKeyCode::Left => {
                self.leftwards_speed = speed;
                true
            }
            VirtualKeyCode::Right => {
                self.rightwards_speed = speed;
                true
            }
            VirtualKeyCode::Up => {
                self.upwards_speed = speed;
                true
            }
            VirtualKeyCode::Down => {
                self.downwards_speed = speed;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.horizontal_rotation = mouse_dx as f32;
        self.vertical_rotation = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();

        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let longitudal = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let transversal = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.forwards_speed - self.backwards_speed) * self.speed * dt;
        camera.position += right * (self.rightwards_speed - self.leftwards_speed) * self.speed * dt;

        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        camera.position.y += (self.upwards_speed - self.downwards_speed) * self.speed * dt;

        camera.yaw += Rad(self.horizontal_rotation) * self.sensitivity * dt;
        camera.pitch += Rad(-self.horizontal_rotation) * self.sensitivity * dt;

        self.horizontal_rotation = 0.0;
        self.vertical_rotation = 0.0;

        // If the program crashes, add a radian angle check, if not, have fun.
}*/
