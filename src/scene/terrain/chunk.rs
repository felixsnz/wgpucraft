use std::sync::{Arc, Mutex, RwLock};


use cgmath::Vector3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};


use crate::render::{atlas::MaterialType, mesh::Mesh, pipelines::terrain::BlockVertex};


use super::{block::Block, LAND_LEVEL};


pub const CHUNK_Y_SIZE:usize = 100;
pub const CHUNK_AREA:usize =16;
pub const TOTAL_CHUNK_SIZE: usize = CHUNK_Y_SIZE * CHUNK_AREA * CHUNK_AREA;


pub type Blocks = Vec<Vec<Vec<Arc<Mutex<Block>>>>>;




fn init_blocks(offset: [i32; 3]) -> Blocks {


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


    blocks


}


// #[derive(Default)]
// pub struct Chunk {
//     pub blocks: Blocks,
//     pub offset: [i32; 3],
//     pub updated: bool
// }


pub fn local_pos_to_world(offset:[i32;3], local_pos: Vector3<i32>) -> Vector3<f32> {
    Vector3::new(
        local_pos.x as f32 + (offset[0] as f32 * CHUNK_AREA as f32),
        local_pos.y as f32 + (offset[1] as f32 * CHUNK_AREA as f32),
        local_pos.z as f32 + (offset[2] as f32 * CHUNK_AREA as f32)
    )
}


#[derive(Default)]
pub struct ChunkArray {
    pub mesh_array: Vec<Arc<RwLock<Mesh<BlockVertex>>>>,
    pub offset_array: Vec<Arc<RwLock<[i32; 3]>>>,
    pub blocks_array: Vec<Arc<RwLock<Blocks>>>,
}






impl ChunkArray {


    pub fn new_chunk(&mut self, offset: [i32; 3]) -> &Self {
        let blocks = init_blocks(offset);
        self.mesh_array.push(Arc::new(RwLock::new(Mesh::new())));
        self.blocks_array.push(Arc::new(RwLock::new(blocks)));
        self.offset_array.push(Arc::new(RwLock::new(offset)));
        return self;
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


pub fn generate_chunk(blocks: &mut Blocks, offset: [i32; 3]) {
    (0..TOTAL_CHUNK_SIZE).into_par_iter().for_each(|i| {
        let z = i / (CHUNK_AREA * CHUNK_Y_SIZE);
        let y = (i - z * CHUNK_AREA * CHUNK_Y_SIZE) / CHUNK_AREA;
        let x = i % CHUNK_AREA;


        // Función matemática simple para generar un terreno 3D con colinas suaves
        let base_height = 10.0;
        let frequency = 0.1;
        let amplitude = 5.0;
        
        let height_variation = (x as f32 * frequency).sin() + (z as f32 * frequency).sin();
        let new_height = (base_height + height_variation * amplitude).round() as usize;

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

