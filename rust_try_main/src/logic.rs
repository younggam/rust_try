use rust_try_lib::system::control::*;

pub struct InnerLogic<T: Control> {
    control: T,
}

impl InnerLogic<WinitControl> {
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

pub type Logic = InnerLogic<WinitControl>;
