use crate::{inputs::Inputs, graphics::Graphics};

pub trait Scene: Send {
    fn enter(&mut self);

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);

    fn update(&mut self, inputs: &Inputs);

    fn render(&mut self, graphics: &Graphics);

    fn should_exit(&self);

    fn exit(&mut self) -> Option<Box<dyn Scene>>;

    fn force_exit(&mut self);
}
