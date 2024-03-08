
pub mod launcher;
pub mod render;
pub mod scene;

use render::renderer::Renderer;
use scene::Scene;
use tokio::runtime::Runtime;
use winit::{
        event_loop::EventLoopWindowTarget,
        event::{WindowEvent, DeviceEvent, KeyEvent, ElementState},
        keyboard::{PhysicalKey, KeyCode},
        window::Window
    };




pub struct GameState {
    pub window: Window,
    renderer: Renderer,
    scene: Scene

}

impl GameState {

    pub fn new(window: Window, runtime: Runtime) -> Self {

        let mut renderer = Renderer::new(&window, &runtime);

        let scene = Scene::new(&mut renderer);

        Self {
            window,
            renderer,
            scene
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
                let dt = now - self.renderer.last_render_time;
                self.renderer.last_render_time = now;
                self.update(dt);
                match self.renderer.render(&self.scene.terrain, &self.scene.globals_bind_group) {
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
                self.scene.camera.camera_controller.process_scroll(&delta);
            }
            
            _ => {}
        }

            
        }

    }



    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
        self.scene.camera.resize(new_size);
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.scene.update(&mut self.renderer, dt);
        self.renderer.update()
    }

    pub fn input(&mut self, event: &DeviceEvent) {
        self.scene.camera.input(event);
    }

    pub fn input_keyboard(&mut self, event: &WindowEvent) -> bool {
        self.scene.camera.input_keyboard(event)
    }

    
}