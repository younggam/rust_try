use crate::application::*;
use crate::globals;
use crate::graphics::wgpu::*;
use crate::graphics::window::*;

use std::cell::Cell;

//kinda.. side-effect of my modular practice
extern crate winit as dep;
use dep::{event::*, event_loop::*};

pub struct ApplicationWinit {
    //dependency specific
    window: WindowWinit,
    event_loop: Cell<Option<EventLoop<()>>>,
    graphics: GraphicsCoreWgpu,

    //common implementation
    scene: Option<Box<dyn Scene>>,
    is_running: bool,
}

impl ApplicationWinit {
    pub fn new<S: 'static + Scene>(title: &'static str, initial_scene: S) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowWinit::new(title, &event_loop);

        Self {
            event_loop: Cell::new(Some(event_loop)),
            graphics: pollster::block_on(GraphicsCoreWgpu::new(&window)),
            window,

            scene: Some(Box::new(initial_scene)),
            is_running: true,
        }
    }

    fn update(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.update();
        }
    }

    fn on_exit(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.force_exit();
        }

        globals::finalize();
    }
}

impl Application for ApplicationWinit {
    type Window = WindowWinit;

    fn init(&mut self) {
        globals::init();
        unsafe {
            globals::APPLICATION.init(crate::utils::UnsafeRef::new(self));
        }

        if let Some(ref mut scene) = self.scene {
            scene.enter();
        }
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
                    Event::MainEventsCleared => {
                        self.update();
                    }
                    Event::RedrawRequested(_) => {}
                    Event::RedrawEventsCleared => {}
                    Event::LoopDestroyed => self.on_exit(),
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

    fn window(&self) -> &Self::Window {
        &self.window
    }
}

///CONTRACTS THAT NEVER EVER TRIES TO MOVE OR DIRECTLY ACCESS ON EVENT_LOOP AND WINDOW FROM NON-MAIN THREAD
unsafe impl Send for ApplicationWinit {}
unsafe impl Sync for ApplicationWinit {}
