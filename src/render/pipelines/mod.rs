
pub mod terrain;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};
use wgpu::BindGroup;

use super::{consts::Consts, texture::Texture};

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct Globals {
    /// Transformation from world coordinate space (with focus_off as the
    /// origin) to the camera space
    view_proj: [[f32; 4]; 4],

}

impl Globals {
    /// Create global consts from the provided parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        view_proj: [[f32; 4]; 4]
    ) -> Self {
        Self {
            view_proj

        }
    }
}

impl Default for Globals {
    fn default() -> Self {
        Self::new(
            Matrix4::identity().into(),

        )
    }
}



// Global scene data spread across several arrays.
pub struct GlobalModel {
    pub globals: Consts<Globals>,

}

pub struct GlobalsLayouts {
    pub globals: wgpu::BindGroupLayout,
    pub atlas_layout: wgpu::BindGroupLayout
}

impl GlobalsLayouts {
    pub fn base_globals_layout() -> Vec<wgpu::BindGroupLayoutEntry> {
        vec![
            // Global uniform
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ]
    }

    pub fn new(device: &wgpu::Device) -> Self {
        let globals = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Globals layout"),
            entries: &Self::base_globals_layout(),
        });

        let atlas_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
            label: Some("atlas_bind_group_layout"),
        });

        Self {
            globals,
            atlas_layout
        }
    }

    fn base_global_entries(
        global_model: &GlobalModel
    ) -> Vec<wgpu::BindGroupEntry> {
        vec![
            // Global uniform
            wgpu::BindGroupEntry {
                binding: 0,
                resource: global_model.globals.buf().as_entire_binding(),
            },
        ]
    }

    pub fn bind(
        &self,
        device: &wgpu::Device,
        global_model: &GlobalModel,
    ) -> BindGroup {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals,
            entries: &Self::base_global_entries(global_model),
        });

        bind_group
    }

    pub fn bind_atlas_texture(
        &self,
        device: &wgpu::Device,
        texture: &Texture
    ) -> BindGroup {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.globals,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        bind_group
    }
}