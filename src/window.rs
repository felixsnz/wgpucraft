use tokio::runtime::Runtime;
use winit::{
    event_loop::{EventLoop, EventLoopWindowTarget},
        window::WindowBuilder,
        event::{WindowEvent, KeyEvent, ElementState},
        keyboard::{PhysicalKey, KeyCode
    }};

use crate::render::renderer::Renderer;

pub struct Window {
    renderer: Renderer,
    window: winit::window::Window
}


impl Window {

    //TODO add settings as parameter
    pub fn new(runtime: Runtime) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        (
            Self {
                renderer : Renderer::new(&window, &runtime),
                window
            },
            event_loop
        )

    }

    pub fn renderer(&self) -> &Renderer { &self.renderer }

    pub fn renderer_mut(&mut self) -> &mut Renderer { &mut self.renderer }

    pub fn id(&mut self) -> winit::window::WindowId {
        return self.window.id()
    }


    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer.resize(new_size)
        }
    }

    // TODO: add global settings as parameter
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
                self.resize(physical_size);
            },

            WindowEvent::RedrawRequested => {
                self.renderer_mut().update();
                match self.renderer_mut().render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.renderer().size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e)
                }
                // TODO
            },
            
            _ => {}
        }

    }
}