use rust_try_lib::graphics::core::*;
use rust_try_lib::graphics::window::*;

pub struct InnerRenderer<C: RendererCore, W: Window> {
    core: C,
    window: W,
}

impl<C: RendererCore, W: Window> InnerRenderer<C, W> {
    pub fn init(&mut self) {
        self.core.init(self.window.as_raw_window_handle());
    }

    pub fn render(&mut self) {
        self.core.render();
    }
}

impl InnerRenderer<CoreAsh, WindowWinit> {
    pub fn new(winit_target: &WinitTarget) -> Self {
        let window = WindowWinit::new(winit_target);
        window.set_title("Rust Try");
        window.set_resizable(true);

        Self {
            core: CoreAsh::new(),
            window,
        }
    }
}

pub type Renderer = InnerRenderer<CoreAsh, WindowWinit>;
