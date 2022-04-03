pub trait Uniform {}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewProjectionUniform {
    pub matrix: [[f32; 4]; 4],
    pub view_position: [f32; 3],
    pub _p: u32,
}

impl Uniform for ViewProjectionUniform {}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightSourceUniform {
    pub position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing and bytemuck refuse empty padding byte, we need to use a padding field here
    pub _p0: u32,
    pub color: [f32; 3],
    pub _p1: u32,
}

impl Uniform for LightSourceUniform {}
