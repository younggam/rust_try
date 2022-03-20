use std::mem;

use cgmath::*;

pub trait Vertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[derive(Clone, Debug, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ColorVertex {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

impl ColorVertex {
    pub const fn new(position: [f32; 4], color: [f32; 4]) -> Self {
        Self { position, color }
    }
}

impl Vertex for ColorVertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    transform: [[f32; 4]; 4],
}

impl Instance {
    pub fn new(position: Vector3<f32>, rotation: Quaternion<f32>) -> Self {
        Self {
            transform: (Matrix4::from_translation(position) * Matrix4::from(rotation)).into(),
        }
    }

    pub fn from_translation(value: Vector3<f32>) -> Self {
        Self {
            transform: Matrix4::from_translation(value).into(),
        }
    }

    pub fn from_transform(transform: Matrix4<f32>) -> Self {
        Self {
            transform: transform.into(),
        }
    }
}

impl Vertex for Instance {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
            ],
        }
    }
}
