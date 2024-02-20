pub mod renderer;
pub mod pipelines;
pub mod texture;
pub mod atlas;



pub trait Vertex: Clone + bytemuck::Pod {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}