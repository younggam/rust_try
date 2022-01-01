use crate::renderer::Renderer;

use rust_try_lib::system::core::*;

pub struct InnerApplication<C: ApplicationCore> {
    renderer: Renderer,
    core: C,
}

impl<C: ApplicationCore> InnerApplication<C> {
    fn init(&mut self) {
        self.renderer.init();
        self.core.init();
    }

    pub fn run(mut self) {
        self.init();

        self.core.run();
    }
}

impl InnerApplication<CoreWinit> {
    pub fn new() -> Self {
        let mut core = CoreWinit::new();

        Self {
            renderer: Renderer::new(core.get_winit_target()),
            core,
        }
    }
}

pub type Application = InnerApplication<CoreWinit>;
