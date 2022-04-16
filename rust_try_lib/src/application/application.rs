use crate::{application::Scene, graphics::Graphics, inputs::Inputs, utils::Utils};

use std::{
    cell::Cell,
    sync::atomic::{AtomicBool, Ordering},
};

//kinda.. side-effect of my modular practice
use winit::{event::*, event_loop::*, window::WindowId};

static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

pub struct Application {
    _title: &'static str,

    event_loop: Cell<Option<EventLoop<()>>>,

    frame_per_sec: f64,

    graphics: Graphics,
    utils: Utils,
    inputs: Inputs,

    //common implementation
    scene: Option<Box<dyn Scene>>,
}

impl Application {
    pub fn new(title: &'static str) -> Self {
        let event_loop = EventLoop::new();
        let graphics = pollster::block_on(Graphics::new(title, &event_loop));

        Self {
            _title: title,

            event_loop: Cell::new(Some(event_loop)),

            frame_per_sec: 60.0,

            graphics,
            utils: Utils::new(),
            inputs: Inputs::new(),

            scene: None,
        }
    }

    pub fn graphics(&self) -> &Graphics {
        &self.graphics
    }
}

impl Application {
    fn init(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.enter();
        }
    }

    fn resize(&mut self, window_id: WindowId, new_size: winit::dpi::PhysicalSize<u32>) {
        self.graphics.resize(window_id, new_size);
        if let Some(ref mut scene) = self.scene {
            scene.resize(new_size);
        }
    }

    fn pre_update(&mut self) {
        self.utils.pre_update();
        self.inputs.pre_update();
    }

    fn update(&mut self) {
        self.graphics.update();

        if let Some(ref mut scene) = self.scene {
            scene.update(&self.utils, &self.inputs);
        }
    }

    fn draw(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.render(&self.graphics);
        }
    }

    pub fn run(mut self, initial_scene: impl Scene + 'static) {
        self.scene = Some(Box::new(initial_scene));
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                match event {
                    Event::NewEvents(start_cause) => match start_cause {
                        StartCause::Init => self.init(),
                        _ => {
                            while 1.0 / self.frame_per_sec
                                > self.utils.time_elapsed() - self.utils.time()
                            {
                                std::hint::spin_loop();
                            }
                            self.pre_update();
                        }
                    },
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::CloseRequested => Self::exit(),
                        WindowEvent::Resized(new_inner_size) => {
                            self.resize(window_id, new_inner_size)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.resize(window_id, *new_inner_size)
                        }
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
