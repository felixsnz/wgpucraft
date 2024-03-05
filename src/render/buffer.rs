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


pub struct DynamicBuffer<T: Copy + Pod>(Buffer<T>);

impl<T: Copy + Pod> DynamicBuffer<T> {
    pub fn new(device: &wgpu::Device, len: usize, usage: wgpu::BufferUsages) -> Self {
        let buffer = Buffer {
            buff: device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                mapped_at_creation: false,
                size: len as u64 * std::mem::size_of::<T>() as u64,
                usage: usage | wgpu::BufferUsages::COPY_DST,
            }),
            len,
            phantom_data: std::marker::PhantomData,
        };
        Self(buffer)
    }

    pub fn update(&self, queue: &wgpu::Queue, vals: &[T], offset: usize) {
        if !vals.is_empty() {
            queue.write_buffer(
                &self.buff,
                offset as u64 * std::mem::size_of::<T>() as u64,
                bytemuck::cast_slice(vals),
            )
        }
    }
}

impl<T: Copy + Pod> std::ops::Deref for DynamicBuffer<T> {
    type Target = Buffer<T>;

    fn deref(&self) -> &Self::Target { &self.0 }
}