use crate::{render::{atlas::{Atlas, MaterialType}, mesh::Mesh, model::Model, pipelines::constructor as pipeline_constructor, renderer::{Draw, Renderer}, Vertex }, scene::camera::CameraLayout};

use self::block::{Block, QuadVertex};

pub mod block;
pub mod chunk;

use wgpu::Error;


pub struct World {
    pipeline: wgpu::RenderPipeline,
    atlas: Atlas,
    model: Model// the world temporarily has only one block model, for debug purposes

}

impl World {                        ///
    pub fn new(renderer: &Renderer) -> Self {
        let atlas = Atlas::new(&renderer.device, &renderer.queue).unwrap();

        let camera_layout = CameraLayout::new(&renderer.device);
        let render_pipeline_layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&atlas.texture_bind_group_layout, &camera_layout.bind_group_layout],
            push_constant_ranges: &[],
        });

        

        let shader = renderer.device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/shader.wgsl"));




        let pipeline = pipeline_constructor(
            &renderer.device,
            &render_pipeline_layout,
            &[QuadVertex::desc()],
            wgpu::PrimitiveTopology::TriangleList,
            shader,
            &renderer.config
        );

        
        let block = Block::new(MaterialType::GRASS, [0,0,0], [0,0,0]);


        let mut mesh = Mesh::new();
        mesh.push_block(block);

        let model = Model::new(&renderer.device, &mesh).unwrap();

        Self {

            pipeline,
            atlas,
            model
        }
    }
}


impl Draw for World {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) -> Result<(), Error> {

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_bind_group(0, &self.atlas.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &uniforms, &[]);
            render_pass.set_vertex_buffer(0, self.model.vbuf().slice(..));
            //render_pass.set_vertex_buffer(1, self.instance_buffer.buff.slice(..));
            render_pass.set_index_buffer(self.model.ibuf().slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.model.num_indices, 0, 0..1 as _);
        

        Ok(())
    }
}