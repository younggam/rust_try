//TODO matrix trait and generic type
use super::vector::*;

use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Mat2 {
    ///col1
    pub a: Vec2,
    ///col2
    pub b: Vec2,
}

impl Mat2 {
    pub const IDENTITY: Mat2 = Self::new(Vec2::UNIT_X, Vec2::UNIT_Y);

    pub const fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    pub const fn new_diagonal(value: Vec2) -> Self {
        Self {
            a: Vec2::new(value.x, 0.0),
            b: Vec2::new(0.0, value.y),
        }
    }
}

impl Default for Mat2 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl AddAssign for Mat2 {
    fn add_assign(&mut self, other: Self) {
        self.a += other.a;
        self.b += other.b;
    }
}

impl Add for Mat2 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Self::Output {
        Self::Output {
            x: self.a.x * vec.x + self.b.x * vec.y,
            y: self.a.y * vec.x + self.b.y * vec.y,
        }
    }
}

impl MulAssign for Mat2 {
    fn mul_assign(&mut self, other: Mat2) {
        let a = *self * other.a;
        let b = *self * other.b;
        self.a = a;
        self.b = b;
    }
}

impl Mul for Mat2 {
    type Output = Mat2;

    fn mul(mut self, other: Mat2) -> Self::Output {
        self *= other;
        self
    }
}

//===================

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Mat3 {
    ///col1
    pub a: Vec3,
    ///col2
    pub b: Vec3,
    ///col3
    pub c: Vec3,
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Self::new(Vec3::UNIT_X, Vec3::UNIT_Y, Vec3::UNIT_Z);

    pub const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }

    pub const fn new_diagonal(value: Vec3) -> Self {
        Self {
            a: Vec3::new(value.x, 0.0, 0.0),
            b: Vec3::new(0.0, value.y, 0.0),
            c: Vec3::new(0.0, 0.0, value.z),
        }
    }

    pub fn translate(self, factor: Vec2) -> Self {
        self * Self::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(factor.x, factor.y, 1.0),
        )
    }

    pub fn scale(self, factor: Vec2) -> Self {
        self * Self::new_diagonal(Vec3::new(factor.x, factor.y, 1.0))
    }

    pub fn rotate(self, radian: f32) -> Self {
        let cos = radian.cos();
        let sin = radian.sin();

        self * Self::new(
            Vec3::new(cos, sin, 0.0),
            Vec3::new(-sin, cos, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl AddAssign for Mat3 {
    fn add_assign(&mut self, other: Self) {
        self.a += other.a;
        self.b += other.b;
        self.c += other.c;
    }
}

impl Add for Mat3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        Self::Output {
            x: self.a.x * vec.x + self.b.x * vec.y + self.c.x * vec.z,
            y: self.a.y * vec.x + self.b.y * vec.y + self.c.y * vec.z,
            z: self.a.z * vec.x + self.b.z * vec.y + self.c.z * vec.z,
        }
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, other: Mat3) {
        let a = *self * other.a;
        let b = *self * other.b;
        let c = *self * other.c;
        self.a = a;
        self.b = b;
        self.c = c;
    }
}

impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(mut self, other: Mat3) -> Self::Output {
        self *= other;
        self
    }
}

//===================

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Mat4 {
    ///col1
    pub a: Vec4,
    ///col2
    pub b: Vec4,
    ///col3
    pub c: Vec4,
    ///col4
    pub d: Vec4,
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Self::new(Vec4::UNIT_X, Vec4::UNIT_Y, Vec4::UNIT_Z, Vec4::UNIT_W);

    pub const fn new(a: Vec4, b: Vec4, c: Vec4, d: Vec4) -> Self {
        Self { a, b, c, d }
    }

    pub const fn new_diagonal(value: Vec4) -> Self {
        Self {
            a: Vec4::new(value.x, 0.0, 0.0, 0.0),
            b: Vec4::new(0.0, value.y, 0.0, 0.0),
            c: Vec4::new(0.0, 0.0, value.z, 0.0),
            d: Vec4::new(0.0, 0.0, 0.0, value.w),
        }
    }

    ///Right Handed, For Vulkan, TODO: config compilation
    pub fn look_at(camera: Vec3, target: Vec3, world_y_axis: Vec3) -> Self {
        let target_to_camera = (camera - target).normalize(); //direction
        let camera_x_axis = world_y_axis.cross(target_to_camera).normalize(); //right
        let camera_y_axis = target_to_camera.cross(camera_x_axis); //up

        Self::new(
            Vec4::new(camera_x_axis.x, camera_y_axis.x, target_to_camera.x, 0.0),
            Vec4::new(camera_x_axis.y, camera_y_axis.y, target_to_camera.y, 0.0),
            Vec4::new(camera_x_axis.z, camera_y_axis.z, target_to_camera.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
        .translate(-camera)
    }

    ///Right Handed, Zero to One. For Vulkan, TODO: config compilation
    pub fn perspective(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let focal_length = 1.0 / (fov_rad * 0.5).tan();
        let depth = near - far;

        Self::new(
            Vec4::new(focal_length / aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -focal_length, 0.0, 0.0),
            Vec4::new(0.0, 0.0, far / depth, -1.0),
            Vec4::new(0.0, 0.0, 2.0*near * far / depth, 0.0),
        )
    }

    pub fn translate(self, factor: Vec3) -> Self {
        self * Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(factor.x, factor.y, factor.z, 1.0),
        )
    }

    pub fn scale(self, factor: Vec3) -> Self {
        self * Self::new_diagonal(Vec4::new(factor.x, factor.y, factor.z, 1.0))
    }

    pub fn rotate(self, radian: f32, axis: Vec3) -> Self {
        let half_radian = radian * 0.5;
        let sin = half_radian.sin();

        let w = half_radian.cos();
        let x = axis.x * sin;
        let y = axis.y * sin;
        let z = axis.z * sin;

        let len2 = w * w + x * x + y * y + z * z;
        let scale = if len2 == 0.0 { 0.0 } else { 2.0 / len2 };
        let (wx, wy, wz) = (scale * w * x, scale * w * y, scale * w * z);
        let (xx, xy, xz) = (scale * x * x, scale * x * y, scale * x * z);
        let (yy, yz, zz) = (scale * y * y, scale * y * z, scale * z * z);

        self * Self::new(
            Vec4::new(1.0 - yy - zz, xy + wz, xz - wy, 0.0),
            Vec4::new(xy - wz, 1.0 - xx - zz, yz + wx, 0.0),
            Vec4::new(xz + wy, yz - wx, 1.0 - xx - yy, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl AddAssign for Mat4 {
    fn add_assign(&mut self, other: Self) {
        self.a += other.a;
        self.b += other.b;
        self.c += other.c;
        self.d += other.d;
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, vec: Vec4) -> Self::Output {
        Self::Output {
            x: self.a.x * vec.x + self.b.x * vec.y + self.c.x * vec.z + self.d.x * vec.w,
            y: self.a.y * vec.x + self.b.y * vec.y + self.c.y * vec.z + self.d.y * vec.w,
            z: self.a.z * vec.x + self.b.z * vec.y + self.c.z * vec.z + self.d.z * vec.w,
            w: self.a.w * vec.x + self.b.w * vec.y + self.c.w * vec.z + self.d.w * vec.w,
        }
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, other: Mat4) {
        let a = *self * other.a;
        let b = *self * other.b;
        let c = *self * other.c;
        let d = *self * other.d;
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(mut self, other: Mat4) -> Self::Output {
        self *= other;
        self
    }
}
