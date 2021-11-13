use crate::*;

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;

use ash::vk;

//Window surface size.
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: [&[u8]; 1] = [b"VK_LAYER_KHRONOS_validation"];

const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

lazy_struct! {
/**Temporary struct(possibly permanent) that manages whole application.

Following Vulkan Tutorial.*/
    //2021.11.07
    pub struct Application {
        entry: ash::Entry,

        -event_loop: utils::Once<winit::event_loop::EventLoop<()>>,
        -window: winit::window::Window,

        -instance: ash::Instance,

        -debug_utils: ash::extensions::ext::DebugUtils,
        debug_messenger: vk::DebugUtilsMessengerEXT,

        physical_device: vk::PhysicalDevice,
    }
}

impl Application {
    pub fn new() -> Self {
        lazy_construct! {
            Self {
                entry: unsafe { ash::Entry::new().unwrap() },

                event_loop,
                window,

                instance,

                debug_utils,
                debug_messenger: vk::DebugUtilsMessengerEXT::null(),

                physical_device: vk::PhysicalDevice::null(),
            }
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
                .unwrap(),
        );

        self.event_loop.init(utils::Once::new(event_loop));
    }

    fn init_vulkan(&mut self) {
        self.create_instance();
        self.setup_debug_messenger();
        self.pick_physical_device();
    }

    fn main_loop(mut self) {
        //TODO: panic이든 뭐든 무조건 종료(정리) 실행
        self.event_loop
            .consume()
            .run(move |event, _, control_flow| match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                        let _ = &self;
                    }
                    _ => {}
                },
                _ => {}
            });
    }

    fn create_instance(&mut self) {
        if ENABLE_VALIDATION_LAYERS && !self.check_validation_layer_support() {
            panic!("Validation layers requested, but not available!");
        }

        let application_name = CString::new("Hello Triangle").unwrap();
        let engine_name = CString::new("No Engine").unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&application_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_0);

        let extensions = self.get_required_extensions();
        let mut create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extensions);

        let mut debug_create_info: vk::DebugUtilsMessengerCreateInfoEXT;

        let raw_layer_names = VALIDATION_LAYERS
            .iter()
            .map(|layer_name| layer_name.as_ptr() as *const c_char)
            .collect::<Vec<_>>();
        if ENABLE_VALIDATION_LAYERS {
            debug_create_info = Self::populate_debug_messenger_create_info();
            create_info = create_info
                .enabled_layer_names(&raw_layer_names)
                .push_next(&mut debug_create_info);
        }

        unsafe {
            self.instance
                .init(self.entry.create_instance(&create_info, None).unwrap())
        };
    }

    fn populate_debug_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
        vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::all()
                    ^ vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
            .pfn_user_callback(Some(Self::debug_callback))
            .build()
    }

    fn setup_debug_messenger(&mut self) {
        if !ENABLE_VALIDATION_LAYERS {
            return;
        }

        self.debug_utils.init(ash::extensions::ext::DebugUtils::new(
            &self.entry,
            &self.instance,
        ));

        let create_info = Self::populate_debug_messenger_create_info();
        unsafe {
            self.debug_messenger = self
                .debug_utils
                .create_debug_utils_messenger(&create_info, None)
                .unwrap();
        }
    }

    fn pick_physical_device(&mut self) {
        let devices = unsafe { self.instance.enumerate_physical_devices() }.unwrap();

        if devices.is_empty() {
            panic!("failed to find GPUs with Vulkan support!");
        }

        for device in devices {
            if self.is_device_suitable(device) {
                self.physical_device = device;
                break;
            }
        }

        if self.physical_device == vk::PhysicalDevice::null() {
            panic!("failed to find a suitable GPU!");
        }
    }

    fn is_device_suitable(&self, device: vk::PhysicalDevice) -> bool {
        let indices = self.find_queue_families(device);

        indices.is_complete()
    }

    fn find_queue_families(&self, device: vk::PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices {
            graphics_family: None,
        };

        let queue_families = unsafe {
            self.instance
                .get_physical_device_queue_family_properties(device)
        };

        let mut i = 0u32;
        for queue_family in queue_families {
            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i);
            }

            if indices.is_complete() {
                break;
            }

            i += 1;
        }

        indices
    }

    fn get_required_extensions(&self) -> Vec<*const c_char> {
        let mut extensions = ash_window::enumerate_required_extensions(self.window.get())
            .unwrap()
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<_>>();

        if ENABLE_VALIDATION_LAYERS {
            extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());
        }

        extensions
    }

    fn check_validation_layer_support(&self) -> bool {
        let available_layers = self.entry.enumerate_instance_layer_properties().unwrap();

        for layer_name in VALIDATION_LAYERS {
            let mut layer_found = false;

            for layer_properties in available_layers.iter() {
                if unsafe { &*(layer_name as *const [u8] as *const [c_char]) }
                    == &layer_properties.layer_name[0..layer_name.len()]
                {
                    layer_found = true;
                    break;
                }
            }

            if !layer_found {
                return false;
            }
        }

        true
    }

    unsafe extern "system" fn debug_callback(
        _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _p_user_data: *mut c_void,
    ) -> vk::Bool32 {
        println!(
            "Validation layer: {:?}",
            CStr::from_ptr((*p_callback_data).p_message)
        );
        false as vk::Bool32
    }
}

impl Drop for Application {
    ///Using this same as `cleanup`
    fn drop(&mut self) {
        unsafe {
            println!("Dropping..");
            if ENABLE_VALIDATION_LAYERS {
                self.debug_utils
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
