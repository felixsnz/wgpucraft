use crate::render::buffer::{self, Buffer};
use crate::render::{pipelines::GlobalsLayouts, renderer::Renderer};
use crate::scene::camera::{Camera, CameraLayout, Projection};
use crate::world::World;
use instant::Instant;
use tokio::runtime::Runtime;
use wgpu::{BindGroup, BindGroupLayoutDescriptor};


use winit::event::{DeviceEvent, MouseButton};
use winit::platform::scancode::PhysicalKeyExtScancode;
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
    camera_bind_group: BindGroup,
    world: World,
    camera:Camera,
    mouse_pressed: bool, //temporal until i find a better place to store this state
    last_render_time: Instant 

}

impl Engine {

    pub fn new(window: Window, runtime: Runtime) -> Self {

        let mut last_render_time = instant::Instant::now();


        
        let renderer = Renderer::new(&window, &runtime);
        let camera = Camera::new(&renderer, (0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));


        let camera_layout = CameraLayout::new(&renderer.device);

        let camera_bind_group = renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_layout.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera.uniform_buffer.buff.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        
        let world = World::new(
            &renderer,
            // &renderer.device.create_bind_group_layout(
            // &BindGroupLayoutDescriptor {
            //     entries: &GlobalsLayouts::base_globals_layout(),
            //     label: Some("Uniform layout")
            //     }
            // )
        );

        Self {
            window,
            renderer,
            camera_bind_group,
            world,
            camera,
            mouse_pressed:false,
            last_render_time
            
        }
        
    }

    //TODO: add global settings as parameter
    pub fn handle_window_event(&mut self, event: WindowEvent, elwt: &EventLoopWindowTarget<()>) {
        if !self.input_keyboard(&event) {
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
            _ => {}
        }
        match event {
            
            WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                let dt = now - self.last_render_time;
                self.last_render_time = now;
                self.update(dt);
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
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.camera_controller.process_scroll(&delta);
            }
            
            _ => {}
        }

            
        }

    }



    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
        self.camera.resize(new_size);
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.camera.update(&self.renderer.queue, dt);
        self.renderer.update()
        //self.world.set_center(&self.renderer.queue, self.camera.position.to_vec());
    }

    pub fn input(&mut self, event: &DeviceEvent) {
        self.camera.input(event);
    }

    pub fn input_keyboard(&mut self, event: &WindowEvent) -> bool {
        self.camera.input_keyboard(event)
    }

    
}