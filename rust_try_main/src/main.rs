use rust_try_lib::*;

mod application;

mod logic;

static mut EVENT_REGISTRY: utils::LazyManual<system::EventRegistry> = utils::LazyManual::new();

fn main() {

    // let mut app = application::Application::new();
    // app.initialize();
    // app.run();
}
