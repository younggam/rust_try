use super::Window;

use raw_window_handle::*;

pub struct WindowWinit {
    inner: winit::window::Window,
}

pub type WinitTarget = winit::event_loop::EventLoopWindowTarget<()>;

impl WindowWinit {
    pub fn new(winit_target: &WinitTarget) -> Self {
        Self {
            inner: winit::window::Window::new(winit_target).unwrap(),
        }
    }
}

impl Window for WindowWinit {
    fn as_raw_window_handle(&self) -> &dyn HasRawWindowHandle {
        &self.inner
    }

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

unsafe impl HasRawWindowHandle for WindowWinit {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}
