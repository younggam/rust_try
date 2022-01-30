use super::Window;

use raw_window_handle::*;

pub struct WindowWinit {
    inner: winit::window::Window,
}

pub type WinitTarget = winit::event_loop::EventLoopWindowTarget<()>;

impl WindowWinit {
    pub fn new(title: &'static str, winit_target: &WinitTarget) -> Self {
        Self {
            inner: winit::window::WindowBuilder::new()
                .with_title(title)
                .build(winit_target)
                .unwrap(),
        }
    }
}

impl Window for WindowWinit {
    fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }

    fn inner_size(&self) -> (u32, u32) {
        let winit::dpi::PhysicalSize { width, height } = self.inner.inner_size();
        (width, height)
    }

    fn outer_size(&self) -> (u32, u32) {
        let winit::dpi::PhysicalSize { width, height } = self.inner.outer_size();
        (width, height)
    }

    fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }

    fn set_resizable(&self, resizeable: bool) {
        self.inner.set_resizable(resizeable);
    }
}

impl std::ops::Deref for WindowWinit {
    type Target = winit::window::Window;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl HasRawWindowHandle for WindowWinit {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}
