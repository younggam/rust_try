use rust_try_lib::graphics::Renderer;
use rust_try_lib::system::Main;

pub struct Application {
    main: Main,
    renderer: Renderer,
}

impl Application {
    pub fn new() -> Self {
        Self {
            main: Main::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.renderer.initialize();
    }

    pub fn run(self) {
        self.main.run();
    }
}
