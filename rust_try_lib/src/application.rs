use ash::vk;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Application {
    event_loop: crate::utils::Once<winit::event_loop::EventLoop<()>>,
    window: winit::window::Window,
    //entry: ash::Entry,
}

impl Application {
    pub fn new(name: &str) -> Self {
        //let entry = unsafe { ash::Entry::new().expect("Vulkan functions loading error") };
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(name)
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
            .with_resizable(false)
            .build(&event_loop)
            .expect("Failed to create window.");

        Self {
            //entry,
            event_loop: crate::utils::Once::new(event_loop),
            window,
        }
    }

    pub fn run(mut self) {}

    fn init_window() {}

    fn init_vulkan() {}

    fn main_loop(&mut self) {
        //TODO: panic이든 뭐든 무조건 종료(정리) 실행
        self.event_loop
            .consume()
            .run(move |event, _, control_flow| match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            });
    }
}

impl Drop for Application {
    fn drop(&mut self) {}
}
