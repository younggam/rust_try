use crate::graphics::elements::Vertex;
use crate::math::vector::*;
use crate::*;

use std::ffi::{c_void, CStr, CString};
use std::io::Cursor;
use std::os::raw::c_char;

use ash::vk;

//Window surface size.
const WINDOW_SIZE: winit::dpi::LogicalSize<u32> = winit::dpi::LogicalSize::new(800, 600);

const MAX_FRAMES_IN_FLIGHT: usize = 2;

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

const VERTICES: [Vertex; 3] = [
    Vertex::new(Vec2::new(0.0, -0.5), Vec3::UNIT_X),
    Vertex::new(Vec2::new(0.5, 0.5), Vec3::UNIT_Y),
    Vertex::new(Vec2::new(-0.5, 0.5), Vec3::UNIT_Z),
];

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
        swapchain_framebuffers: Vec<vk::Framebuffer>,

        render_pass: vk::RenderPass,
        pipeline_layout: vk::PipelineLayout,
        graphics_pipeline: vk::Pipeline,

        command_pool: vk::CommandPool,

        vertex_buffer: vk::Buffer,
        vertex_buffer_memory: vk::DeviceMemory,

        command_buffers: Vec<vk::CommandBuffer>,

        image_available_semaphores: Vec<vk::Semaphore>,
        render_finished_semaphores: Vec<vk::Semaphore>,
        in_flight_fences: Vec<vk::Fence>,
        images_in_flight: Vec<vk::Fence>,
        current_frame: usize,

        framebuffer_resized: bool,
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
                swapchain_framebuffers: Vec::<vk::Framebuffer>::new(),

                render_pass: vk::RenderPass::null(),
                pipeline_layout: vk::PipelineLayout::null(),
                graphics_pipeline: vk::Pipeline::null(),

                command_pool: vk::CommandPool::null(),

                vertex_buffer: vk::Buffer::null(),
                vertex_buffer_memory: vk::DeviceMemory::null(),

                command_buffers: Vec::<vk::CommandBuffer>::new(),

                image_available_semaphores: Vec::<vk::Semaphore>::new(),
                render_finished_semaphores: Vec::<vk::Semaphore>::new(),
                in_flight_fences: Vec::<vk::Fence>::new(),
                images_in_flight: Vec::<vk::Fence>::new(),
                current_frame: 0,

                framebuffer_resized: false,
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
                .with_resizable(true)
                .build(&event_loop)
                .unwrap(),
        );

        self.event_loop.init(utils::Once::new(event_loop));
    }

    pub fn framebuffer_resize_callback(&mut self) {
        self.framebuffer_resized = true;
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
        self.create_framebuffers();
        self.create_command_pool();
        self.create_vertex_buffer();
        self.create_command_buffers();
        self.create_sync_objects();
    }

    fn main_loop(mut self) {
        //TODO: panic이든 뭐든 무조건 종료(정리) 실행
        self.event_loop
            .consume()
            .run(move |event, _, control_flow| {
                //소유권 가져오기
                //let _ = &self;

                match event {
                    winit::event::Event::MainEventsCleared => self.draw_frame(),
                    winit::event::Event::WindowEvent { event, .. } => match event {
                        winit::event::WindowEvent::Resized(_) => self.framebuffer_resize_callback(),
                        winit::event::WindowEvent::CloseRequested => {
                            *control_flow = winit::event_loop::ControlFlow::Exit;
                        }
                        _ => {}
                    },
                    winit::event::Event::LoopDestroyed => {
                        unsafe {
                            self.device.device_wait_idle().unwrap();
                        }
                        println!("shut down!");
                    }
                    _ => {}
                }
            });
    }

    fn cleanup_swapchain(&mut self) {
        unsafe {
            for framebuffer in self.swapchain_framebuffers.drain(..) {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device
                .free_command_buffers(self.command_pool, &self.command_buffers);

            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);

            for swapchain_image_view in self.swapchain_image_views.drain(..) {
                self.device.destroy_image_view(swapchain_image_view, None);
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }

    fn recreate_swapchain(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
        }

        self.cleanup_swapchain();

        self.create_swapchain();
        self.create_image_views();
        self.create_render_pass();
        self.create_graphics_pipeline();
        self.create_framebuffers();
        self.create_command_buffers();

        self.framebuffer_resized = false;
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

        let device_features = vk::PhysicalDeviceFeatures::builder();

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

        let dependency = [vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .build()];

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&color_attachment)
            .subpasses(&subpass)
            .dependencies(&dependency);

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

        let shader_stages = [*vert_shader_stage_info, *frag_shader_stage_info];

        let mut vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();

        let binding_description = [Vertex::get_binding_description()];
        let attribute_descriptions = Vertex::get_attribute_descriptions();

        vertex_input_info = vertex_input_info
            .vertex_binding_descriptions(&binding_description)
            .vertex_attribute_descriptions(&attribute_descriptions);

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

        let pipeline_info = [vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .layout(self.pipeline_layout)
            .render_pass(self.render_pass)
            .subpass(0)
            .build()];

        self.graphics_pipeline = unsafe {
            self.device
                .create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_info, None)
                .expect("failed to create graphics pipeline!")[0]
        };

        unsafe {
            self.device.destroy_shader_module(vert_shader_module, None);
            self.device.destroy_shader_module(frag_shader_module, None);
        }
    }

    fn create_framebuffers(&mut self) {
        for swapchain_image_view in self.swapchain_image_views.iter() {
            let swapchain_image_view = [*swapchain_image_view];

            let framebuffer_info = vk::FramebufferCreateInfo::builder()
                .render_pass(self.render_pass)
                .attachments(&swapchain_image_view)
                .width(self.swapchain_extent.width)
                .height(self.swapchain_extent.height)
                .layers(1);

            self.swapchain_framebuffers.push(unsafe {
                self.device
                    .create_framebuffer(&framebuffer_info, None)
                    .expect("failed to create framebuffer!")
            });
        }
    }

    fn create_command_pool(&mut self) {
        let queue_family_indices = self.find_queue_families(self.physical_device);

        let pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_indices.graphics_family());

        self.command_pool = unsafe {
            self.device
                .create_command_pool(&pool_info, None)
                .expect("failed to create command pool!")
        };
    }

    fn create_vertex_buffer(&mut self) {
        let buffer_size = std::mem::size_of_val(&VERTICES[0]) as u64 * 3;

        let (staging_buffer, staging_buffer_memory) = self.create_buffer(
            buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        unsafe {
            let data = self
                .device
                .map_memory(
                    staging_buffer_memory,
                    0,
                    buffer_size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap();
            std::ptr::copy_nonoverlapping(
                &VERTICES[0] as *const Vertex,
                data as *mut Vertex,
                VERTICES.len(),
            );
            self.device.unmap_memory(staging_buffer_memory);
        }

        let (buffer, buffer_memory) = self.create_buffer(
            buffer_size,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        self.vertex_buffer = buffer;
        self.vertex_buffer_memory = buffer_memory;

        self.copy_buffer(staging_buffer, self.vertex_buffer, buffer_size);

        unsafe {
            self.device.destroy_buffer(staging_buffer, None);
            self.device.free_memory(staging_buffer_memory, None);
        }
    }

    fn create_buffer(
        &self,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe {
            self.device
                .create_buffer(&buffer_info, None)
                .expect("failed to create buffer!")
        };

        let mem_requirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_requirements.size)
            .memory_type_index(
                self.find_memory_type(mem_requirements.memory_type_bits, properties),
            );

        let buffer_memory = unsafe {
            self.device
                .allocate_memory(&alloc_info, None)
                .expect("failed to allocate buffer memory!")
        };

        unsafe {
            self.device
                .bind_buffer_memory(buffer, buffer_memory, 0)
                .unwrap();
        }

        (buffer, buffer_memory)
    }

    fn copy_buffer(&self, src_buffer: vk::Buffer, dst_buffer: vk::Buffer, size: vk::DeviceSize) {
        let alloc_info = vk::CommandBufferAllocateInfo::builder()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(self.command_pool)
            .command_buffer_count(1);

        let command_buffer = unsafe { self.device.allocate_command_buffers(&alloc_info).unwrap() };

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device
                .begin_command_buffer(command_buffer[0], &begin_info)
                .unwrap();
        }

        let copy_region = vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size,
        };
        unsafe {
            self.device
                .cmd_copy_buffer(command_buffer[0], src_buffer, dst_buffer, &[copy_region]);

            self.device.end_command_buffer(command_buffer[0]).unwrap();
        }

        let submit_info = vk::SubmitInfo::builder().command_buffers(&command_buffer);

        unsafe {
            self.device
                .queue_submit(self.graphics_queue, &[*submit_info], vk::Fence::null())
                .unwrap();
            self.device.queue_wait_idle(self.graphics_queue).unwrap();

            self.device
                .free_command_buffers(self.command_pool, &command_buffer);
        }
    }

    fn find_memory_type(&self, type_filter: u32, properties: vk::MemoryPropertyFlags) -> u32 {
        let mem_properties = unsafe {
            self.instance
                .get_physical_device_memory_properties(self.physical_device)
        };

        for i in 0..mem_properties.memory_type_count {
            if (type_filter & (1 << i)) > 0
                && (mem_properties.memory_types[i as usize]
                    .property_flags
                    .contains(properties))
            {
                return i;
            }
        }

        panic!("failed to find suitable memory type!");
    }

    fn create_command_buffers(&mut self) {
        let alloc_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(self.swapchain_framebuffers.len() as u32);

        self.command_buffers = unsafe {
            self.device
                .allocate_command_buffers(&alloc_info)
                .expect("failed to allocate command buffers!")
        };

        for i in 0..self.command_buffers.len() {
            let begin_info = vk::CommandBufferBeginInfo::builder();

            unsafe {
                self.device
                    .begin_command_buffer(self.command_buffers[i], &begin_info)
                    .expect("failed to begin recording command buffer!");
            }

            let mut render_pass_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass)
                .framebuffer(self.swapchain_framebuffers[i])
                .render_area(vk::Rect2D {
                    extent: self.swapchain_extent,
                    ..Default::default()
                });

            let clear_color = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];
            render_pass_info = render_pass_info.clear_values(&clear_color);

            unsafe {
                self.device.cmd_begin_render_pass(
                    self.command_buffers[i],
                    &render_pass_info,
                    vk::SubpassContents::INLINE,
                );

                self.device.cmd_bind_pipeline(
                    self.command_buffers[i],
                    vk::PipelineBindPoint::GRAPHICS,
                    self.graphics_pipeline,
                );

                let vertex_buffers = [self.vertex_buffer];
                let offsets: [vk::DeviceSize; 1] = [0];
                self.device.cmd_bind_vertex_buffers(
                    self.command_buffers[i],
                    0,
                    &vertex_buffers,
                    &offsets,
                );

                self.device
                    .cmd_draw(self.command_buffers[i], VERTICES.len() as u32, 1, 0, 0);

                self.device.cmd_end_render_pass(self.command_buffers[i]);

                self.device
                    .end_command_buffer(self.command_buffers[i])
                    .expect("failed to record command buffer!");
            }
        }
    }

    fn create_sync_objects(&mut self) {
        self.images_in_flight
            .resize(self.swapchain_images.len(), vk::Fence::null());

        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            //and_then chain for one expect
            unsafe { self.device.create_semaphore(&semaphore_info, None) }
                .and_then(|x| {
                    self.image_available_semaphores.push(x);
                    unsafe { self.device.create_semaphore(&semaphore_info, None) }
                })
                .and_then(|x| {
                    self.render_finished_semaphores.push(x);
                    unsafe { self.device.create_fence(&fence_info, None) }
                })
                .and_then(|x| {
                    self.in_flight_fences.push(x);
                    Ok(vk::Fence::null())
                })
                .expect("failed to create synchronization objects for a frame!");
        }
    }

    fn draw_frame(&mut self) {
        let framebuffer_size = self.window.inner_size();
        if framebuffer_size.width == 0 || framebuffer_size.height == 0 {
            return;
        }

        unsafe {
            self.device
                .wait_for_fences(&[self.in_flight_fences[self.current_frame]], true, u64::MAX)
                .unwrap();
        }

        let image_index = match unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available_semaphores[self.current_frame],
                vk::Fence::null(),
            )
        } {
            Ok((image_index, _)) => image_index as usize,
            Err(e) => match e {
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    self.recreate_swapchain();
                    return;
                }
                _ => panic!("failed to acquire swap chain image! : {:#?}", e),
            },
        };

        if self.images_in_flight[image_index] != vk::Fence::null() {
            unsafe {
                self.device
                    .wait_for_fences(&[self.images_in_flight[image_index]], true, u64::MAX)
                    .unwrap();
            }
        }
        self.images_in_flight[image_index] = self.in_flight_fences[self.current_frame];

        let mut submit_info = vk::SubmitInfo::builder();

        let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        submit_info = submit_info
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&self.command_buffers[image_index..=image_index]);

        let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];
        let submit_info = submit_info.signal_semaphores(&signal_semaphores);

        unsafe {
            self.device
                .reset_fences(&[self.in_flight_fences[self.current_frame]])
                .unwrap();

            self.device
                .queue_submit(
                    self.graphics_queue,
                    &[*submit_info],
                    self.in_flight_fences[self.current_frame],
                )
                .expect("failed to submit draw commmand buffer!");
        }

        let mut present_info = vk::PresentInfoKHR::builder().wait_semaphores(&signal_semaphores);

        let swapchains = [self.swapchain];
        let image_indices = [image_index as u32];
        present_info = present_info
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let mut error = false;
        match unsafe {
            self.swapchain_loader
                .queue_present(self.graphics_queue, &present_info)
        } {
            Ok(sub_optimal) => error |= sub_optimal,
            Err(e) => match e {
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    error = true;
                }
                _ => panic!("failed to acquire swap chain image! : {:#?}", e),
            },
        };
        if error || self.framebuffer_resized {
            self.recreate_swapchain();
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
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
            let physical_size = self.window.inner_size();

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
            "Validation layer: {:?}\n",
            CStr::from_ptr((*p_callback_data).p_message)
        );
        false as vk::Bool32
    }
}

impl Drop for Application {
    ///Using this same as `cleanup`
    fn drop(&mut self) {
        println!("Dropping..");
        self.cleanup_swapchain();

        unsafe {
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);

            for _ in 0..MAX_FRAMES_IN_FLIGHT {
                self.device
                    .destroy_semaphore(self.render_finished_semaphores.swap_remove(0), None);
                self.device
                    .destroy_semaphore(self.image_available_semaphores.swap_remove(0), None);
                self.device
                    .destroy_fence(self.in_flight_fences.swap_remove(0), None);
            }

            self.device.destroy_command_pool(self.command_pool, None);

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
