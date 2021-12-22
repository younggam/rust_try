use crate::logic::*;

use rust_try_lib::graphics::Renderer;

pub struct Application {
    logic: Logic,
    renderer: Renderer,
}

impl Application {
    pub fn new() -> Self {
        Self {
            logic: Logic::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.renderer.initialize();
    }

    pub fn run(self) {
        self.logic.run();
    }
}
