use crate::render::{buffer::Buffer, mesh::Mesh};

use super::{buffer::DynamicBuffer, Vertex};
/// Represents a mesh that has been sent to the GPU.
pub struct Model<V: Vertex>{
    vbuf: Buffer<V>,
    ibuf: Buffer<u16>,
    pub num_indices: u16,
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
            num_indices: mesh.indices().len() as u16,
        })
    }

    
    pub fn vbuf(&self) -> &wgpu::Buffer { &self.vbuf.buff }
    pub fn ibuf(&self) -> &wgpu::Buffer { &self.ibuf.buff }
    pub fn len(&self) -> u16 { self.vbuf.len() as u16}
}


/// Represents a mesh that has been sent to the GPU.
pub struct DynamicModel<V: Vertex> {
    vbuf: DynamicBuffer<V>,
    ibuf: DynamicBuffer<u16>,
    pub num_indices: u16,
}

impl<V: Vertex> DynamicModel<V> {
    pub fn new(device: &wgpu::Device, size: usize) -> Self {
        Self {
            vbuf: DynamicBuffer::new(device, size, wgpu::BufferUsages::VERTEX),
            ibuf: DynamicBuffer::new(device,  size, wgpu::BufferUsages::INDEX),
            num_indices: 0,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, mesh: &Mesh<V>, offset: usize) {
        self.vbuf.update(queue, mesh.vertices(), offset);
        self.ibuf.update(queue, mesh.indices(), offset);
        self.num_indices = mesh.indices().len() as u16;
    }

    pub fn vbuf(&self) -> &wgpu::Buffer { &self.vbuf.buff }
    pub fn ibuf(&self) -> &wgpu::Buffer { &self.ibuf.buff }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize { self.vbuf.len() }
}

