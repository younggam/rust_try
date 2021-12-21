use super::control::*;

pub struct InnerMain<T: Control> {
    control: T,
}

#[cfg(feature = "winit")]
impl InnerMain<WinitControl> {
    pub fn new() -> Self {
        Self {
            control: WinitControl::new(),
        }
    }

    pub fn run(self) {
        self.control.run();
    }
}

#[cfg(feature = "winit")]
pub type Main = InnerMain<WinitControl>;
