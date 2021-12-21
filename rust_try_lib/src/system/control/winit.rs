use super::Control;

use winit::*;

pub struct WinitControl {
    event_loop: event_loop::EventLoop<()>,
}

impl WinitControl {
    pub fn new() -> Self {
        Self {
            event_loop: event_loop::EventLoop::new(),
        }
    }
}

impl Control for WinitControl {
    fn run(self) {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                event::Event::WindowEvent { event, .. } => match event {
                    event::WindowEvent::CloseRequested => {
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            });
    }
}
