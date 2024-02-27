use bytemuck::Pod;
use wgpu::{util::DeviceExt, BufferUsages};
pub struct Buffer<T: Copy + Pod> {
    pub buff: wgpu::Buffer,
    len: usize,
    phantom_data: std::marker::PhantomData<T>,
}

impl<T:Copy + Pod> Buffer<T> {
    pub fn new(device: &wgpu::Device, usage: wgpu::BufferUsages, data: &[T]) -> Self {
        let contents = bytemuck::cast_slice(data);

        Self {
            buff: device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: None,
                contents,
                usage: usage | BufferUsages::COPY_DST,
            }),
            len: data.len(),
            phantom_data: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize { self.len }
}