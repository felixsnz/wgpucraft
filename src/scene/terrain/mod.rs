pub mod block;
pub mod chunk;
pub mod noise;
pub mod biomes;
use std::{collections::VecDeque, sync::{Arc, Barrier, RwLock}};

use crate::render::{atlas::{Atlas, MaterialType}, mesh::Mesh, model::DynamicModel, pipelines::terrain::{BlockVertex, TerrainPipeline}, renderer::{Draw, Renderer}};
use crate::render::pipelines::GlobalsLayouts;
use self::chunk::{generate_chunk, Blocks, CHUNK_AREA, CHUNK_Y_SIZE, Chunk, pos_in_chunk_bounds};


use biomes::{MOUNTAIN_PARAMS, PRAIRIE_PARAMS};
use block::Block;
use cgmath::{EuclideanSpace, Point3, Vector3};
use chunk::local_pos_to_world;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use wgpu::Queue;


pub const LAND_LEVEL: usize = 9;
pub const CHUNKS_VIEW_SIZE: usize = 2;
pub const CHUNKS_ARRAY_SIZE: usize = CHUNKS_VIEW_SIZE * CHUNKS_VIEW_SIZE;




pub struct Terrain {
    pipeline: wgpu::RenderPipeline,
    atlas: Atlas,
    pub chunks: Vec<Arc<RwLock<Chunk>>>,
    chunk_indices: Arc<RwLock<[Option<usize>; CHUNKS_ARRAY_SIZE]>>,
    free_chunk_indices: Arc<RwLock<VecDeque<usize>>>,
    updated_indices: Arc<RwLock<[bool; CHUNKS_ARRAY_SIZE]>>,
    center_offset: Vector3<i32>,
    chunks_origin: Vector3<i32>,
    chunk_models: Vec<DynamicModel<BlockVertex>>


}


impl Terrain {
    pub fn new(renderer: &Renderer) -> Self {
        let global_layouts = GlobalsLayouts::new(&renderer.device);
        let atlas = Atlas::new(&renderer.device, &renderer.queue, &global_layouts).unwrap();
        let mut chunk_models = vec![];
        let mut chunks:Vec<Arc<RwLock<Chunk>>> = Vec::default();
        let chunk_indices: [Option<usize>; CHUNKS_ARRAY_SIZE] = [None; CHUNKS_ARRAY_SIZE];
        let updated_indices = Arc::new(RwLock::new([false; CHUNKS_ARRAY_SIZE]));
        let mut free_chunk_indices = VecDeque::new();


        for x in 0..CHUNKS_ARRAY_SIZE {
            //println!("initial x from new terrain: {:?}", x);
            chunks.push(Arc::new(RwLock::new(Chunk::new([0, 0, 0]))));
            let mesh = Mesh::new(); //empty mesh
            let mut chunk_model = DynamicModel::new(&renderer.device, (CHUNK_AREA ^ 2) * CHUNK_Y_SIZE * 24);
            chunk_model.update(&renderer.queue, &mesh, 0);
            chunk_models.push(chunk_model);
            free_chunk_indices.push_back(x);


        }

        


        let shader = renderer.device.create_shader_module(
            wgpu::include_wgsl!("../../../assets/shaders/shader.wgsl")
        );


        let terrain_pipeline = TerrainPipeline::new(
            &renderer.device,
            &global_layouts,
            shader,
            &renderer.config
        );


        let center_offset = Vector3::new(0, 0, 0);
        let chunks_origin = center_offset - Vector3::new(CHUNKS_VIEW_SIZE as i32 / 2, 0, CHUNKS_VIEW_SIZE as i32 / 2);


        let mut terrain = Self {
            pipeline: terrain_pipeline.pipeline,
            atlas,
            chunks,
            chunk_models,
            center_offset,
            chunks_origin,
            updated_indices,
            chunk_indices: Arc::new(RwLock::new(chunk_indices)),
            free_chunk_indices: Arc::new(RwLock::new(free_chunk_indices))
        };

        // Establecer referencias a los vecinos
        // for i in 0..CHUNKS_ARRAY_SIZE {
        //     let chunk_offset = terrain.get_chunk_offset(i);
        //     let neighbors = [
        //         terrain.get_chunk_at_offset(chunk_offset + Vector3::new(1, 0, 0)),  // vecino en +X
        //         terrain.get_chunk_at_offset(chunk_offset + Vector3::new(-1, 0, 0)), // vecino en -X
        //         terrain.get_chunk_at_offset(chunk_offset + Vector3::new(0, 1, 0)),  // vecino en +Y
        //         terrain.get_chunk_at_offset(chunk_offset + Vector3::new(0, -1, 0)), // vecino en -Y
        //         terrain.get_chunk_at_offset(chunk_offset + Vector3::new(0, 0, 1)),  // vecino en +Z
        //         terrain.get_chunk_at_offset(chunk_offset + Vector3::new(0, 0, -1))  // vecino en -Z
        //     ];
        //     terrain.chunks[i].write().unwrap().neighbors = neighbors;
        // }


        println!("about to load first chunks");
        terrain.load_empty_chunks(&renderer.queue);


        terrain
    }

