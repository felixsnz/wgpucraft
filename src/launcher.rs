use winit:: {
    event::{Event, WindowEvent, KeyEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, keyboard::{PhysicalKey, KeyCode}
};

pub fn run() {

    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move | event, elwt | {
        match event {
            Event::WindowEvent {
                window_id,
                event
            }
            if window_id == window.id() => match event {
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
                _ => {}
            },
            _ => ()
        }
    }).unwrap();
}