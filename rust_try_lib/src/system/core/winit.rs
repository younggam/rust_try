use std::cell::Cell;

use super::ApplicationCore;
use crate::globals;
use crate::graphics::window::WinitTarget;

use winit::*;

pub struct CoreWinit {
    event_loop: Cell<Option<event_loop::EventLoop<()>>>,
}

impl CoreWinit {
    pub fn new() -> Self {
        Self {
            event_loop: Cell::new(Some(event_loop::EventLoop::new())),
        }
    }

    pub fn get_winit_target(&mut self) -> &WinitTarget {
        self.event_loop.get_mut().as_ref().unwrap()
    }
}

impl ApplicationCore for CoreWinit {
    fn init(&self) {
        globals::init_globals();
    }

    fn run(self) {
        self.init();

        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, _control_flow| match event {
                event::Event::NewEvents(_) => {}
                event::Event::WindowEvent { .. } => {}
                event::Event::DeviceEvent { .. } => {}
                event::Event::UserEvent(_) => {}
                event::Event::Suspended => {}
                event::Event::Resumed => {}
                event::Event::MainEventsCleared => {}
                event::Event::RedrawRequested(_) => {}
                event::Event::RedrawEventsCleared => {}
                event::Event::LoopDestroyed => {}
            });
    }
}
