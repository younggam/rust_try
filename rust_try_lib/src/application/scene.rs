use crate::{graphics::Graphics, inputs::Inputs, utils::Utils};

use winit::window::WindowId;

pub trait Scene: Send {
    fn enter(&mut self);

    fn resize(&mut self, window_id: WindowId, new_size: winit::dpi::PhysicalSize<u32>);

    fn update(&mut self, utils: &Utils, inputs: &Inputs);

    fn render(&mut self, graphics: &Graphics);

    fn should_exit(&self);

    fn exit(&mut self) -> Option<Box<dyn Scene>>;

    fn force_exit(&mut self);
}
