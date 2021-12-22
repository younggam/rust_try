use super::control::*;

pub struct InnerMain<T: Control> {
    control: T,
}

#[cfg(feature = "winit")]
impl InnerMain<WinitControl> {
    pub fn new() -> Self {
        let mut control = WinitControl::new();
        control.set_start_task(Self::start);
        control.set_update_task(Self::update);
        control.set_render_task(Self::render);
        control.set_quit_task(Self::quit);

        Self { control }
    }

    fn start() {}

    fn update() {}

    fn render() {}

    fn quit() {}

    pub fn run(self) {
        self.control.run();
    }
}

#[cfg(feature = "winit")]
pub type Main = InnerMain<WinitControl>;
