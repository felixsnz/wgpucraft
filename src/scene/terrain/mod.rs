pub mod block;
pub mod chunk;
use std::{collections::VecDeque, sync::{Arc, Mutex}};

use crate::render::{atlas::Atlas, mesh::Mesh, model::{DynamicModel, Model}, pipelines::terrain::{BlockVertex, TerrainPipeline}, renderer::{Draw, Renderer}};
use crate::render::pipelines::GlobalsLayouts;
use self::chunk::{generate_chunk, generate_chunks, Chunk, CHUNK_AREA, CHUNK_Y_SIZE};

use bevy_ecs::world::Mut;
use cgmath::{EuclideanSpace, Vector3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use wgpu::Error;

use super::camera::Camera;

pub const LAND_LEVEL: usize = 9;
pub const CHUNKS_VIEW_SIZE: usize = 5; //chunks around
pub const CHUNKS_ARRAY_SIZE: usize = CHUNKS_VIEW_SIZE * CHUNKS_VIEW_SIZE;

pub struct Terrain {
    pipeline: wgpu::RenderPipeline,
    atlas: Atlas,
    chunks: Vec<Arc<Mutex<Chunk>>>,
    chunk_indices: Arc<Mutex<[Option<usize>; CHUNKS_ARRAY_SIZE]>>,
    free_chunk_indices: Arc<Mutex<VecDeque<usize>>>,
    center_offset: Vector3<i32>,
    chunks_origin: Vector3<i32>,
    chunk_models: Vec<DynamicModel<BlockVertex>>

}

impl Terrain {                        ///
    pub fn new(renderer: &Renderer) -> Self {
        

        let shader = renderer.device.create_shader_module(
            wgpu::include_wgsl!("../../../assets/shaders/shader.wgsl")
        );

        let global_layouts = GlobalsLayouts::new(&renderer.device);
        let atlas = Atlas::new(&renderer.device, &renderer.queue, &global_layouts).unwrap();
        let terrain_pipeline = TerrainPipeline::new(
            &renderer.device, 
            &global_layouts,
            shader,
            &renderer.config
        );
        let mut mesh = Mesh::new();

        let mut chunk_models = vec![];
        let mut chunks = generate_chunks(CHUNKS_VIEW_SIZE as i32);

        for chunk in &chunks {
            //println!("chunk offset: {:?}", chunk.offset);
            
            mesh.push_chunk(&chunk.lock().unwrap());
            
        }

        let chunk_indices: [Option<usize>; CHUNKS_ARRAY_SIZE] = [None; CHUNKS_ARRAY_SIZE];
        let mut free_chunk_indices = VecDeque::new();

        let center_offset = Vector3::new(0, 0, 0);
        let chunks_origin = center_offset - Vector3::new(CHUNKS_VIEW_SIZE as i32 / 2, 0, CHUNKS_VIEW_SIZE as i32 / 2);

        for x in 0..CHUNKS_VIEW_SIZE {

            chunks.push(Arc::new(Mutex::new(Chunk::new([0, 0, 0]))));

            if let Some(mesh) = Mesh::from(&chunks.last().unwrap().lock().unwrap()) {
                let mesh = mesh.clone();
                let chunk_model = DynamicModel::new(&renderer.device, (CHUNK_AREA ^ 2) * CHUNK_Y_SIZE * 24);
                chunk_model.update(&renderer.queue, &mesh, 0);
                chunk_models.push(chunk_model);
                free_chunk_indices.push_back(x);
            }
        }

        


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

    pub fn load_empty_chunks(&mut self, queue: &wgpu::Queue) {
        (0..CHUNKS_ARRAY_SIZE).into_par_iter().for_each(|i| {
            let chunk_index = self.chunk_indices.lock().unwrap()[i].clone();
            if let None = chunk_index {
                let new_index = self.free_chunk_indices.lock().unwrap().pop_front().clone();
                if let Some(new_index) = new_index {
                    let chunk_offset = self.get_chunk_offset(i);
                    if !self.chunk_in_bounds(chunk_offset) {
                        panic!("Error: Cannot load chunk")
                    }

                    self.chunks.get(new_index).unwrap().lock().unwrap().offset = chunk_offset.into();

                    generate_chunk( 
                        &mut self.chunks.get(new_index).unwrap().lock().unwrap().blocks,
                        chunk_offset.into(),
                        //self.world_seed,
                        //self.config.flat_world,
                    );

                    // let mesh = self.compute_mesh(&self.chunks.blocks_array[new_index].lock().unwrap());
                    // *self.chunks.mesh_array[new_index].lock().unwrap() = mesh;

                    self.chunk_indices.lock().unwrap()[i] = Some(new_index);
                } else {
                    panic!("Error: No free space for chunk")
                }
            }
        });

        (0..CHUNKS_ARRAY_SIZE).for_each(|i| {
            let mut mesh = Mesh::new();
            mesh.push_chunk(&self.chunks.get(i).unwrap().lock().unwrap());
            self.chunk_models[i].update(queue, &mesh, 0);
        });
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


    pub fn update(&mut self, renderer: &Renderer, camera: &Camera) {

        let new_chunk_offset = Self::world_pos_to_chunk_offset(camera.position.to_vec());
        let new_chunk_origin = new_chunk_offset - Vector3::new(CHUNKS_VIEW_SIZE as i32 / 2, 0, CHUNKS_VIEW_SIZE as i32 / 2);

        if new_chunk_origin == self.chunks_origin {
            return;
        }

        self.center_offset = new_chunk_offset;
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
                    } else {
                        self.free_chunk_indices.lock().unwrap().push_back(chunk_index);
                    }
                }
                None => {}
            }
        }

        self.load_empty_chunks(&renderer.queue);



    }
}


impl Draw for Terrain {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, globals: &'a wgpu::BindGroup) -> Result<(), Error> {

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.atlas.bind_group, &[]);
            render_pass.set_bind_group(1, &globals, &[]);

            for chunk_model in &self.chunk_models {
                render_pass.set_vertex_buffer(0, chunk_model.vbuf().slice(..));
                render_pass.set_index_buffer(chunk_model.ibuf().slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..chunk_model.num_indices, 0, 0..1 as _);
            }
            
        

        Ok(())
    }
}