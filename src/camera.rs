use cgmath::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

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

        return OPENGL_TO_WGPU_MATRIX * proj * view;
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
