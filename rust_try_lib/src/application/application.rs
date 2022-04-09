use crate::{
    application::Scene,
    graphics::{Batch, Graphics},
    inputs::Inputs,
    utils::Utils,
};

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

//kinda.. side-effect of my modular practice
use winit::{event::*, event_loop::*};

static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

pub struct Application {
    _title: &'static str,

    event_loop: Cell<Option<EventLoop<()>>>,

    graphics: Graphics,
    batch: Batch,

    utils: Utils,
    inputs: Inputs,

    //common implementation
    scene: Option<Box<dyn Scene>>,
}

impl Application {
    pub fn new(title: &'static str, initial_scene: impl Scene + 'static) -> Self {
        let event_loop = EventLoop::new();
        let graphics = pollster::block_on(Graphics::new(title, &event_loop));

        Self {
            _title: title,

            event_loop: Cell::new(Some(event_loop)),

            batch: Batch::new(&graphics),
            graphics,

            utils: Utils::new(),
            inputs: Inputs::new(),

            scene: Some(Box::new(initial_scene)),
        }
    }

    fn init(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.enter();
        }
    }

    fn pre_update(&mut self) {
        self.inputs.pre_update();
    }

    fn update(&mut self) {
        self.utils.update();
        self.inputs.update();
        self.graphics.update();

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
                        StartCause::Init => self.init(),
                        _ => self.pre_update(),
                    },
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::CloseRequested => Self::exit(),
                        WindowEvent::Resized(new_inner_size) => {
                            self.batch
                                .resize(&mut self.graphics, window_id, new_inner_size)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self
                            .batch
                            .resize(&mut self.graphics, window_id, *new_inner_size),
                        _ => self.inputs.handle_input(event),
                    },
                    Event::DeviceEvent { .. } => {}
                    Event::UserEvent(_) => {}
                    Event::Suspended => {}
                    Event::Resumed => {}
                    Event::MainEventsCleared => {
                        self.update();
                        self.draw();
                    }
                    Event::RedrawRequested(_window_id) => {
                        self.batch.flush(&mut self.graphics);
                        self.graphics.present();
                    }
                    Event::RedrawEventsCleared => {}
                    Event::LoopDestroyed => self.fin(),
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