    fn get_chunk_at_offset(&self, offset: Vector3<i32>) -> Option<Arc<RwLock<Chunk>>> {
        if self.chunk_in_bounds(offset) {
            let index = self.get_chunk_world_index(offset);
            self.chunk_indices.read().unwrap()[index].map(|i| self.chunks[i].clone())
        } else {
            None
        }
    }




    pub fn load_empty_chunks(&mut self, queue: &Queue) {

        let chunks_to_update: usize = (0..CHUNKS_ARRAY_SIZE)
            .filter(|&i| self.chunk_indices.read().unwrap()[i].is_none())
            .count();

        let barrier = Arc::new(Barrier::new(chunks_to_update));

        println!("chunks array size: {:?}", CHUNKS_ARRAY_SIZE);

        (0..CHUNKS_ARRAY_SIZE).into_par_iter().for_each(|i| { 
            let c = Arc::clone(&barrier); // Clonar la referencia a la barrera para cada hilo

            


            let chunk_index = self.chunk_indices.read().unwrap()[i].clone();

            

            
            if let None = chunk_index {

                
                let new_index = self.free_chunk_indices.write().unwrap().pop_front();
                
                if let Some(new_index) = new_index {
                    let chunk_offset = self.get_chunk_offset(i);
                    if !self.chunk_in_bounds(chunk_offset) {
                        panic!("Error: Cannot load chunk")
                    }
                    


                    self.chunks[new_index].write().unwrap().offset = chunk_offset.into();
                    generate_chunk(
                        &mut self.chunks[new_index].write().unwrap().blocks,
                        chunk_offset.into(),
                        892984781,
                        &PRAIRIE_PARAMS
                    );
                    println!("barrera alcaza");
                    
                    self.chunk_indices.write().unwrap()[i] = Some(new_index);

                    c.wait();
                    let mesh = self.update_mesh(&self.chunks[new_index].read().unwrap().blocks, &self.chunks[new_index].read().unwrap().offset);
                    self.chunks[new_index].write().unwrap().mesh = mesh;

                    // Mark this index as updated
                    self.updated_indices.write().unwrap()[new_index] = true;


                } else {
                    panic!("Error: No free space for chunk")
                }
            }
        });
    

        (0..CHUNKS_ARRAY_SIZE).for_each(|i| {

            if self.updated_indices.read().unwrap()[i] {
                self.chunk_models[i].update(queue, &self.chunks[i].read().unwrap().mesh, 0);
                self.updated_indices.write().unwrap()[i] = false;
                //println!("selected to update")
            }

        });


        println!("---------------------------------");
    }



    pub fn update_mesh(&self, blocks: &Blocks, offset: &[i32; 3]) -> Mesh<BlockVertex> {


        let mut verts =Vec::new();
        let mut indices = Vec::new();
        for y in 0.. CHUNK_Y_SIZE{
            for z in 0..CHUNK_AREA {
                for x in 0..CHUNK_AREA {
                    let block = blocks[y][x][z].read().unwrap().clone();
                    let mut block_vertices = Vec::with_capacity(4 * 6);
                    let mut block_indices: Vec<u16> = Vec::with_capacity(6 * 6);
                    if block.material_type as i32 == MaterialType::AIR as i32 {
                        continue;
                    }


                    let mut quad_counter = 0;


                    for quad in block.quads.iter() {
                        let neighbor_pos: Vector3<i32> = block.get_vec_position() + quad.side.to_vec();
                        let neighbor_exists = self.neighbor_exists(&neighbor_pos, blocks, offset);


                        if !neighbor_exists { //if neigbor does not exists, the current face is visible
                            block_vertices.extend_from_slice(&quad.vertices);
                            block_indices.extend_from_slice(&quad.get_indices(quad_counter));
                            quad_counter += 1;
                        }
                    }
                    block_indices = block_indices.iter().map(|i| i + verts.len() as u16).collect();
                    verts.extend(block_vertices);
                    indices.extend(block_indices);
                }
            }
        }


        Mesh { verts, indices }
    }


