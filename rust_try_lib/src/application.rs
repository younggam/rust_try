pub struct Application {
    event_loop: crate::utils::Once<winit::event_loop::EventLoop<()>>,
    window: winit::window::Window,
    renderer: crate::graphics::Renderer,
    entry: ash::Entry,
}

impl Application {
    pub fn new(name: &str) -> Self {
        let entry = unsafe { ash::Entry::new().expect("Vulkan functions loading error") };
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(name)
            .with_inner_size(winit::dpi::LogicalSize::new(512, 512))
            .build(&event_loop)
            .expect("Failed to create window.");

        let renderer = crate::graphics::Renderer::new(&entry);

        Self {
            entry,
            event_loop: crate::utils::Once::new(event_loop),
            window,
            renderer,
        }
    }

    pub fn run(mut self) {
        //TODO: panic이든 뭐든 무조건 종료(정리) 실행
        self.event_loop
            .consume()
            .run(move |event, _, control_flow| match event {
                winit::event::Event::NewEvents(event) => match event {
                    winit::event::StartCause::Init => {
                        self.renderer.startup(&self.entry, &self.window);
                    }
                    _ => {}
                },
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
                            _ => {}
                        },
                    },
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                },
                winit::event::Event::MainEventsCleared => {
                    self.renderer.render();
                }
                winit::event::Event::LoopDestroyed => {
                    self.renderer.shutdown();
                }
                _ => {}
            });
    }
}
