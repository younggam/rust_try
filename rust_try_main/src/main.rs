mod application;

mod renderer;

fn main() {
    let app = application::Application::new();
    app.run();
}
