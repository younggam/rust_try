use crate::*;

use ash::vk;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Application {
    event_loop: utils::LazyManual<utils::Once<winit::event_loop::EventLoop<()>>>,
    window: utils::LazyManual<winit::window::Window>,
    entry: ash::Entry,
}

impl Application {
    pub fn new() -> Self {
        let entry = unsafe { ash::Entry::new().expect("Vulkan functions loading error") };

        Self {
            entry,
            event_loop: utils::LazyManual::new(),
            window: utils::LazyManual::new(),
        }
    }

    pub fn run(mut self) {
        self.init_window();
        self.init_vulkan();
        self.main_loop();
    }

    fn init_window(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new();

        self.window.init(
            winit::window::WindowBuilder::new()
                .with_title("Vulkan")
                .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
                .with_resizable(false)
                .build(&event_loop)
                .expect("Failed to create window."),
        );

        self.event_loop.init(utils::Once::new(event_loop));
    }

    fn init_vulkan(&self) {}

    fn main_loop(mut self) {
        //TODO: panic이든 뭐든 무조건 종료(정리) 실행
        self.event_loop
            .get_mut()
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
