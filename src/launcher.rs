use winit:: {
    event::Event,
    event_loop::ControlFlow,
};

use winit::{
    event_loop::EventLoop,
        window::WindowBuilder,
    };

use crate::GameState;


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
    let window = WindowBuilder::new().build(&event_loop).unwrap();


    let mut state = GameState::new(window, runtime);
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move | event, elwt | {
        match event {

            

            Event::DeviceEvent { ref event, .. } => {
                state.input(event);
            }
            
            
            Event::WindowEvent {
                window_id,
                event
            }
            if window_id == state.window.id() => {
                

                state.handle_window_event(event, elwt)

            }
            Event::AboutToWait => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window.request_redraw();
            }
            
            _ => ()
        }
    }).unwrap();
}