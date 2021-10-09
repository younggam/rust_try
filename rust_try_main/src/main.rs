use rust_try_lib::*;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let rust_try = crate::Application::new("rust_try", &event_loop);
    rust_try.run(event_loop);
}
