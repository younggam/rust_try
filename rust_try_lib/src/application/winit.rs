use crate::application::*;
use crate::*;
use graphics::window::*;
use graphics::GraphicsCore;
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
    graphics: GraphicsCore,

    //common implementation
    scene: Option<Box<dyn Scene>>,
}

impl ApplicationWinit {
    pub fn new<S: 'static + Scene>(title: &'static str, initial_scene: S) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowWinit::new(title, &event_loop);

        Self {
            event_loop: Cell::new(Some(event_loop)),
            graphics: pollster::block_on(GraphicsCore::new(&window)),
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
                    Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
                        match event {
                            WindowEvent::Resized(phyiscal_size) => {
                                self.graphics
                                    .resize(phyiscal_size.width, phyiscal_size.height);
                            }
                            WindowEvent::CloseRequested => Self::exit(),
                            WindowEvent::KeyboardInput { input, .. } => handle_keyboard(input),
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                self.graphics
                                    .resize(new_inner_size.width, new_inner_size.height);
                            }
                            _ => {}
                        }
                    }
                    Event::DeviceEvent { .. } => {}
                    Event::UserEvent(_) => {}
                    Event::Suspended => {}
                    Event::Resumed => {}
                    Event::MainEventsCleared => {
                        keyboard::update();
                        self.graphics.update();
                        self.update();

                        self.window.request_redraw();
                    }
                    Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                        self.graphics.render();
                    }
                    Event::RedrawEventsCleared => {}
                    Event::LoopDestroyed => {
                        self.fin();
                        keyboard::fin();
                        time::fin();
                    }
                    _ => {}
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

#[cfg(feature = "winit")]
fn handle_keyboard(keyboard_input: KeyboardInput) {
    if let Some(key) = keyboard_input.virtual_keycode {
        let key = key as usize;
        let state = match keyboard_input.state {
            ElementState::Pressed => keyboard::KeyState::Pressed,
            ElementState::Released => keyboard::KeyState::Released,
        };
        keyboard::enqueue(key, state);
    }
}

///CONTRACTS THAT NEVER EVER TRIES TO MOVE OR DIRECTLY ACCESS ON EVENT_LOOP AND WINDOW FROM NON-MAIN THREAD
unsafe impl Send for ApplicationWinit {}
unsafe impl Sync for ApplicationWinit {}
