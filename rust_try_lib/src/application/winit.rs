use crate::application::*;
use crate::*;
use graphics::wgpu::*;
use graphics::window::*;
use input::keyboard;
use time;

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
        }
    }

    fn update(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.update();
        }
    }
}

impl Application for ApplicationWinit {
    type Window = WindowWinit;

    fn init(&mut self) {
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
                        StartCause::Init => {
                            time::init();
                            keyboard::init();
                            self.init();
                        }
                        StartCause::Poll => {
                            time::update();
                            keyboard::pre_update();
                        }
                        _ => {}
                    },
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => Self::exit(),
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(key) = input.virtual_keycode {
                                let key = key as usize;
                                let state = match input.state {
                                    ElementState::Pressed => keyboard::KeyState::Pressed,
                                    ElementState::Released => keyboard::KeyState::Released,
                                };
                                keyboard::enqueue(key, state);
                            }
                        }
                        _ => {}
                    },
                    Event::DeviceEvent { .. } => {}
                    Event::UserEvent(_) => {}
                    Event::Suspended => {}
                    Event::Resumed => {}
                    Event::MainEventsCleared => {
                        keyboard::update();
                        self.update();
                    }
                    Event::RedrawRequested(_) => {}
                    Event::RedrawEventsCleared => {}
                    Event::LoopDestroyed => {
                        self.fin();
                        keyboard::fin();
                        time::fin();
                    }
                }

                if Self::should_exit() {
                    *control_flow = ControlFlow::Exit;
                }
            });
    }

    fn fin(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.force_exit();
        }
    }

    fn window(&self) -> &Self::Window {
        &self.window
    }
}

///CONTRACTS THAT NEVER EVER TRIES TO MOVE OR DIRECTLY ACCESS ON EVENT_LOOP AND WINDOW FROM NON-MAIN THREAD
unsafe impl Send for ApplicationWinit {}
unsafe impl Sync for ApplicationWinit {}
