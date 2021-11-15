use crate::*;

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;

use ash::vk;

//Window surface size.
const WINDOW_SIZE: winit::dpi::LogicalSize<u32> = winit::dpi::LogicalSize::new(800, 600);

const VALIDATION_LAYERS: [&[u8]; 1] = [b"VK_LAYER_KHRONOS_validation"];

const DEVICE_EXTENSIONS: [fn() -> &'static CStr; 1] = [ash::extensions::khr::Swapchain::name];

const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    fn graphics_family(&self) -> u32 {
        self.graphics_family.clone().unwrap()
    }

    fn present_family(&self) -> u32 {
        self.present_family.clone().unwrap()
    }
}

struct SwapChainSupportDetails {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
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
        -debug_utils_loader: ash::extensions::ext::DebugUtils,
        debug_messenger: vk::DebugUtilsMessengerEXT,
        -surface_loader: ash::extensions::khr::Surface,
        surface: vk::SurfaceKHR,

        physical_device: vk::PhysicalDevice,
        -device: ash::Device,

        graphics_queue: vk::Queue,
        present_queue: vk::Queue,

        //-swap_chain_loader::ash::extensions::khr::Swapchain
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
                debug_utils_loader,
                debug_messenger: vk::DebugUtilsMessengerEXT::null(),
                surface_loader,
                surface: vk::SurfaceKHR::null(),

                physical_device: vk::PhysicalDevice::null(),
                device,

