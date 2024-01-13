use winit:: {
    event::Event,
    event_loop::ControlFlow,
};

use crate::{GameState, window::Window};



pub fn run() {

    env_logger::init();


    //TODO: establish this parameters from settings
    let runtime = tokio::runtime::Builder::new_current_thread()
        .worker_threads(4)
        .thread_name("wgpucraft")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

        let (window, event_loop)= Window::new(runtime);

    let mut game_state = GameState {
        window
    };

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move | event, elwt | {
        match event {
            
            Event::WindowEvent {
                window_id,
                event
            }
            if window_id == game_state.window.id() => {

                game_state.window.handle_window_event(event, elwt);

            },
            
            _ => ()
        }
    }).unwrap();
}