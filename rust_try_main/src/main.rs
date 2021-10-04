use rust_try_lib::*;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("rust_try")
        .with_inner_size(winit::dpi::LogicalSize::new(512, 512))
        .build(&event_loop)
        .expect("Failed to create window.");
    let mut rust_try = rust_try::RustTry::new(window);

    //TODO: panic이든 뭐든 무조건 종료(정리) 실행
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::NewEvents(event) => match event {
            winit::event::StartCause::Init => {
                rust_try.on_startup();
            }
            //
            _ => {}
        },
        //
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::KeyboardInput { input, .. } => match input {
                winit::event::KeyboardInput {
                    virtual_keycode,
                    state,
                    ..
                } => match (virtual_keycode, state) {
                    (
                        Some(winit::event::VirtualKeyCode::Escape),
                        winit::event::ElementState::Pressed,
                    ) => {
                        dbg!();
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    //
                    _ => {}
                },
            },
            //
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            //
            winit::event::WindowEvent::Destroyed => {
                rust_try.on_shutdown();
            }
            //
            _ => {}
        },
        //
        winit::event::Event::MainEventsCleared => {
            rust_try.on_render();
        }
        //
        winit::event::Event::LoopDestroyed => {
            rust_try.on_shutdown();
        }
        //
        _ => {}
    });
    //
}
