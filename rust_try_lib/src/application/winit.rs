use crate::application::*;
use crate::globals;
use crate::graphics::window::*;

use std::cell::Cell;

//kinda.. side-effect of my modular practice
extern crate winit as dep;
use dep::{event::*, event_loop::*};

pub struct ApplicationWinit {
    //dependency specific
    window: WindowWinit,
    event_loop: Cell<Option<EventLoop<()>>>,

    //common implementation
    title: &'static str,
    core: Box<dyn Module>,
    modules: Vec<Box<dyn Module>>,
    is_running: bool,
}

impl ApplicationWinit {
    pub fn new<C: 'static + Module>(title: &'static str, core: C) -> Self {
        let event_loop = EventLoop::new();

        Self {
            window: WindowWinit::new(&event_loop),
            event_loop: Cell::new(Some(event_loop)),

            title,
            core: Box::new(core),
            modules: Vec::new(),
            is_running: true,
        }
    }

    fn update(&mut self) {
        self.core.update();
        self.operate(Module::update);
    }

    fn operate(&mut self, op: fn(&mut (dyn Module + 'static))) {
        for module in self.modules.iter_mut() {
            op(module.as_mut());
        }
    }

    fn graphic_test() {
        let mut g = crate::graphics::ash::GraphicsCoreAsh::new();
        g.init();
    }
}

impl Application for ApplicationWinit {
    fn init(&mut self) {
        globals::init();
        unsafe { globals::APPLICATION.init(crate::utils::UnsafeRef::new(self)) };

        self.window.set_title(self.title);

        self.core.init();
        Self::graphic_test();
    }

    fn run(mut self) {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                match event {
                    Event::NewEvents(start_cause) => match start_cause {
                        StartCause::Init => self.init(),
                        StartCause::Poll => globals::pre_update(),
                        _ => {}
                    },
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => self.exit(),
                        WindowEvent::KeyboardInput { input, .. } => {
                            unsafe { globals::KEYBOARD.get_mut().handle_input(input) };
                        }
                        _ => {}
                    },
                    Event::DeviceEvent { .. } => {}
                    Event::UserEvent(_) => {}
                    Event::Suspended => {}
                    Event::Resumed => {}
                    Event::MainEventsCleared => self.update(),
                    Event::RedrawRequested(_) => {}
                    Event::RedrawEventsCleared => {}
                    Event::LoopDestroyed => self.core.on_exit(),
                }

                if !self.is_running {
                    *control_flow = ControlFlow::Exit;
                }
            });
    }

    fn exit(&self) {
        //# SAFETY
        //Mutual call or access doesn't affect on its purpose
        unsafe { &mut *(self as *const Self as *mut Self) }.is_running = false;
    }

    fn raw_window_handle(&self) -> &dyn raw_window_handle::HasRawWindowHandle {
        self.window.as_raw_window_handle()
    }
}

///CONTRACTS THAT NEVER EVER TRIES TO MOVE OR DIRECTLY ACCESS ON EVENT_LOOP AND WINDOW FROM NON-MAIN THREAD
unsafe impl Send for ApplicationWinit {}
unsafe impl Sync for ApplicationWinit {}
