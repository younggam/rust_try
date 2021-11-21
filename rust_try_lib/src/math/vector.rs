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
}

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
