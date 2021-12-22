use rust_try_lib::*;

mod application;

mod logic;

static mut EVENT_REGISTRY: utils::LazyManual<system::EventRegistry<system::TestEvent>> =
    utils::LazyManual::new();

fn main() {
    unsafe {
        EVENT_REGISTRY.init(system::EventRegistry::<system::TestEvent>::new());
    }

    let mut app = application::Application::new();
    app.initialize();
    app.run();
}
