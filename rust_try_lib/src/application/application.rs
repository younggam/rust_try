use crate::{
    application::Scene,
    graphics::{Batch, GraphicsCore},
    input::keyboard,
    time,
};

use std::cell::Cell;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

//kinda.. side-effect of my modular practice
use winit::{event::*, event_loop::*, window::Window};

static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

pub struct Application {
    event_loop: Cell<Option<EventLoop<()>>>,

    graphics_core: Arc<GraphicsCore>,
    batch: Batch,

    //common implementation
    scene: Option<Box<dyn Scene>>,
}

impl Application {
    pub fn new<S: 'static + Scene>(title: &'static str, initial_scene: S) -> Self {
        let event_loop = EventLoop::new();
        let graphics_core = Arc::new(pollster::block_on(GraphicsCore::new(title, &event_loop)));

        Self {
            event_loop: Cell::new(Some(event_loop)),

            graphics_core,
            batch: Batch::new(graphics_core),

            scene: Some(Box::new(initial_scene)),
        }
    }

    fn window(&self) -> &Window {
        &self.batch.window_and_surface().window()
    }

    fn init(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.enter();
        }
    }

    fn update(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.update();
        }
    }

    fn draw(&mut self) {
        if let Some(ref scene) = self.scene {
            scene.draw(&mut self.batch);
        }
    }

    pub fn run(mut self) {
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
                    Event::WindowEvent { window_id, event } if window_id == self.window().id() => {
                        match event {
                            WindowEvent::Resized(new_inner_size) => {
                                self.batch
                                    .resize(new_inner_size);
                            }
                            WindowEvent::CloseRequested => Self::exit(),
                            WindowEvent::KeyboardInput { input, .. } => handle_keyboard(input),
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                self.batch
                                    .resize(*new_inner_size);
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
                        self.batch.window_and_surface_mut().update();
                        self.update();
                        self.draw();

                        self.window().request_redraw();
                    }
                    Event::RedrawRequested(_window_id) /*if window_id == self.window.id()*/=> {
                        self.batch.present();
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

    pub fn exit() {
        SHOULD_EXIT.store(true, Ordering::Release);
    }

    fn should_exit() -> bool {
        SHOULD_EXIT.load(Ordering::Acquire)
    }
}

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
