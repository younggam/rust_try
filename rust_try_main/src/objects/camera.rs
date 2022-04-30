use crate::objects::transform::{Transform, *};

use rust_try_lib::{cgmath::*, inputs::*, utils::Utils, winit, winit::window::WindowId};

pub struct Camera {
    projection: PerspectiveFov<f32>,

    transform: Transform<EulerLike<Rad<f32>>>,
    speed: f32,
    rotate_speed: Rad<f32>,
}

impl Camera {
    const FRONT: Vector3<f32> = vec3(0.0, 0.0, -1.0);

    pub fn new(
        aspect: f32,
        position: Point3<f32>,
        front: Vector3<f32>,
        speed: f32,
        rotate_speed: impl Into<Rad<f32>>,
    ) -> Self {
        Self {
            projection: PerspectiveFov {
                fovy: Rad(std::f32::consts::FRAC_PI_4),
                aspect,
                near: 0.1,
                far: 100.0,
            },

            transform: Transform::new(
                position,
                EulerLike::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                vec3(1.0, 1.0, 1.0),
            ),
            speed,
            rotate_speed: rotate_speed.into(),
        }
    }

    pub fn screen_motion_to_camera_motion(screen_motion: Vector2<f32>) -> Vector3<f32> {
        vec3(screen_motion.x, -screen_motion.y, 0.0)
    }

    pub fn position(&self) -> Point3<f32> {
        self.transform.position()
    }

    pub fn rotation(&self) -> Quaternion<f32> {
        self.transform.rotation()
    }

    pub fn scale(&self) -> Vector3<f32> {
        self.transform.scale()
    }
}

impl Camera {
    pub fn handle_input(&mut self, target_window_id: WindowId, utils: &Utils, inputs: &Inputs) {
        if let Some(keyboard) = inputs.window_keyboard(target_window_id) {
            let forward = if keyboard.is_pressed(KeyCode::W) {
                1f32
            } else {
                0f32
            };
            let backward = if keyboard.is_pressed(KeyCode::S) {
                1f32
            } else {
                0f32
            };
            let left = if keyboard.is_pressed(KeyCode::A) {
                1f32
            } else {
                0f32
            };
            let right = if keyboard.is_pressed(KeyCode::D) {
                1f32
            } else {
                0f32
            };
            let up = if keyboard.is_pressed(KeyCode::Space) {
                1f32
            } else {
                0f32
            };
            let down = if keyboard.is_pressed(KeyCode::LShift) {
                1f32
            } else {
                0f32
            };
            self.r#move(
                utils.time_delta() as f32,
                vec3(forward - backward, right - left, up - down),
            );
        };

        if let Some(cursor) = inputs.cursor(target_window_id) {
            if let Some(mouse) = inputs.device_mouse(None) {
                let motion = if cursor.is_just_entered() {
                    mouse.last_motion()
                } else if cursor.is_entered() {
                    mouse.motion()
                } else if cursor.is_just_left() {
                    mouse.first_motion()
                } else {
                    Vector2::zero()
                };
                self.rotate(motion);
            }
        }
    }

    pub fn proj_matrix(&self) -> Matrix4<f32> {
        let persp = self.projection;
        let f = Rad::cot(persp.fovy / 2.0);
        let d = persp.far - persp.near;
        Matrix4::from_cols(
            vec4(f / persp.aspect, 0.0, 0.0, 0.0),
            vec4(0.0, f, 0.0, 0.0),
            vec4(0.0, 0.0, -0.5 * (persp.far + persp.near) / d - 0.5, -1.0),
            vec4(0.0, 0.0, -persp.far * persp.near / d, 0.0),
        )
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let front = self.rotation().rotate_vector(Self::FRONT);
        let up = vec3(0.0, 1.0, 0.0);

        Matrix4::look_to_rh(self.position(), front, up)
    }

    pub fn view_proj_matrix(&self) -> Matrix4<f32> {
        self.proj_matrix() * self.view_matrix()
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let new_size = new_size.cast::<f32>();
        self.projection.aspect = new_size.width / new_size.height;
    }

    pub fn r#move(&mut self, delta: f32, dir: Vector3<f32>) {
        if dir.x.abs() <= f32::EPSILON && dir.y.abs() <= f32::EPSILON && dir.z.abs() <= f32::EPSILON
        {
            return;
        }

        let mut to = self.rotation().rotate_vector(vec3(dir.y, 0.0, -dir.x));
        to[2] += dir.z;

        self.transform.r#move(self.speed * delta * to.normalize());
    }

    pub fn rotate(&mut self, screen_motion: Vector2<f32>) {
        let dir = Self::screen_motion_to_camera_motion(screen_motion);
        if dir.x.abs() <= f32::EPSILON && dir.y.abs() <= f32::EPSILON && dir.z.abs() <= f32::EPSILON
        {
            return;
        }

        let coeff = self.rotate_speed * dir.magnitude();
        let dir = dir.normalize();

        let rotation = EulerLike::new(coeff * dir.y, coeff * -dir.x, Rad(0.0));

        self.transform.rotate(rotation);
    }
}
