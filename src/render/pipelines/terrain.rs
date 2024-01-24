use super::GlobalsLayouts;

pub struct TerrainLayout {
    pub locals: wgpu::BindGroupLayout,
}

impl TerrainLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            locals: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    // locals
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
                ],
            }),
        }
    }
}

pub struct TerrainPipeline {
    pub pipeline: wgpu::RenderPipeline
}

impl TerrainPipeline {
    pub fn new(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        global_layouts: &GlobalsLayouts,
        layout: &TerrainLayout
                

    ) -> Self {

        todo!()
    }
}