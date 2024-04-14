pub mod block;
pub mod chunk;
use crate::render::{atlas::Atlas, mesh::Mesh, model::Model, pipelines::terrain::{BlockVertex, TerrainPipeline}, renderer::{Draw, Renderer}};
use crate::render::pipelines::GlobalsLayouts;
use self::chunk::{generate_chunks, Chunk};

use wgpu::Error;

use super::camera::Camera;

pub const LAND_LEVEL: usize = 9;
pub const WORLD_SIZE: usize = 2; //chunks around

pub struct Terrain {
    pipeline: wgpu::RenderPipeline,
    atlas: Atlas,
    chunks: Vec<Chunk>,
    model: Model<BlockVertex>, // the world temporarily has only one block model, for debug purposes

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


        let chunks = generate_chunks(WORLD_SIZE as i32);

        for chunk in &chunks {
            //println!("chunk offset: {:?}", chunk.offset);
            
            mesh.push_chunk(&chunk);
            
        }

        let model = Model::new(&renderer.device, &mesh).unwrap();

        Self {

            pipeline: terrain_pipeline.pipeline,
            atlas,
            chunks,
            model,
        }
    }


}


impl Draw for Terrain {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, globals: &'a wgpu::BindGroup) -> Result<(), Error> {

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.atlas.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.model.vbuf().slice(..));
            render_pass.set_bind_group(1, &globals, &[]);
            render_pass.set_index_buffer(self.model.ibuf().slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.model.num_indices, 0, 0..1 as _);
        

        Ok(())
    }
}