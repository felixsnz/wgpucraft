pub mod renderer;
pub mod pipelines;
pub mod texture;
pub mod atlas;
pub mod mesh;
pub mod model;
pub mod buffer;
pub mod consts;
pub mod binding;




pub trait Vertex: Clone + bytemuck::Pod {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}