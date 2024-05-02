use std::sync::{Arc, Mutex};

use cgmath::Vector3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::render::atlas::MaterialType;

use super::{block::Block, LAND_LEVEL};

pub const CHUNK_Y_SIZE:usize = 100;
pub const CHUNK_AREA:usize =16;
pub const TOTAL_CHUNK_SIZE: usize = CHUNK_Y_SIZE * CHUNK_AREA * CHUNK_AREA;
pub const CHUNKS_VIEW:usize = 2;

pub type Blocks = Vec<Vec<Vec<Arc<Mutex<Block>>>>>;

pub struct Chunk {
    pub blocks: Blocks,
    pub offset: [i32; 3]

}

impl Chunk {
    pub fn new(offset: [i32; 3]) -> Self {

        let mut blocks = vec![
            vec![
                vec![
                    Arc::new(
                        Mutex::new(
                            Block::new(
                                MaterialType::DEBUG,
                                [0, 0, 0],
                                offset
                            )
                        )
                    )
                    ; CHUNK_AREA
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

                    blocks[y][x][z] = Arc::new(Mutex::new(Block::new(material_type, position.into(), offset)));
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


pub fn generate_chunks(world_size: i32) -> Vec<Arc<Mutex<Chunk>>> {
    let mut chunks = Vec::new();
    for x in -world_size..=world_size {
        for z in -world_size..=world_size{
            let offset = [x, 0, z];
            //println!("offset: {:?}", offset);
            let chunk = Arc::new(Mutex::new(Chunk::new(offset)));
            chunks.push(chunk);
        }
    }
    //chunks.reverse();
    chunks
}


// pub fn generate_chunk_w_seed(blocks: &mut Blocks, offset: [i32; 3], seed: u32, flat_world: bool) {
//     if flat_world {
//         generate_flat_world(blocks, offset);
//         return;
//     }

//     let noise_map = get_noise_map(offset, seed);
//     (0..TOTAL_CHUNK_SIZE).into_par_iter().for_each(|i| {
//         let z = i / (CHUNK_X_SIZE * CHUNK_Y_SIZE);
//         let y = (i - z * CHUNK_X_SIZE * CHUNK_Y_SIZE) / CHUNK_X_SIZE;
//         let x = i - CHUNK_X_SIZE * (y + CHUNK_Y_SIZE * z);

//         let noise_height = noise_map.get_value(x, z);
//         let new_height = normalize_noise(noise_height);

//         let block_type = if y > new_height {
//             if y <= SEA_LEVEL {
//                 BlockType::WATER
//             } else {
//                 BlockType::AIR
//             }
//         } else if y == new_height {
//             BlockType::GRASS
//         } else if y == 0 {
//             BlockType::ROCK
//         } else {
//             BlockType::DIRT
//         };

//         blocks[y][x][z].lock().unwrap().update(block_type, offset);
//     });
// }

pub fn generate_chunk(blocks: &mut Blocks, offset: [i32; 3]) {
    (0..TOTAL_CHUNK_SIZE).into_par_iter().for_each(|i| {
        let z = i / (CHUNK_AREA * CHUNK_Y_SIZE);
        let y = (i - z * CHUNK_AREA * CHUNK_Y_SIZE) / CHUNK_AREA;
        let x = i % CHUNK_AREA;

        // Example mathematical function: a simple sine wave pattern
        let new_height = ((x as f32 + offset[0] as f32).sin() + (z as f32 + offset[2] as f32).sin() * 10.0).round() as usize;

        let block_type = if y > new_height {
            if y <= LAND_LEVEL {
                MaterialType::WATER
            } else {
                MaterialType::AIR
            }
        } else if y == new_height {
            MaterialType::GRASS
        } else if y == 0 {
            MaterialType::ROCK
        } else {
            MaterialType::DIRT
        };

        blocks[y][x][z].lock().unwrap().update(block_type, offset);
    });
}