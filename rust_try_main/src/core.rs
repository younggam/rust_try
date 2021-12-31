use rust_try_lib::system::backend::*;

pub struct InnerCore<T: Backend> {
    backend: T,
}

impl InnerCore<WinitBackend> {
    pub fn new() -> Self {
        let mut backend = WinitBackend::new();
        backend.set_start_task(Self::start);
        backend.set_update_task(Self::update);
        backend.set_render_task(Self::render);
        backend.set_quit_task(Self::quit);

        Self { backend }
    }

    fn start() {}

    fn update() {}

    fn render() {}

    fn quit() {}

    pub fn run(self) {
        self.backend.run();
    }
}

pub type Core = InnerCore<WinitBackend>;
