mod application;

mod core;

fn main() {
    let mut app = application::Application::new();
    app.initialize();
    app.run();
}