                graphics_queue: vk::Queue::null(),
                present_queue: vk::Queue::null(),
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
                .with_inner_size(WINDOW_SIZE)
                .with_resizable(false)
                .build(&event_loop)
                .unwrap(),
        );

        self.event_loop.init(utils::Once::new(event_loop));
    }

    fn init_vulkan(&mut self) {
        self.create_instance();
        self.setup_debug_messenger();
        self.create_surface();
        self.pick_physical_device();
        self.create_logical_device();
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
                // vk::DebugUtilsMessageSeverityFlagsEXT::all()
                //     ^ vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                vk::DebugUtilsMessageSeverityFlagsEXT::all()
                    ^ vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
            )
            .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
            .pfn_user_callback(Some(Self::debug_callback))
            .build()
    }

    fn setup_debug_messenger(&mut self) {
        if !ENABLE_VALIDATION_LAYERS {
            return;
        }

        self.debug_utils_loader
            .init(ash::extensions::ext::DebugUtils::new(
                &self.entry,
                &self.instance,
            ));

        let create_info = Self::populate_debug_messenger_create_info();
        unsafe {
            self.debug_messenger = self
                .debug_utils_loader
                .create_debug_utils_messenger(&create_info, None)
                .unwrap();
        }
    }

    fn create_surface(&mut self) {
        self.surface_loader.init(ash::extensions::khr::Surface::new(
            &self.entry,
            &self.instance,
        ));
        self.surface = unsafe {
            ash_window::create_surface(&self.entry, &self.instance, self.window.get(), None)
                .expect("failed to create window surface!")
        };
    }

    fn pick_physical_device(&mut self) {
        let devices = unsafe { self.instance.enumerate_physical_devices().unwrap() };

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

    fn create_logical_device(&mut self) {
        let indices = self.find_queue_families(self.physical_device);

        let mut queue_create_infos = Vec::<vk::DeviceQueueCreateInfo>::new();
        let unique_queue_families = [indices.graphics_family(), indices.present_family()];

        let queue_priority = [1.0f32];
        let mut previous: Option<u32> = None;
        for queue_family in unique_queue_families {
            //tutorial 꺼 validation오류 발생으로 인한 수정. 만약 present, graphics queue familes 가 겹치면 스킵
            if previous.is_some() && previous == Some(queue_family) {
                continue;
            }
            let queue_create_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family)
                .queue_priorities(&queue_priority)
                .build();
            queue_create_infos.push(queue_create_info);
            previous = Some(queue_family);
        }

        let device_features: vk::PhysicalDeviceFeatures = Default::default();

        let device_extensions = DEVICE_EXTENSIONS
            .iter()
            .map(|extension| extension().as_ptr())
            .collect::<Vec<_>>();
        let mut create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(&device_extensions);

        let raw_layer_names = VALIDATION_LAYERS
            .iter()
            .map(|layer_name| layer_name.as_ptr() as *const c_char)
            .collect::<Vec<_>>();
        if ENABLE_VALIDATION_LAYERS {
            create_info = create_info.enabled_layer_names(&raw_layer_names);
        }

        self.device.init(unsafe {
            self.instance
                .create_device(self.physical_device, &create_info, None)
                .expect("failed to create logical device!")
        });

        self.graphics_queue = unsafe { self.device.get_device_queue(indices.graphics_family(), 0) };
        self.present_queue = unsafe { self.device.get_device_queue(indices.present_family(), 0) };
    }

    fn choose_swap_surface_format(
        available_formats: &Vec<vk::SurfaceFormatKHR>,
    ) -> vk::SurfaceFormatKHR {
        for available_format in available_formats {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return *available_format;
            }
        }
        available_formats[0]
    }

    fn choose_swap_present_mode(
        available_present_modes: &Vec<vk::PresentModeKHR>,
    ) -> vk::PresentModeKHR {
        for available_present_mode in available_present_modes {
            if *available_present_mode == vk::PresentModeKHR::MAILBOX {
                return *available_present_mode;
            }
        }
        return vk::PresentModeKHR::FIFO;
    }

    fn choose_swap_extent(&self, capabilities: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            return capabilities.current_extent;
        } else {
            //same as glfwGetFrameBufferSize
            let physical_size = WINDOW_SIZE.to_physical(self.window.scale_factor());

            let actual_extent = vk::Extent2D {
                width: physical_size.width,
                height: physical_size.height,
            };

            actual_extent.width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            actual_extent.height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            return actual_extent;
        }
    }

    fn query_swap_chain_support(&self, device: vk::PhysicalDevice) -> SwapChainSupportDetails {
        SwapChainSupportDetails {
            capabilities: unsafe {
                self.surface_loader
                    .get_physical_device_surface_capabilities(device, self.surface)
                    .unwrap()
            },
            formats: unsafe {
                self.surface_loader
                    .get_physical_device_surface_formats(device, self.surface)
                    .unwrap()
            },
            present_modes: unsafe {
                self.surface_loader
                    .get_physical_device_surface_present_modes(device, self.surface)
                    .unwrap()
            },
        }
    }

    fn is_device_suitable(&self, device: vk::PhysicalDevice) -> bool {
        let indices = self.find_queue_families(device);

        let extensions_supported = self.check_device_extension_support(device);

        let mut swap_chain_adequate = false;
        if extensions_supported {
            let swap_chain_support = self.query_swap_chain_support(device);
            swap_chain_adequate = !swap_chain_support.formats.is_empty()
                && !swap_chain_support.present_modes.is_empty();
        }

        indices.is_complete() && extensions_supported && swap_chain_adequate
    }

    fn check_device_extension_support(&self, device: vk::PhysicalDevice) -> bool {
        let available_extensions = unsafe {
            self.instance
                .enumerate_device_extension_properties(device)
                .unwrap()
        };

        let mut result = true;

        for required_extension in DEVICE_EXTENSIONS {
            let required_extension = required_extension();
            let mut temp = false;

            for extension in available_extensions.iter() {
                let extension_name =
                    unsafe { CStr::from_ptr(&extension.extension_name as *const c_char) };
                temp |= extension_name == required_extension;
            }
            result &= temp;
        }

        result
    }

    fn find_queue_families(&self, device: vk::PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
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

            if indices.graphics_family.is_some()
                && unsafe {
                    self.surface_loader
                        .get_physical_device_surface_support(device, i, self.surface)
                        .unwrap()
                }
            {
                indices.present_family = Some(i);
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
            self.device.destroy_device(None);
            if ENABLE_VALIDATION_LAYERS {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}
