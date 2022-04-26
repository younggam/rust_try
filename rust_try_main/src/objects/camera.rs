use crate::objects::transform::Transform;

use rust_try_lib::{cgmath::*, winit};

pub struct Camera {
    projection: PerspectiveFov<f32>,

    transform: Transform,
    speed: f32,
    rotate_speed: Rad<f32>,
}

impl Camera {
    const FRONT: Vector3<f32> = vec3(1.0, 0.0, 0.0);
    const RIGHT: Vector3<f32> = vec3(0.0, 0.0, 1.0);

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
                Quaternion::from_arc(Self::FRONT, front, None),
                vec3(1.0, 1.0, 1.0),
            ),
            speed,
            rotate_speed: rotate_speed.into(),
        }
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
        let pos = self.position();
        let f = self.rotation().rotate_vector(Self::FRONT);
        let r = self.rotation().rotate_vector(Self::RIGHT);
        let u = r.cross(f).normalize();

        Matrix4::from_cols(
            vec4(r.x, u.x, -f.x, 0.0),
            vec4(r.y, u.y, -f.y, 0.0),
            vec4(r.z, u.z, -f.z, 0.0),
            vec4(-pos.dot(r), -pos.dot(u), pos.dot(f), 1.0),
        )
    }

    pub fn view_proj_matrix(&self) -> Matrix4<f32> {
        self.proj_matrix() * self.view_matrix()
    }
}

impl Camera {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let new_size = new_size.cast::<f32>();
        self.projection.aspect = new_size.width / new_size.height;
    }

    pub fn r#move(&mut self, delta: f32, forward: f32, right: f32, up: f32) {
        if forward.abs() <= f32::EPSILON && right.abs() <= f32::EPSILON && up.abs() <= f32::EPSILON
        {
            return;
        }

        let mut direction = self.rotation().rotate_vector(vec3(forward, 0.0, right));
        direction[2] += up;

        self.transform
            .r#move(self.speed * delta * direction.normalize());
    }

    pub fn rotate(&mut self, to_right: f32, to_up: f32) {
        if to_up.abs() <= f32::EPSILON && to_right.abs() <= f32::EPSILON {
            return;
        }

        let dest = vec3(0.0, to_up, to_right);
        let magnitude = dest.magnitude();
        let axis = self
            .rotation()
            .rotate_vector(Self::FRONT.cross(dest.normalize()));

        let rotation = Quaternion::from_axis_angle(axis, self.rotate_speed * magnitude);

        self.transform.rotate(rotation);
    }

    pub fn rotate2(&mut self, to_right: f32, to_up: f32) {
        if to_up.abs() <= f32::EPSILON && to_right.abs() <= f32::EPSILON {
            return;
        }

        let to = vec2(to_up, to_right);
        let mag = to.magnitude();
        let to = to.normalize();
    }
}
