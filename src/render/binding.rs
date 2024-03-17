

use wgpu::BindGroup;

use crate::render::{renderer::Renderer, texture::Texture};

impl Renderer {


    pub fn bind_atlas_texture(
        &self,
        tex: &Texture,
    ) -> BindGroup {
        self.layouts.global.bind_atlas_texture(
            &self.device,
            tex,
        )
    }
}