use crate::render::atlas::MaterialType;

use super::{block::Block, LAND_LEVEL};


pub const CHUNK_Y_SIZE:i32 = 10;
pub const CHUNK_AREA:u16 = 16;



pub struct Chunk {
    pub blocks: Vec<Block>,

}

impl Chunk {
    pub fn new(offset: [i32; 3]) -> Self {

        
        // Assuming CHUNK_Y_SIZE is a usize or similar that represents the height.
        let blocks = (0..CHUNK_AREA).flat_map(|z| {
            (0..CHUNK_AREA).flat_map(move |x| {
                (0..CHUNK_Y_SIZE).map(move |y| { // Iterate over y dimension
                    let position = cgmath::Vector3 { x: x as i32, y: y as i32, z: z as i32 };

                    let mut material_type = MaterialType::GRASS;
                    if y < LAND_LEVEL as i32 {
                        material_type = MaterialType::DIRT
                    }

                    Block::new(material_type, position.into(), offset)
                })
            })
        }).collect::<Vec<_>>();

        Self { blocks }
    }
}