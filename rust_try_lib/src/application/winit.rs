use std::cell::Cell;

use crate::application::*;
use crate::globals;
use crate::graphics::window::*;

//kinda.. side-effect of my modular practice
extern crate winit as dep;
use dep::{event::*, event_loop::*};

pub struct ApplicationWinit {
    //dependency specific
    window: WindowWinit,
    event_loop: Cell<Option<EventLoop<()>>>,

    //user implementation
    core: Box<dyn Module>,
}

impl ApplicationWinit {
    pub fn new<C: 'static + Module>(core: C) -> Self {
        let event_loop = EventLoop::new();

        Self {
            window: WindowWinit::new(&event_loop),
            event_loop: Cell::new(Some(event_loop)),

            core: Box::new(core),
        }
    }
}

impl Application for ApplicationWinit {
    fn init(&mut self) {
        globals::init_globals();

        self.window.set_title("Rust Try");

        self.core.init();
    }

    fn run(mut self) {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| match event {
                Event::NewEvents(start_cause) => match start_cause {
                    StartCause::Init => self.init(),
                    _ => {}
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::MainEventsCleared => {}
                Event::RedrawRequested(_) => {}
                Event::RedrawEventsCleared => {}
                Event::LoopDestroyed => self.core.on_exit(),
            });
    }
}
