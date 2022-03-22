use super::*;

use std::sync::atomic::{AtomicU32, Ordering};

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
}

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
