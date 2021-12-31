mod application;

mod logic;

mod globals;

fn main() {
    // globals::init_globals();

    let mut app = application::Application::new();
    app.initialize();
    app.run();
}
