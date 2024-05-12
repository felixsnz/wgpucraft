pub mod block;
pub mod chunk;
use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};

use crate::render::{atlas::{Atlas, MaterialType}, mesh::Mesh, model::DynamicModel, pipelines::terrain::{BlockVertex, TerrainPipeline}, renderer::{Draw, Renderer}};
use crate::render::pipelines::GlobalsLayouts;
use self::{block::Block, chunk::{generate_chunk, Chunk, CHUNK_AREA, CHUNK_Y_SIZE}};

use cgmath::{EuclideanSpace, Point3, Vector3};
use instant::Duration;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use wgpu::{Error, Queue};

pub const LAND_LEVEL: usize = 9;
pub const CHUNKS_VIEW_SIZE: usize = 15;
pub const CHUNKS_ARRAY_SIZE: usize = CHUNKS_VIEW_SIZE * CHUNKS_VIEW_SIZE;

pub struct Terrain {
    pipeline: wgpu::RenderPipeline,
    atlas: Atlas,
    pub chunks: Vec<Arc<Mutex<Chunk>>>,
    chunk_indices: Arc<Mutex<[Option<usize>; CHUNKS_ARRAY_SIZE]>>,
    free_chunk_indices: Arc<Mutex<VecDeque<usize>>>,
    center_offset: Vector3<i32>,
    chunks_origin: Vector3<i32>,
    chunk_models: Vec<DynamicModel<BlockVertex>>

}

impl Terrain {
    pub fn new(renderer: &Renderer) -> Self {
        let global_layouts = GlobalsLayouts::new(&renderer.device);
        let atlas = Atlas::new(&renderer.device, &renderer.queue, &global_layouts).unwrap();
        let mut chunk_models = vec![];
        let mut chunks: Vec<Arc<Mutex<Chunk>>> = Vec::new();
        let chunk_indices: [Option<usize>; CHUNKS_ARRAY_SIZE] = [None; CHUNKS_ARRAY_SIZE];
        let mut free_chunk_indices = VecDeque::new();

        for x in 0..CHUNKS_ARRAY_SIZE {
            //println!("initial x from new terrain: {:?}", x);
            chunks.push(Arc::new(Mutex::new(Chunk::new([0, 0, 0]))));
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
            chunk_indices: Arc::new(Mutex::new(chunk_indices)),
            free_chunk_indices: Arc::new(Mutex::new(free_chunk_indices))
        };

        terrain.load_empty_chunks(&renderer.queue);

        terrain
    }


    pub fn load_empty_chunks(&mut self, queue: &Queue) {

        (0..CHUNKS_ARRAY_SIZE).into_par_iter().for_each(|i| {
            let chunk_index = self.chunk_indices.lock().unwrap()[i].clone();
            if let None = chunk_index {
                let new_index = self.free_chunk_indices.lock().unwrap().pop_front().clone();
                if let Some(new_index) = new_index {
                    let chunk_offset = self.get_chunk_offset(i);
                    if !self.chunk_in_bounds(chunk_offset) {
                        panic!("Error: Cannot load chunk")
                    }

                    self.chunks[new_index].lock().unwrap().offset = chunk_offset.into();

                    generate_chunk( 
                        &mut self.chunks[new_index].lock().unwrap().blocks,
                        chunk_offset.into(),
                    );

                    self.chunk_indices.lock().unwrap()[i] = Some(new_index);
                } else {
                    panic!("Error: No free space for chunk")
                }
            }
        });

        (0..CHUNKS_ARRAY_SIZE).for_each(|i| {
            let mut chunk = self.chunks.get(i).unwrap().lock().unwrap();
            if chunk.updated {
                let mesh = self.update_mesh(&mut chunk);
                self.chunk_models[i].update(queue, &mesh, 0);
                chunk.updated = false; 
            }
        });

        println!("---------------------------------");
    }

    fn get_local_pos_in_neighbor(&self, world_pos: Vector3<i32>) -> Vector3<i32> {
        let x = ((world_pos.x % CHUNK_AREA as i32) + CHUNK_AREA as i32) % CHUNK_AREA as i32;
        let y = ((world_pos.y % CHUNK_Y_SIZE as i32) + CHUNK_Y_SIZE as i32) % CHUNK_Y_SIZE as i32;
        let z = ((world_pos.z % CHUNK_AREA as i32) + CHUNK_AREA as i32) % CHUNK_AREA as i32;
        Vector3::new(x, y, z)
    }


