use crate::*;

use std::ffi::CString;

use ash::vk;

use raw_window_handle::HasRawWindowHandle;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Application {
    event_loop: utils::LazyManual<utils::Once<winit::event_loop::EventLoop<()>>>,
    window: utils::LazyManual<winit::window::Window>,
    entry: ash::Entry,
}

impl Application {
    pub fn new() -> Self {
        Self {
            entry: unsafe { ash::Entry::new().expect("Vulkan functions loading error") },
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

    fn init_vulkan(&self) {
        self.create_instance();
    }

    fn main_loop(mut self) {
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

    fn create_instance(&self) {
        let application_name = CString::new("Hello Triangle").unwrap();
        let engine_name = CString::new("No Engine").unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&application_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_0);

        let surface_extensions = ash_window::enumerate_required_extensions(self.window.get())
            .unwrap()
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<_>>();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&surface_extensions);

        unsafe {
            self.entry
                .create_instance(&create_info, None)
                .expect("failed to create instance!")
        };
    }

    //도전과제 : Challenge
}

impl Drop for Application {
    fn drop(&mut self) {}
}
