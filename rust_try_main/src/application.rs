use crate::core::*;

use rust_try_lib::graphics::Renderer;

pub struct Application {
    core: Core,
    renderer: Renderer,
}

impl Application {
    pub fn new() -> Self {
        Self {
            core: Core::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.renderer.initialize();
    }

    pub fn run(self) {
        self.core.run();
    }
}
