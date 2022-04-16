use crate::objects::transform::Transform;

use rust_try_lib::cgmath::*;

pub struct Camera {
    transform: Transform,
    speed: f32,
    rotate_speed: Rad<f32>,
}

impl Camera {
    const FRONT: Vector3<f32> = vec3(1.0, 0.0, 0.0);

    pub fn new(
        position: Point3<f32>,
        front: Vector3<f32>,
        speed: f32,
        rotate_speed: impl Into<Rad<f32>>,
    ) -> Self {
        Self {
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

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position(),
            self.rotation().rotate_vector(Self::FRONT),
            vec3(0.0, 1.0, 0.0),
        )
    }
}

impl Camera {
    pub fn r#move(&mut self, delta: f32, forward: f32, right: f32, up: f32) {
        let mut direction = self.rotation().rotate_vector(vec3(forward, 0.0, right));
        direction[2] += up;
        if direction.magnitude2() > 1.0 {
            direction = direction.normalize();
        }
        self.transform.r#move(self.speed * delta * direction);
    }

    pub fn rotate(&mut self, rad: impl Into<Rad<f32>>, to_right: f32, to_up: f32) {
        let front = self.rotation().rotate_vector(Self::FRONT);

        let mut dest = self.rotation().rotate_vector(vec3(0.0, to_up, to_right));
        if to_up.abs() > f32::EPSILON || to_right.abs() > f32::EPSILON {
            dest = dest.normalize();
        }

        let axis = front.cross(dest);
        let rotation = Quaternion::from_axis_angle(axis, self.rotate_speed * rad.into().0);

        self.transform.rotate(rotation);
    }
}
