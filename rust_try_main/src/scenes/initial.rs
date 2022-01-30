use rust_try_lib::application::Scene;

pub struct InitialScene;

impl Scene for InitialScene {
    fn enter(&mut self) {}

    fn update(&mut self) {}

    fn should_exit(&self) {}

    fn exit(&mut self) -> Option<Box<dyn Scene>> {
        None
    }

    fn force_exit(&mut self) {}
}
