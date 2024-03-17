use winit:: {
    event::Event,
    event_loop::ControlFlow,
};

use winit::{
    event_loop::EventLoop,
        window::WindowBuilder,
    };

use crate::Game;

pub fn run() {

    env_logger::init();

    //TODO: establish this parameters from settings
    let runtime = tokio::runtime::Builder::new_current_thread()
        .worker_threads(4)
        .thread_name("wgpucraft")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut game = Game::new(window, runtime);
    game.initialize();
    
    event_loop.run(move | event, elwt: &winit::event_loop::EventLoopWindowTarget<()> | {
        match event {
            Event::WindowEvent {
                window_id,
                event
            }
            if window_id == game.window.id() => {
                game.handle_window_event(event, elwt)
            }
            Event::DeviceEvent { ref event, .. } => {
                game.handle_device_input(event, elwt);
            }
            Event::AboutToWait => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                game.window.request_redraw();
            }
            _ => ()
        }
    }).unwrap();
}