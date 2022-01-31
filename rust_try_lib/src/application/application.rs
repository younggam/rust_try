use std::sync::atomic::{AtomicBool, Ordering};

static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

///This is interface for application and concrete implementation
///Concrete type of this interface should implement inner logic depends on specific crate
pub trait Application {
    type Window: crate::graphics::window::Window;

    fn init(&mut self) {}

    ///provides executation of assembled logics
    fn run(self);

    ///Mutual call or access should not affect on its purpose or consequence
    fn fin(&mut self);

    fn exit() {
        SHOULD_EXIT.store(true, Ordering::Relaxed);
    }

    fn should_exit() -> bool {
        SHOULD_EXIT.load(Ordering::Relaxed)
    }

    fn window(&self) -> &Self::Window;
}
