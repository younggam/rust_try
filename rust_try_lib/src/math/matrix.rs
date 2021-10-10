use super::vector::*;

pub struct Mat2 {
    pub a: Vec2,
    pub b: Vec2,
}

impl Mat2 {
    pub const IDENTITY: Mat2 = Self::new(Vec2::UNIT_X, Vec2::UNIT_Y);

    pub const fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    pub fn new_diagonal(value: Vec2) -> Self {
        Self {
            a: Vec2::new(value.x, 0.0),
            b: Vec2::new(0.0, value.y),
        }
    }
}

pub struct Mat3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Self::new(Vec3::UNIT_X, Vec3::UNIT_Y, Vec3::UNIT_Z);

    pub const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }

    pub fn new_diagonal(value: Vec3) -> Self {
        Self {
            a: Vec3::new(value.x, 0.0, 0.0),
            b: Vec3::new(0.0, value.y, 0.0),
            c: Vec3::new(0.0, 0.0, value.z),
        }
    }
}

pub struct Mat4 {
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
    pub d: Vec4,
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Self::new(Vec4::UNIT_X, Vec4::UNIT_Y, Vec4::UNIT_Z, Vec4::UNIT_W);

    pub const fn new(a: Vec4, b: Vec4, c: Vec4, d: Vec4) -> Self {
        Self { a, b, c, d }
    }

    pub fn new_diagonal(value: Vec4) -> Self {
        Self {
            a: Vec4::new(value.x, 0.0, 0.0, 0.0),
            b: Vec4::new(0.0, value.y, 0.0, 0.0),
            c: Vec4::new(0.0, 0.0, value.z, 0.0),
            d: Vec4::new(0.0, 0.0, 0.0, value.w),
        }
    }
}
