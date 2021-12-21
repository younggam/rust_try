use ash;
use ash::vk;
use ash_window;
use std::ffi::CString;
pub use winit;

pub struct RustTry {
    entry: ash::Entry,
    window: winit::window::Window,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    device: ash::Device,
    queue: vk::Queue,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
}

impl RustTry {
    pub fn new(window: winit::window::Window) -> Self {
        let entry = unsafe { ash::Entry::new().expect("Vulkan functions loading error") };
        let instance = Self::create_instance(&entry);
        let physical_device = Self::create_physical_device(&instance);
        let queue_family_index = Self::find_queue_family_index(&instance, &physical_device);
        let device = Self::create_device(&instance, &physical_device, &queue_family_index);
        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);

        RustTry {
            entry,
            window,
            instance,
            physical_device,
            queue_family_index,
            device,
            queue,
            surface_loader,
            surface: vk::SurfaceKHR::null(),
            swapchain_loader,
            swapchain: vk::SwapchainKHR::null(),
            swapchain_images: Vec::<vk::Image>::new(),
        }
    }

    fn print_instance_layer_properties(entry: &ash::Entry) {
        let layer_properties = entry
            .enumerate_instance_layer_properties()
            .expect("There is no available layer");
        layer_properties.iter().for_each(|x| println!("{:#?}", x));
        println!("\n");
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        //
        let layer_names = [CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        let layer_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let instance_extension_names = [
            CString::new("VK_KHR_surface").unwrap(),
            CString::new("VK_KHR_win32_surface").unwrap(),
            //CString::new("VK_MVK_macos_surface").unwrap(), not exist
        ];
        let instance_extension_names_raw: Vec<*const i8> = instance_extension_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        //

        //
        let create_info = vk::InstanceCreateInfo::builder()
            .enabled_layer_names(&layer_names_raw)
            .enabled_extension_names(&instance_extension_names_raw)
            .build();
        //

        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        }
    }

    fn create_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Physical device error")
        };

        physical_devices.iter().for_each(|physical_device| {
            let properties = unsafe { instance.get_physical_device_properties(*physical_device) };
            println!(
                "{}",
                String::from_utf8(unsafe { std::mem::transmute(properties.device_name.to_vec()) })
                    .unwrap()
            );
        });

        physical_devices[0]
    }

    fn find_queue_family_index(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
    ) -> u32 {
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };

        let mut result = usize::MAX;

        for (i, property) in queue_family_properties.iter().enumerate() {
            if property.queue_count > 0 && property.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                result = i;
                break;
            }
        }
        result as u32
    }

    fn create_device(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        queue_family_index: &u32,
    ) -> ash::Device {
        let priority: f32 = 1.0;

        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(*queue_family_index)
            .queue_priorities(&[priority])
            .build();

        let device_extension_names = [
            CString::new("VK_KHR_swapchain").unwrap(),
            //CString::new("VK_MVK_macos_surface").unwrap(), not exist
        ];
        let device_extension_names_raw: Vec<*const i8> = device_extension_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&[queue_create_info])
            .enabled_extension_names(&device_extension_names_raw)
            .build();

        unsafe {
            instance
                .create_device(*physical_device, &device_create_info, None)
                .expect("Device creation error")
        }
    }

    fn print_instance_extensions_properties(entry: &ash::Entry) {
        let instance_extension_properties = entry
            .enumerate_instance_extension_properties()
            .expect("There is no available extension");
        instance_extension_properties
            .iter()
            .for_each(|x| println!("{:#?}", x));
        println!("\n");
    }

    fn print_device_extensions_properties(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
    ) {
        let device_extension_properties = unsafe {
            instance
                .enumerate_device_extension_properties(*physical_device)
                .expect("There is no available extension")
        };
        device_extension_properties
            .iter()
            .for_each(|x| println!("{:#?}", x));
    }

    fn print_present_modes(
        surface_loader: &ash::extensions::khr::Surface,
        surface: &vk::SurfaceKHR,
        physical_device: &vk::PhysicalDevice,
    ) {
        let present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(*physical_device, *surface)
                .expect("There is no available present modes")
        };
        present_modes
            .iter()
            .for_each(|mode| println!("{:#?}", mode));
    }

    fn create_swapchain(&self) -> vk::SwapchainKHR {
        let surface_formats = unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(self.physical_device, self.surface)
                .expect("There is no available surface format")
        };
        /*surface_formats
        .iter()
        .for_each(|mode| println!("{:#?}", mode));*/
        let surface_format = surface_formats[0];
        let surface_capabilities = unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(self.physical_device, self.surface)
                .expect("Getting surface capabilities error")
        };

        let mut composite_alpha = vk::CompositeAlphaFlagsKHR::empty();
        for i in 0..4 {
            let flag = vk::CompositeAlphaFlagsKHR::from_raw(0b1 << i);
            if surface_capabilities
                .supported_composite_alpha
                .contains(flag)
            {
                composite_alpha = flag;
                break;
            }
        }

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface)
            .min_image_count(2)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(surface_capabilities.current_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(composite_alpha)
            .present_mode(vk::PresentModeKHR::FIFO)
            .build();

        unsafe {
            self.swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Swapchain creation error")
        }
    }

    pub fn on_startup(&mut self) {
        unsafe {
            self.surface =
                ash_window::create_surface(&self.entry, &self.instance, &self.window, None)
                    .expect("Surface creation error");

            let supported = self
                .surface_loader
                .get_physical_device_surface_support(
                    self.physical_device,
                    self.queue_family_index,
                    self.surface,
                )
                .expect("Surface support is invalid");
            assert!(supported, "Suface is not supported");

            self.swapchain = self.create_swapchain();
            self.swapchain_images = self
                .swapchain_loader
                .get_swapchain_images(self.swapchain)
                .expect("Getting swapchain images error");
        }
    }

    pub fn on_shutdown(&mut self) {
        unsafe {
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}

impl Drop for RustTry {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
