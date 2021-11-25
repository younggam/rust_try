use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Vector {
    fn normalize(self) -> Self;
    fn dot(self, other: Self) -> Self;
}

#[derive(Clone, Debug, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const UNIT_X: Vec2 = Self::new(1.0, 0.0);
    pub const UNIT_Y: Vec2 = Self::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Vector for Vec2 {
    fn normalize(self) -> Self {
        self / (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(mut self, other: Self) -> Self {
        self.x *= other.x;
        self.y *= other.y;
        self
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(mut self, scalar: f32) -> Self::Output {
        self *= scalar;
        self
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(mut self, scalar: f32) -> Self::Output {
        self /= scalar;
        self
    }
}

// impl Mul for Vec2 {
//     type Output = f32;
//
//     ///dot product
//     fn mul(self, other: Self) -> Self::Output {
//         self.x * other.x + self.y * other.y
//     }
// }

//===========================

#[derive(Clone, Debug, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const UNIT_X: Vec3 = Self::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Vec3 = Self::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Vec3 = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            -self.x * other.z + self.z * other.x,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Vector for Vec3 {
    fn normalize(self) -> Self {
        self / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn dot(mut self, other: Self) -> Self {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(mut self, scalar: f32) -> Self::Output {
        self *= scalar;
        self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(mut self, scalar: f32) -> Self::Output {
        self /= scalar;
        self
    }
}

// impl Mul for Vec3 {
//     type Output = f32;
//
//     ///dot product
//     fn mul(self, other: Self) -> Self::Output {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }
// }

//===========================

#[derive(Clone, Debug, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const UNIT_X: Vec4 = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const UNIT_Y: Vec4 = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const UNIT_Z: Vec4 = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const UNIT_W: Vec4 = Self::new(0.0, 0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Vector for Vec4 {
    fn normalize(self) -> Self {
        self / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    fn dot(mut self, other: Self) -> Self {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
        self
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(mut self, scalar: f32) -> Self::Output {
        self *= scalar;
        self
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(mut self, scalar: f32) -> Self::Output {
        self /= scalar;
        self
    }
}
