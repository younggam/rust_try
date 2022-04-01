use super::*;

use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};

use wgpu::util::DeviceExt;

static LAST_MESH_ID: AtomicU32 = AtomicU32::new(0);

pub struct Mesh {
    id: u32,
    vertices: Vec<ColorVertex>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<ColorVertex>, indices: Vec<u32>) -> Self {
        //possible overflow
        let id = LAST_MESH_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            vertices,
            indices,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn vertices(&self) -> &[ColorVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn to_buffer(&self, device: &wgpu::Device) -> MeshBuffer {
        MeshBuffer::new(self.id, device, &self.vertices, &self.indices)
    }
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Mesh {}

impl Hash for Mesh {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct MeshBuffer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices_count: usize,
}

impl MeshBuffer {
    pub fn new(id: u32, device: &wgpu::Device, vertices: &[ColorVertex], indices: &[u32]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Vertex Buffer {id}")),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Index Buffer {id}")),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            indices_count: indices.len(),
        }
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn indices_count(&self) -> usize {
        self.indices_count
    }
}

//

pub struct Model {
    meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(meshes: Vec<Mesh>) -> Self {
        Self { meshes }
    }

    pub fn meshes(&self) -> &[Mesh] {
        &self.meshes
    }
}
