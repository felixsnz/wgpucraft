use crate::render::buffer::{self, Buffer};
use crate::render::{pipelines::GlobalsLayouts, renderer::Renderer};
use crate::scene::camera::{Camera, CameraLayout, CameraUniform, Projection};
use crate::world::World;
use tokio::runtime::Runtime;
use wgpu::{BindGroup, BindGroupLayoutDescriptor};


use winit::window::Window;


use winit::{
    event_loop::{EventLoop, EventLoopWindowTarget},
        window::WindowBuilder,
        event::{WindowEvent, KeyEvent, ElementState},
        keyboard::{PhysicalKey, KeyCode
    }};



pub struct Engine {
    pub window: Window,
    renderer: Renderer,
    camera: Camera,
    camera_bind_group: BindGroup,
    world: World,

}

impl Engine {

    pub fn new(window: Window, runtime: Runtime) -> Self {

        let camera = Camera::new(
            (0.0, 5.0, 10.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0)
        );

        let renderer = Renderer::new(&window, &runtime);

        let projection = Projection::new(renderer.config.width, renderer.config.height, cgmath::Deg(45.0), 0.1, 100.0);

        

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer = Buffer::new(&renderer.device, wgpu::BufferUsages::UNIFORM, &[camera_uniform]);


        

        let camera_layout = CameraLayout::new(&renderer.device);

        let camera_bind_group = renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_layout.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.buff.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        
        let world = World::new(
            &renderer,
            &renderer.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                entries: &GlobalsLayouts::base_globals_layout(),
                label: Some("Uniform layout")
            }
        ));

        Self {
            window,
            renderer,
            camera,
            camera_bind_group,
            world
            
        }
        
    }

    //TODO: add global settings as parameter
    pub fn handle_window_event(&mut self, event: WindowEvent, elwt: &EventLoopWindowTarget<()>) {

        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key:PhysicalKey::Code(KeyCode::Escape),
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                elwt.exit()
            },

            WindowEvent::Resized(physical_size) => {
                self.renderer.resize(physical_size);
            },

            WindowEvent::RedrawRequested => {
                self.renderer.update();
                match self.renderer.render(&self.world, &self.camera_bind_group) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.renderer.resize(self.renderer.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e)
                }
                
            },
            
            _ => {}
        }

    }

    
}