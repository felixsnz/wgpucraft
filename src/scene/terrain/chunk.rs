use crate::render::{buffer::Buffer, pipelines::terrain::BlockInstance};

use super::block::Block;

use cgmath::prelude::*;
use wgpu::Device;


const CHUNK_Z_SIZE:u16 = 2;
const CHUNK_AREA:u16 = 2;



pub struct Chunk {
    pub blocks: Vec<BlockInstance>,
    pub buff: Buffer<BlockInstance>
}


impl Chunk {
    pub fn new(device: &Device, offset: [i32; 3]) -> Self {
        const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(CHUNK_AREA as f32 * 0.5, 0.0, CHUNK_Z_SIZE as f32 * 0.5);
        
        let blocks = (0..CHUNK_Z_SIZE).flat_map(|z| {
            (0..CHUNK_AREA).map(move |x| {
                let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - INSTANCE_DISPLACEMENT;

                let rotation = if position.is_zero() {
                    // this is needed so an object at (0, 0, 0) won't get scaled to zero
                    // as Quaternions can affect scale if they're not created correctly
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0))
                };

                BlockInstance::new(position, rotation)
            })
        }).collect::<Vec<_>>();
        let buff = Buffer::new(&device, wgpu::BufferUsages::VERTEX, &blocks);

        Self {
            blocks,
            buff
        }
    }
}
