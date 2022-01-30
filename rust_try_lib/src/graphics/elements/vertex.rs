use crate::math::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Vec3,
    pub tex_coord: Vec2,
}

impl Vertex {
    pub const fn new(pos: Vec3, color: Vec3, tex_coord: Vec2) -> Self {
        Self {
            pos,
            color,
            tex_coord,
        }
    }
}
