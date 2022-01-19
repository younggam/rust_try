///This is interface for application and concrete implementation
///Concrete type of this interface should implement inner logic depends on specific crate
pub trait Application {
    type Window: crate::graphics::window::Window;

    fn init(&mut self) {}

    ///provides executation of assembled logics
    fn run(self);

    ///Mutual call or access should not affect on its purpose or consequence
    fn exit(&self);

    fn window(&self) -> &Self::Window;
}
