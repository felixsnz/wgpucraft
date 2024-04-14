use cgmath::Vector3;

use crate::render::atlas::MaterialType;

use super::{block::Block, LAND_LEVEL};

pub const CHUNK_Y_SIZE:usize = 100;
pub const CHUNK_AREA:usize =16;
pub const TOTAL_CHUNK_SIZE: usize = CHUNK_Y_SIZE * CHUNK_AREA * CHUNK_AREA;
pub const CHUNKS_VIEW:usize = 2;



pub struct Chunk {
    pub blocks: Vec<Vec<Vec<Block>>>,
    pub offset: [i32; 3]

}

impl Chunk {
    pub fn new(offset: [i32; 3]) -> Self {

        let mut blocks = vec![
            vec![
                vec![
                    Block::new(
                        MaterialType::DEBUG,
                        [0, 0, 0],
                        offset
                    ); CHUNK_AREA
                ]; CHUNK_AREA
            ]; CHUNK_Y_SIZE
        ];
        
        // Assuming CHUNK_Y_SIZE is a usize or similar that represents the height.
        for y in 0..CHUNK_Y_SIZE{
            for z in 0..CHUNK_AREA {
                for x in 0..CHUNK_AREA {
                    let position = cgmath::Vector3 { x: x as i32, y: y as i32, z: z as i32 };
                    let material_type =
                    if y < LAND_LEVEL {
                        MaterialType::DEBUG
                    }
                    else if y == LAND_LEVEL{
                        MaterialType::DEBUG
                    }
                    else {
                        MaterialType::AIR
                    };

                    blocks[y][x][z] = Block::new(material_type, position.into(), offset);
                }
            }
        }
        Self { blocks, offset}
    }


    pub fn pos_in_chunk_bounds(pos: Vector3<i32>) -> bool {
        if pos.x >= 0 && pos.y >= 0 && pos.z >= 0 {
            if pos.x < CHUNK_AREA as i32 && pos.y < CHUNK_Y_SIZE as i32 && pos.z < CHUNK_AREA as i32 {
                return true;
            }
        }
        return false;
    }

    
}


pub fn generate_chunks(world_size: i32) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    for x in -world_size..=world_size {
        for z in -world_size..=world_size{
            let offset = [x, 0, z];
            println!("offset: {:?}", offset);
            let chunk = Chunk::new(offset);
            chunks.push(chunk);
        }
    }
    //chunks.reverse();
    chunks
}