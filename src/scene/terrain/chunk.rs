use std::sync::{Arc, Mutex, RwLock};


use cgmath::Vector3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};


use crate::render::{atlas::MaterialType, mesh::Mesh, pipelines::terrain::BlockVertex};


use super::{block::Block, LAND_LEVEL, noise::NoiseGenerator, biomes::BiomeParameters};


pub const CHUNK_Y_SIZE:usize = 100;
pub const CHUNK_AREA:usize =16;
pub const TOTAL_CHUNK_SIZE: usize = CHUNK_Y_SIZE * CHUNK_AREA * CHUNK_AREA;


pub type Blocks = Vec<Vec<Vec<Arc<RwLock<Block>>>>>;


#[derive(Default)]
pub struct Chunk {
    pub blocks: Blocks,
    pub offset: [i32; 3],
    pub updated: bool,
    pub mesh: Mesh<BlockVertex>,
    //pub neighbors: [Option<Arc<RwLock<Chunk>>>; 6]
}




impl Chunk {
    pub fn new(offset: [i32; 3]) -> Self {

        let mut blocks = vec![
            vec![
                vec![
                    Arc::new(
                        RwLock::new(
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

                    blocks[y][x][z] = Arc::new(RwLock::new(Block::new(material_type, position.into(), offset)));
                }
            }
        }
        Self { updated: true, blocks, offset, /* neighbors: Default::default(),*/ mesh: Default::default()}
    }
}



pub fn local_pos_to_world(offset: &[i32;3], local_pos: &Vector3<i32>) -> Vector3<f32> {
    Vector3::new(
        local_pos.x as f32 + (offset[0] as f32 * CHUNK_AREA as f32),
        local_pos.y as f32 + (offset[1] as f32 * CHUNK_AREA as f32),
        local_pos.z as f32 + (offset[2] as f32 * CHUNK_AREA as f32)
    )
}


pub fn pos_in_chunk_bounds(pos: Vector3<i32>) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.z >= 0 {
        if pos.x < CHUNK_AREA as i32 && pos.y < CHUNK_Y_SIZE as i32 && pos.z < CHUNK_AREA as i32 {
            return true;
        }
    }
    return false;
}





pub fn generate_chunk(blocks: &mut Blocks, offset: [i32; 3], seed: u32, biome: &BiomeParameters) {
    let noise_generator = NoiseGenerator::new(seed);

    (0..TOTAL_CHUNK_SIZE).into_par_iter().for_each(|i| {
        let z = i / (CHUNK_AREA * CHUNK_Y_SIZE);
        let y = (i - z * CHUNK_AREA * CHUNK_Y_SIZE) / CHUNK_AREA;
        let x = i % CHUNK_AREA;
        let world_pos = local_pos_to_world(&offset, &Vector3::new(x as i32, y as i32, z as i32));

        let height_variation = noise_generator.get_height(world_pos.x as f32, world_pos.z as f32, biome.frequency, biome.amplitude);
        let new_height = (biome.base_height + height_variation).round() as usize;

        //let new_height = y;

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

        blocks[y][x][z].write().unwrap().update(block_type, offset);
    });
}