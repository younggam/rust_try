use rust_try_lib::cgmath::*;

use std::ops::AddAssign;

pub struct Transform<T: Into<Quaternion<f32>> + Copy> {
    position: Point3<f32>,
    rotation: T,
    scale: Vector3<f32>,
}

impl<T: Into<Quaternion<f32>> + Copy> Transform<T> {
    pub fn new(position: Point3<f32>, rotation: T, scale: Vector3<f32>) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    pub fn rotation(&self) -> Quaternion<f32> {
        self.rotation.into()
    }

    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }
}

impl<T: Into<Quaternion<f32>> + Copy> Transform<T> {
    pub fn r#move(&mut self, velocity: Vector3<f32>) {
        self.position += velocity;
    }

    pub fn scaling(&mut self, scale: Vector3<f32>) {
        self.scale += scale;
    }
}

impl Transform<Quaternion<f32>> {
    pub fn rotate(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation * self.rotation;
    }
}

impl Transform<EulerLike<Rad<f32>>> {
    pub fn rotate(&mut self, rotation: EulerLike<Rad<f32>>) {
        self.rotation += rotation;
    }
}

//

#[derive(Clone, Copy)]
pub struct EulerLike<A> {
    x_angle: A,
    y_angle: A,
    z_angle: A,
    cache: Quaternion<f32>,
}

impl<A: Into<Rad<f32>> + Copy> EulerLike<A> {
    pub fn new(x_angle: A, y_angle: A, z_angle: A) -> Self {
        let mut ret = Self {
            x_angle,
            y_angle,
            z_angle,
            cache: Quaternion::one(),
        };
        ret.update_cache();
        ret
    }

    pub fn update_cache(&mut self) {
        let y_rot = Quaternion::from_angle_y(self.y_angle.into());
        let xy_rot = Quaternion::from_axis_angle(
            y_rot.rotate_vector(Vector3::unit_x()),
            self.x_angle.into(),
        ) * y_rot;

        self.cache = Quaternion::from_axis_angle(
            xy_rot.rotate_vector(Vector3::unit_z()),
            self.z_angle.into(),
        ) * xy_rot;
    }
}

impl AddAssign<EulerLike<Rad<f32>>> for EulerLike<Rad<f32>> {
    fn add_assign(&mut self, other: Self) {
        const LIMIT: Rad<f32> = Rad(std::f32::consts::FRAC_PI_2 - std::f32::consts::PI / 180.0);

        self.x_angle += other.x_angle;
        if self.x_angle > LIMIT {
            self.x_angle = LIMIT;
        }
        if self.x_angle < -LIMIT {
            self.x_angle = -LIMIT;
        }
        self.y_angle += other.y_angle;
        self.z_angle += other.z_angle;

        self.update_cache();
    }
}

impl AddAssign<EulerLike<Deg<f32>>> for EulerLike<Deg<f32>> {
    fn add_assign(&mut self, other: Self) {
        const LIMIT: Deg<f32> = Deg(89.0);

        self.x_angle += other.x_angle;
        if self.x_angle > LIMIT {
            self.x_angle = LIMIT;
        }
        if self.x_angle < -LIMIT {
            self.x_angle = -LIMIT;
        }
        self.y_angle += other.y_angle;
        self.z_angle += other.z_angle;

        self.update_cache();
    }
}

impl<A> From<EulerLike<A>> for Quaternion<f32>
where
    A: Angle<Unitless = f32> + Into<Rad<f32>>,
{
    fn from(euler: EulerLike<A>) -> Self {
        euler.cache
    }
}
