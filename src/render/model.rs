
use crate::render::{buffer::Buffer, mesh::Mesh};

use super::Vertex;
/// Represents a mesh that has been sent to the GPU.
pub struct Model<V: Vertex>{
    vbuf: Buffer<V>,
    ibuf: Buffer<u16>,
    pub num_indices: u32,
}

impl<V: Vertex> Model<V>{
    pub fn new(device: &wgpu::Device, mesh: &Mesh<V>) -> Option<Self> {
        if mesh.vertices().is_empty() || mesh.indices().is_empty() {
            return None;
        }

        let vbuf = Buffer::new(device, wgpu::BufferUsages::VERTEX, mesh.vertices());
        let ibuf = Buffer::new(device, wgpu::BufferUsages::INDEX, mesh.indices());

        Some(Self {
            vbuf,
            ibuf,
            num_indices: mesh.indices().len() as u32,
        })
    }
    pub fn vbuf(&self) -> &wgpu::Buffer { &self.vbuf.buff }
    pub fn ibuf(&self) -> &wgpu::Buffer { &self.ibuf.buff }
    pub fn len(&self) -> u32 { self.vbuf.len() as u32}
}

