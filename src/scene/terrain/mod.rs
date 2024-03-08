pub mod block;
use crate::render::{atlas::{Atlas, MaterialType}, mesh::Mesh, model::Model, pipelines::terrain::TerrainPipeline, renderer::{Draw, Renderer}};
use crate::render::pipelines::GlobalsLayouts;
use super::terrain::block::Block;


use wgpu::Error;


pub struct Terrain {
    pipeline: wgpu::RenderPipeline,
    atlas: Atlas,
    model: Model// the world temporarily has only one block model, for debug purposes

}

impl Terrain {                        ///
    pub fn new(renderer: &Renderer) -> Self {
        let atlas = Atlas::new(&renderer.device, &renderer.queue).unwrap();

        let shader = renderer.device.create_shader_module(wgpu::include_wgsl!("../../../assets/shaders/shader.wgsl"));

        let terrain_pipeline = TerrainPipeline::new(
            &renderer.device, 
            &GlobalsLayouts::new(&renderer.device),
            shader,
            &renderer.config
        );
        
        let block = Block::new(MaterialType::GRASS, [0,0,0], [0,0,0]);


        let mut mesh = Mesh::new();
        mesh.push_block(block);

        let model = Model::new(&renderer.device, &mesh).unwrap();

        Self {

            pipeline: terrain_pipeline.pipeline,
            atlas,
            model
        }
    }
}


impl Draw for Terrain {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, globals: &'a wgpu::BindGroup) -> Result<(), Error> {

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_bind_group(0, &self.atlas.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.model.vbuf().slice(..));
            render_pass.set_bind_group(1, &globals, &[]);
            //render_pass.set_vertex_buffer(1, self.instance_buffer.buff.slice(..));
            render_pass.set_index_buffer(self.model.ibuf().slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.model.num_indices, 0, 0..1 as _);
        

        Ok(())
    }
}