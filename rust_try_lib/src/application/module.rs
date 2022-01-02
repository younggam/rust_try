pub trait Module {
    fn init(&mut self);

    fn update(&mut self);

    fn on_exit(&mut self);
}
