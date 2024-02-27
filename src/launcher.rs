use winit:: {
    event::Event,
    event_loop::ControlFlow,
};

use winit::{
    event_loop::{EventLoop, EventLoopWindowTarget},
        window::WindowBuilder,
        event::{WindowEvent, KeyEvent, ElementState},
        keyboard::{PhysicalKey, KeyCode
    }};


use crate::engine::Engine;



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


    //let mut engine = Engine::new(window, runtime);
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move | event, elwt | {
        match event {
            
            Event::WindowEvent {
                window_id,
                event
            }
            if true => {

                print!("Asd");

            },
            
            _ => ()
        }
    }).unwrap();
}