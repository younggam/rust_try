use ash;
use ash::vk;
use ash_window;
use std::ffi::CString;
pub use winit;

const IMAGE_AVAILABLE_INDEX: usize = 0;
const RENDERING_DONE_INDEX: usize = 1;

pub struct RustTry {
    entry: ash::Entry,
    window: winit::window::Window,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    device: ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    command_buffer: Vec<vk::CommandBuffer>,
    semaphores: [vk::Semaphore; 2],
    fences: [vk::Fence; 2],
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
        let device = Self::create_device(&instance, &queue_family_index, &physical_device);
        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let command_pool = Self::create_command_pool(&device, &queue_family_index);
        let command_buffer = Self::allocate_command_buffer(&device, &command_pool);
        let semaphores = Self::create_semaphores(&device);
        let fences = Self::create_fences(&device);
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
            command_pool,
            command_buffer,
            semaphores,
            fences,
            surface_loader,
            surface: vk::SurfaceKHR::null(),
            swapchain_loader,
            swapchain: vk::SwapchainKHR::null(),
            swapchain_images: Vec::<vk::Image>::new(),
        }
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
        let instance_create_info = vk::InstanceCreateInfo::builder()
            .enabled_layer_names(&layer_names_raw)
            .enabled_extension_names(&instance_extension_names_raw)
            .build();
        //

        unsafe {
            entry
                .create_instance(&instance_create_info, None)
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
        queue_family_index: &u32,
        physical_device: &vk::PhysicalDevice,
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

    fn create_command_pool(device: &ash::Device, queue_family_index: &u32) -> vk::CommandPool {
        let command_pool_create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(*queue_family_index)
            .build();

        unsafe {
            device
                .create_command_pool(&command_pool_create_info, None)
                .expect("Command poll creation error")
        }
    }

    fn allocate_command_buffer(
        device: &ash::Device,
        command_pool: &vk::CommandPool,
    ) -> Vec<vk::CommandBuffer> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(*command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1)
            .build();

        unsafe {
            device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Command buffer allocation error")
        }
    }

    fn create_semaphores(device: &ash::Device) -> [vk::Semaphore; 2] {
        let mut semaphores = [vk::Semaphore::null(); 2];
        let semaphore_create_info = vk::SemaphoreCreateInfo::builder()
            .flags(vk::SemaphoreCreateFlags::empty())
            .build();

        for i in 0..2 {
            match unsafe { device.create_semaphore(&semaphore_create_info, None) } {
                Err(e) if e == vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
                    println!("VK_ERROR_OUR_OF_HOST_MEMORY");
                    break;
                }
                //
                Err(e) if e == vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                    println!("VK_ERROR_OUR_OF_DEVICE_MEMORY");
                    break;
                }
                //
                Ok(semaphore) => semaphores[i as usize] = semaphore,
                //
                _ => break,
            }
        }
        semaphores
    }

    fn create_fences(device: &ash::Device) -> [vk::Fence; 2] {
        let mut fences = [vk::Fence::null(); 2];
        let mut fence_create_info = vk::FenceCreateInfo::builder().build();

        for i in 0..2 {
            fence_create_info.flags = vk::FenceCreateFlags::empty();

            match unsafe { device.create_fence(&fence_create_info, None) } {
                Err(e) if e == vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
                    println!("VK_ERROR_OUR_OF_HOST_MEMORY");
                    break;
                }
                //
                Err(e) if e == vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                    println!("VK_ERROR_OUR_OF_DEVICE_MEMORY");
                    break;
                }
                //
                Ok(fence) => fences[i as usize] = fence,
                //
                _ => break,
            };
        }
        fences
    }

    fn create_surface(&mut self) {
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
        }
    }

    fn create_swapchain(&mut self) {
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
            .image_usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(composite_alpha)
            .present_mode(vk::PresentModeKHR::FIFO)
            .build();

        self.swapchain = unsafe {
            self.swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Swapchain creation error")
        };
    }

    pub fn init_swapchain_images(&mut self) {
        self.swapchain_images = unsafe {
            self.swapchain_loader
                .get_swapchain_images(self.swapchain)
                .expect("Getting swapchain images error")
        };

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        unsafe {
            self.device
                .begin_command_buffer(self.command_buffer[0], &command_buffer_begin_info)
                .expect("Command buffer begining error");
        }

        let mut image_memory_barriers = Vec::<vk::ImageMemoryBarrier>::new();
        self.swapchain_images.iter().for_each(|image| {
            let image_memory_barrier = vk::ImageMemoryBarrier::builder()
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .src_queue_family_index(self.queue_family_index)
                .dst_queue_family_index(self.queue_family_index)
                .image(*image)
                .subresource_range(
                    vk::ImageSubresourceRange::builder()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .level_count(1)
                        .layer_count(1)
                        .build(),
                )
                .build();

            image_memory_barriers.push(image_memory_barrier);
        });

        unsafe {
            self.device.cmd_pipeline_barrier(
                self.command_buffer[0],
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &image_memory_barriers,
            );
            self.device
                .end_command_buffer(self.command_buffer[0])
                .expect("Command buffer ending error");
        }

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&self.command_buffer[..1])
            .build();

        unsafe {
            self.device
                .queue_submit(
                    self.queue,
                    &[submit_info],
                    self.fences[RENDERING_DONE_INDEX],
                )
                .expect("Queue submitting error");
            self.device
                .device_wait_idle()
                .expect("Device waiting error");
        }
    }

    pub fn on_startup(&mut self) {
        self.create_surface();
        self.create_swapchain();
        self.init_swapchain_images();
    }

    pub fn on_render(&self) {
        let (swapchain_image_index, is_suboptimal) = unsafe {
            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    u64::MAX,
                    self.semaphores[IMAGE_AVAILABLE_INDEX],
                    self.fences[IMAGE_AVAILABLE_INDEX],
                )
                .expect("Next image acquiring error")
        };

        unsafe {
            self.device
                .wait_for_fences(&self.fences[..=IMAGE_AVAILABLE_INDEX], true, u64::MAX)
                .expect("Fences waiting error");
            self.device
                .reset_fences(&self.fences[..=IMAGE_AVAILABLE_INDEX])
                .expect("Fences reseting error");
            if !self
                .device
                .get_fence_status(self.fences[RENDERING_DONE_INDEX])
                .expect("Fence status getting error")
            {
                self.device
                    .wait_for_fences(&self.fences[RENDERING_DONE_INDEX..], true, u64::MAX)
                    .expect("Fences waiting error");
            }
            self.device
                .reset_fences(&self.fences[RENDERING_DONE_INDEX..])
                .expect("Fences reseting error");

            self.device
                .reset_command_buffer(self.command_buffer[0], vk::CommandBufferResetFlags::empty())
                .expect("Command buffer reseting error");
        }

        let swapchain_image = &self.swapchain_images[swapchain_image_index as usize];

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        unsafe {
            self.device
                .begin_command_buffer(self.command_buffer[0], &command_buffer_begin_info)
                .expect("Command buffer begining error");
        }

        let image_memory_barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_queue_family_index(self.queue_family_index)
            .dst_queue_family_index(self.queue_family_index)
            .image(*swapchain_image)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .build();

        unsafe {
            self.device.cmd_pipeline_barrier(
                self.command_buffer[0],
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        let clear_color = vk::ClearColorValue {
            //        R    G    B    A
            float32: [1.0, 0.0, 1.0, 1.0],
        };

        let image_subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1)
            .build();

        unsafe {
            self.device.cmd_clear_color_image(
                self.command_buffer[0],
                *swapchain_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &clear_color,
                &[image_subresource_range],
            );
        }

        let image_memory_barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .src_queue_family_index(self.queue_family_index)
            .dst_queue_family_index(self.queue_family_index)
            .image(*swapchain_image)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .build();

        unsafe {
            self.device.cmd_pipeline_barrier(
                self.command_buffer[0],
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
            self.device
                .end_command_buffer(self.command_buffer[0])
                .expect("Command buffer ending error");
        }

        let wait_dst_stage_mask = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&self.semaphores[..=IMAGE_AVAILABLE_INDEX])
            .wait_dst_stage_mask(&[wait_dst_stage_mask])
            .command_buffers(&self.command_buffer[..1])
            .signal_semaphores(&self.semaphores[RENDERING_DONE_INDEX..])
            .build();

        unsafe {
            self.device
                .queue_submit(
                    self.queue,
                    &[submit_info],
                    self.fences[RENDERING_DONE_INDEX],
                )
                .expect("Queue submitting error");
        }

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&self.semaphores[RENDERING_DONE_INDEX..])
            .swapchains(&[self.swapchain])
            .image_indices(&[swapchain_image_index])
            .build();

        unsafe {
            self.swapchain_loader
                .queue_present(self.queue, &present_info)
                .expect("Queue presenting error");
        }
    }

    pub fn on_shutdown(&mut self) {
        unsafe {
            self.device
                .device_wait_idle()
                .expect("Device waiting error");
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }

    fn print_instance_layer_properties(entry: &ash::Entry) {
        let layer_properties = entry
            .enumerate_instance_layer_properties()
            .expect("There is no available layer");
        layer_properties.iter().for_each(|x| println!("{:#?}", x));
        println!("\n");
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
        physical_device: &vk::PhysicalDevice,
        surface: &vk::SurfaceKHR,
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
}

impl Drop for RustTry {
    fn drop(&mut self) {
        unsafe {
            for semaphore in self.semaphores {
                self.device.destroy_semaphore(semaphore, None);
            }
            for fence in self.fences {
                self.device.destroy_fence(fence, None);
            }
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
