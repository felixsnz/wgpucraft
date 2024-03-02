

pub fn constructor(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    primitive_topology: wgpu::PrimitiveTopology,
    shader: wgpu::ShaderModule,
    config: &wgpu::SurfaceConfiguration,//in the future i better add a config struct global

) -> wgpu::RenderPipeline {

    let mut primitive = wgpu::PrimitiveState::default();
    primitive.topology = primitive_topology;
    primitive.front_face = wgpu::FrontFace::Ccw;
    primitive.cull_mode = Some(wgpu::Face::Back);

    //later use the config global struct to control this mode
    primitive.polygon_mode = wgpu::PolygonMode::Fill;



    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render generic Pipeline"),
        layout: Some(&layout),
        primitive: primitive,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        depth_stencil: None, //change this when add depth texture
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}


pub struct GlobalsLayouts {
    pub globals: wgpu::BindGroupLayout,
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

        Self {
            globals,
        }
    }
}