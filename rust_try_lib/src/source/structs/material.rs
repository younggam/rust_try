#[derive(Clone, Debug, Copy)]
pub struct Material {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Material {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}
