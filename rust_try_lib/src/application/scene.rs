pub trait Scene: Send {
    fn enter(&mut self);

    fn update(&mut self);

    fn should_exit(&self);

    fn exit(&mut self) -> Option<Box<dyn Scene>>;

    fn force_exit(&mut self);
}
