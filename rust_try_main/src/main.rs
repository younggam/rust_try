use rust_try_lib::*;

pub mod application;

fn main() {
    let mut app = application::Application::new();
    app.initialize();
    app.run();
}