    fn neighbor_exists(&self, neighbor_pos: &Vector3<i32>, blocks: &Blocks, chunk_offset: &[i32; 3]) -> bool {
        if pos_in_chunk_bounds(*neighbor_pos) {
            let neighbor_block = blocks[neighbor_pos.y as usize][neighbor_pos.x as usize][neighbor_pos.z as usize].read().unwrap();
            return !(neighbor_block.material_type as u16 == MaterialType::AIR as u16);
        } else {

            if neighbor_pos.y < 0 || neighbor_pos.y >= CHUNK_Y_SIZE as i32 {
                return false;
            }
            let world_pos = local_pos_to_world(chunk_offset, neighbor_pos);
            let neighbor_chunk_offset = Self::world_pos_to_chunk_offset(world_pos);
            
            if let Some(neighbor_chunk) = self.get_chunk_at_offset(neighbor_chunk_offset) {
                let neighbor_chunk = neighbor_chunk.read().unwrap();
                let local_pos = Vector3::new(
                    world_pos.x as i32 - (neighbor_chunk_offset[0] * CHUNK_AREA as i32),
                    world_pos.y as i32- (neighbor_chunk_offset[1] * CHUNK_Y_SIZE as i32),
                    world_pos.z as i32- (neighbor_chunk_offset[2] * CHUNK_AREA as i32),
                );

                if pos_in_chunk_bounds(local_pos) {
                    let neighbor_block = neighbor_chunk.blocks[local_pos.y as usize][local_pos.x as usize][local_pos.z as usize].read().unwrap();
                    return !(neighbor_block.material_type as u16 == MaterialType::AIR as u16);
                }
                else {
                    //println!("{:?}",neighbor_chunk_offset);
                    println!("debug {:?}", local_pos);
                    println!("error esto no deberia ??");
                    return false;
                }
                
            }
            else {
                println!("no encontro chunk")
            }
            return false;
        }
    }


    // world array index -> chunk offset
    fn get_chunk_offset(&self, i: usize) -> Vector3<i32> {
        return self.chunks_origin + Vector3::new(i as i32 % CHUNKS_VIEW_SIZE as i32, 0, i as i32 / CHUNKS_VIEW_SIZE as i32);
    }


    fn chunk_in_bounds(&self, chunk_offset: Vector3<i32>) -> bool {
        let p = chunk_offset - self.chunks_origin;
        if p.x >= 0 && p.z >= 0 && p.x < CHUNKS_VIEW_SIZE as i32 && p.z < CHUNKS_VIEW_SIZE as i32 {
            return true;
        }
        return false;
    }


    fn world_pos_to_chunk_offset(world_pos: Vector3<f32>) -> Vector3<i32> {
        Vector3::new(
            (world_pos.x / CHUNK_AREA as f32).floor() as i32,
            0,
            (world_pos.z / CHUNK_AREA as f32).floor() as i32,
        )
    }
    
    fn get_chunk_world_index(&self, chunk_offset: Vector3<i32>) -> usize {
        let p = chunk_offset - self.chunks_origin;
        (p.z as usize * CHUNKS_VIEW_SIZE) + p.x as usize
    }


    //called every frame
    pub fn update(&mut self, queue: &Queue, player_position: &Point3<f32>) {
        let new_center_offset = Self::world_pos_to_chunk_offset(player_position.to_vec());
        let new_chunk_origin = new_center_offset - Vector3::new(CHUNKS_VIEW_SIZE as i32 / 2, 0, CHUNKS_VIEW_SIZE as i32 / 2);

        if new_chunk_origin == self.chunks_origin {
            return;
        }

        self.center_offset = new_center_offset;
        self.chunks_origin = new_chunk_origin;
        println!("chunks origin updated {:?}", self.chunks_origin);

        let chunk_indices_copy = self.chunk_indices.read().unwrap().clone();
        self.chunk_indices = Arc::new(RwLock::new([None; CHUNKS_ARRAY_SIZE]));

        for i in 0..CHUNKS_ARRAY_SIZE {
            match chunk_indices_copy[i] {
                Some(chunk_index) => {
                    //let chunk_offset = self.chunks.offset_array.get(chunk_index).unwrap().read().unwrap().clone();
                    let chunk_offset = self.chunks.get(chunk_index).unwrap().read().unwrap().offset.clone();
                    if self.chunk_in_bounds(chunk_offset.into()) {
                        let new_chunk_world_index = self.get_chunk_world_index(chunk_offset.into());
                        self.chunk_indices.write().unwrap()[new_chunk_world_index] = Some(chunk_index);
                    } else {
                        self.free_chunk_indices.write().unwrap().push_back(chunk_index);
                    }
                }
                None => {}
            }
        }

        self.load_empty_chunks(queue);
    }


}

impl Draw for Terrain {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, globals: &'a wgpu::BindGroup) -> Result<(), wgpu::Error> {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.atlas.bind_group, &[]);
        render_pass.set_bind_group(1, &globals, &[]);
        
        for chunk_model in &self.chunk_models {
                let vertex_buffer = chunk_model.vbuf().slice(..);
                let index_buffer = chunk_model.ibuf().slice(..);
                let num_indices = chunk_model.num_indices;

                render_pass.set_vertex_buffer(0, vertex_buffer);
                render_pass.set_index_buffer(index_buffer, wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..num_indices as u32, 0, 0..1 as _);
        }
        
        Ok(())
    }
}