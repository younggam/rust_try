use rust_try_lib::application::Module;

pub struct CoreModule;

impl Module for CoreModule {
    fn init(&mut self) {}

    fn update(&mut self) {}

    fn on_exit(&mut self) {}
}
