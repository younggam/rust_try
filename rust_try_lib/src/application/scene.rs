use crate::graphics::Batch;

pub trait Scene: Send {
    fn enter(&mut self);

    fn update(&mut self);

    fn draw(&self, graphics: &mut Batch);

    fn should_exit(&self);

    fn exit(&mut self) -> Option<Box<dyn Scene>>;

    fn force_exit(&mut self);
}
