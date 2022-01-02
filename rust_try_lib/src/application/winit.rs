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
    is_running: bool,
}

impl ApplicationWinit {
    pub fn new<C: 'static + Module>(core: C) -> Self {
        let event_loop = EventLoop::new();

        Self {
            window: WindowWinit::new(&event_loop),
            event_loop: Cell::new(Some(event_loop)),

            core: Box::new(core),
            is_running: true,
        }
    }

    fn init_globals(&self) {
        lazy_static::initialize(&globals::EVENT_REGISTRY);
        //rust_try_lib::globals::APPLICATION_WINIT.init(std::sync::Mutex::new(self));
    }
}

impl Application for ApplicationWinit {
    fn init(&mut self) {
        self.init_globals();

        self.window.set_title("Rust Try");

        self.core.init();
    }

    fn run(mut self) {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                match event {
                    Event::NewEvents(start_cause) => match start_cause {
                        StartCause::Init => self.init(),
                        _ => {}
                    },
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => self.exit(),
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
                }

                if !self.is_running {
                    *control_flow = ControlFlow::Exit;
                }
            });
    }

    fn exit(&mut self) {
        self.is_running = false;
    }
}

///CONTRACTS THAT NEVER EVER TRIES TO MOVE OR DIRECTLY ACCESS ON EVENT_LOOP AND WINDOW FROM NON-MAIN THREAD
unsafe impl Send for ApplicationWinit {}
unsafe impl Sync for ApplicationWinit {}