    fn get_chunk_by_world_position(&self, world_pos: Vector3<i32>) -> Option<Arc<Mutex<Chunk>>> {

        if world_pos.y < 0 || world_pos.y >= CHUNK_Y_SIZE as i32 {
            return None;
        }
        let index_x = (world_pos.x.div_euclid(CHUNK_AREA as i32) - self.chunks_origin.x);
        let index_z = (world_pos.z.div_euclid(CHUNK_AREA as i32) - self.chunks_origin.z);

        if index_x >= 0 && index_x < CHUNKS_VIEW_SIZE as i32 && index_z >= 0 && index_z < CHUNKS_VIEW_SIZE as i32 {
            let index = (index_z * CHUNKS_VIEW_SIZE as i32 + index_x) as usize;
            self.chunk_indices.lock().unwrap().get(index)
                .and_then(|&idx| self.chunks.get(idx.unwrap()))
                .map(Arc::clone)
        } else {
            None
        }
    }

    pub fn update_mesh(&self, chunk: &mut Chunk) -> Mesh<BlockVertex> {

        let mut verts =Vec::new();
        let mut indices = Vec::new();
        for y in 0.. CHUNK_Y_SIZE{
            for z in 0..CHUNK_AREA {
                for x in 0..CHUNK_AREA {
                    let block = chunk.blocks[y][x][z].lock().unwrap().clone();
                    let mut block_vertices = Vec::with_capacity(4 * 6);
                    let mut block_indices: Vec<u16> = Vec::with_capacity(6 * 6);
                    if block.material_type as i32 == MaterialType::AIR as i32 {
                        continue;
                    }
            
                    let mut quad_counter = 0;
            
                    for quad in block.quads.iter() {
                        let neighbour_pos: Vector3<i32> = block.get_vec_position() + quad.side.to_vec();
                        let visible = self.determine_visibility(&neighbour_pos, chunk);
            
                        if visible {
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
    
    /// Helper function to check visibility of a block
    fn determine_visibility(&self, neighbour_pos: &Vector3<i32>,chunk: &mut Chunk) -> bool {
        if Chunk::pos_in_chunk_bounds(*neighbour_pos) {
            let neighbour_block = chunk.blocks[neighbour_pos.y as usize][neighbour_pos.x as usize][neighbour_pos.z as usize].lock().unwrap();
            return neighbour_block.material_type as u16 == MaterialType::AIR as u16;
        } else {
            let world_pos = chunk.local_pos_to_world(*neighbour_pos);
            chunk.updated = true;
            //println!("world pos {:?}", world_pos);
            if let Some(neighbour_chunk) = self.get_chunk_by_world_position(world_pos) {
                let neighbour_chunk = neighbour_chunk.lock().unwrap();
                let local_pos_in_neighbour = self.get_local_pos_in_neighbor(world_pos);
                let neighbour_block = neighbour_chunk.blocks[local_pos_in_neighbour.y as usize][local_pos_in_neighbour.x as usize][local_pos_in_neighbour.z as usize].lock().unwrap();
                return neighbour_block.material_type as u16 == MaterialType::AIR as u16;
            } else {
                return true; // If the chunk doesn't exist, treat the block as visible
            }
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
        return Vector3::new(
            (world_pos.x / CHUNK_AREA as f32).floor() as i32,
            0,
            (world_pos.z / CHUNK_AREA as f32).floor() as i32,
        );
    }

    // chunk offset -> world array index
    fn get_chunk_world_index(&self, chunk_offset: Vector3<i32>) -> usize {
        let p = chunk_offset - self.chunks_origin;
        return p.z as usize * CHUNKS_VIEW_SIZE + p.x as usize;
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

        let chunk_indices_copy = self.chunk_indices.lock().unwrap().clone();

        self.chunk_indices = Arc::new(Mutex::new([None; CHUNKS_ARRAY_SIZE]));
        for i in 0..CHUNKS_ARRAY_SIZE {
            match chunk_indices_copy[i] {
                Some(chunk_index) => {
                    let chunk_offset = self.chunks.get(chunk_index).unwrap().lock().unwrap().offset.clone();
                    if self.chunk_in_bounds(chunk_offset.into()) {
                        let new_chunk_world_index = self.get_chunk_world_index(chunk_offset.into());
                        self.chunk_indices.lock().unwrap()[new_chunk_world_index] = Some(chunk_index);
                        self.chunks[chunk_index].lock().unwrap().updated = false; // Marcar como actualizado
                        
                    } else {
                        self.free_chunk_indices.lock().unwrap().push_back(chunk_index);
                        self.chunks[chunk_index].lock().unwrap().updated = true; // Marcar como actualizado
                    }
                }
                None => {}
            }
        }

        self.load_empty_chunks(queue);
    }

}


impl Draw for Terrain {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, globals: &'a wgpu::BindGroup) -> Result<(), Error> {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.atlas.bind_group, &[]);
        render_pass.set_bind_group(1, &globals, &[]);
        for chunk_model in &self.chunk_models {

            render_pass.set_vertex_buffer(0, chunk_model.vbuf().slice(..));
            render_pass.set_index_buffer(chunk_model.ibuf().slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..chunk_model.num_indices as u32, 0, 0..1 as _);
        }
        Ok(())
    }
}