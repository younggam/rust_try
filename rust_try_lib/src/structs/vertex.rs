#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub u: f32,
    pub v: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, u: f32, v: f32) -> Self {
        Self {
            x,
            y,
            z,
            r,
            g,
            b,
            u,
            v,
        }
    }
}
