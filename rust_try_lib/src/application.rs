use crate::*;

use std::ffi::{c_void, CStr, CString};
use std::io::Cursor;
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

        -swapchain_loader: ash::extensions::khr::Swapchain,
        swapchain: vk::SwapchainKHR,
        swapchain_images: Vec<vk::Image>,
        swapchain_image_format: vk::Format,
        swapchain_extent: vk::Extent2D,
        swapchain_image_views: Vec<vk::ImageView>,

        render_pass: vk::RenderPass,
        pipeline_layout: vk::PipelineLayout,
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

                swapchain_loader,
                swapchain: vk::SwapchainKHR::null(),
                swapchain_images: Vec::<vk::Image>::new(),
                swapchain_image_format: vk::Format::default(),
                swapchain_extent: vk::Extent2D::default(),
                swapchain_image_views: Vec::<vk::ImageView>::new(),

                render_pass: vk::RenderPass::null(),
                pipeline_layout: vk::PipelineLayout::null(),
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
        self.create_swapchain();
        self.create_image_views();
        self.create_render_pass();
        self.create_graphics_pipeline();
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

    fn create_swapchain(&mut self) {
        let swapchain_support = self.query_swapchain_support(self.physical_device);

        let surface_format = self.choose_swap_surface_format(&swapchain_support.formats);
        let preset_mode = self.choose_swap_present_mode(&swapchain_support.present_modes);
        let extent = self.choose_swap_extent(&swapchain_support.capabilities);

        let mut image_count = swapchain_support.capabilities.min_image_count + 1;
        if swapchain_support.capabilities.max_image_count > 0
            && image_count > swapchain_support.capabilities.max_image_count
        {
            image_count = swapchain_support.capabilities.max_image_count;
        }

        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

        let indices = self.find_queue_families(self.physical_device);
        let queue_family_indices = [indices.graphics_family(), indices.present_family()];

        if indices.graphics_family != indices.present_family {
            create_info = create_info
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_family_indices);
        } else {
            create_info = create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
        }

        create_info = create_info
            .pre_transform(swapchain_support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(preset_mode)
            .clipped(true);

        self.swapchain_loader
            .init(ash::extensions::khr::Swapchain::new(
                &self.instance,
                &self.device,
            ));
        self.swapchain = unsafe {
            self.swapchain_loader
                .create_swapchain(&create_info, None)
                .expect("failed to create swap chain!")
        };

        self.swapchain_images = unsafe {
            self.swapchain_loader
                .get_swapchain_images(self.swapchain)
                .unwrap()
        };

        self.swapchain_image_format = surface_format.format;
        self.swapchain_extent = extent;
    }

    fn create_image_views(&mut self) {
        for swapchain_image in self.swapchain_images.iter() {
            let create_info = vk::ImageViewCreateInfo::builder()
                .image(*swapchain_image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(self.swapchain_image_format)
                //.components(vk::ComponentMapping::default())
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            self.swapchain_image_views.push(unsafe {
                self.device
                    .create_image_view(&create_info, None)
                    .expect("failed to create image views!")
            })
        }
    }

    fn create_render_pass(&mut self) {
        let color_attachment = [vk::AttachmentDescription::builder()
            .format(self.swapchain_image_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .build()];

        let color_attachment_ref = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpass = [vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_ref)
            .build()];

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&color_attachment)
            .subpasses(&subpass);

        self.render_pass = unsafe {
            self.device
                .create_render_pass(&render_pass_info, None)
                .expect("failed to create render pass!")
        };
    }

    fn create_graphics_pipeline(&mut self) {
        let vert_shader_code = include_bytes!("../../assets/shaders/vert.spv");
        let frag_shader_code = include_bytes!("../../assets/shaders/frag.spv");

        let vert_shader_module = self.create_shader_module(vert_shader_code);
        let frag_shader_module = self.create_shader_module(frag_shader_code);

        let entry_point_name = CString::new("main").unwrap();
        let vert_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(&entry_point_name);

        let frag_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(&entry_point_name);

        let shader_stages = [vert_shader_stage_info, frag_shader_stage_info];

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        let viewport = [vk::Viewport::builder()
            .width(self.swapchain_extent.width as f32)
            .height(self.swapchain_extent.height as f32)
            .max_depth(1.0f32)
            .build()];

        let scissor = [vk::Rect2D::builder().extent(self.swapchain_extent).build()];

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewport)
            .scissors(&scissor);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0f32)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = [vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .build()];

        let color_blending =
            vk::PipelineColorBlendStateCreateInfo::builder().attachments(&color_blend_attachment);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder();

        self.pipeline_layout = unsafe {
            self.device
                .create_pipeline_layout(&pipeline_layout_info, None)
                .expect("failed to create pipeline layout!")
        };

        unsafe {
            self.device.destroy_shader_module(vert_shader_module, None);
            self.device.destroy_shader_module(frag_shader_module, None);
        }
    }

    fn create_shader_module(&self, code: &[u8]) -> vk::ShaderModule {
        let code = ash::util::read_spv(&mut Cursor::new(code)).unwrap();
        let create_info = vk::ShaderModuleCreateInfo::builder().code(&code);

        unsafe {
            self.device
                .create_shader_module(&create_info, None)
                .expect("failed to create shader module!")
        }
    }

    fn choose_swap_surface_format(
        &self,
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
        &self,
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

            let mut actual_extent = vk::Extent2D {
                width: physical_size.width,
                height: physical_size.height,
            };

            actual_extent.width = actual_extent.width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            actual_extent.height = actual_extent.height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            return actual_extent;
        }
    }

    fn query_swapchain_support(&self, device: vk::PhysicalDevice) -> SwapChainSupportDetails {
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

        let mut swapchain_adequate = false;
        if extensions_supported {
            let swapchain_support = self.query_swapchain_support(device);
            swapchain_adequate = !swapchain_support.formats.is_empty()
                && !swapchain_support.present_modes.is_empty();
        }

        indices.is_complete() && extensions_supported && swapchain_adequate
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
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);

            for image_view in self.swapchain_image_views.iter() {
                self.device.destroy_image_view(*image_view, None);
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
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
