use crate::graphics::elements::Vertex;
use crate::math::*;
use crate::*;

use std::ffi::{c_void, CStr, CString};
use std::io::Cursor;
use std::os::raw::c_char;

use ash::vk;

//Window surface size.
const WINDOW_SIZE: winit::dpi::LogicalSize<u32> = winit::dpi::LogicalSize::new(800, 600);

const MODEL_PATH: &str = "assets/models/viking_room.obj";
const TEXTURE_PATH: &str = "assets/textures/viking_room.png";

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

#[derive(Clone, Default, Copy)]
#[repr(C)]
pub struct UniformBufferObject {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

// const VERTICES: [Vertex; 8] = [
//     Vertex::new(
//         Vec3::new(-0.5, -0.5, 0.0),
//         Vec3::new(1.0, 0.0, 0.0),
//         Vec2::new(1.0, 0.0),
//     ),
//     Vertex::new(
//         Vec3::new(0.5, -0.5, 0.0),
//         Vec3::new(0.0, 1.0, 0.0),
//         Vec2::new(0.0, 0.0),
//     ),
//     Vertex::new(
//         Vec3::new(0.5, 0.5, 0.0),
//         Vec3::new(0.0, 0.0, 1.0),
//         Vec2::new(0.0, 1.0),
//     ),
//     Vertex::new(
//         Vec3::new(-0.5, 0.5, 0.0),
//         Vec3::new(1.0, 1.0, 1.0),
//         Vec2::new(1.0, 1.0),
//     ),
//     //
//     Vertex::new(
//         Vec3::new(-0.5, -0.5, -0.5),
//         Vec3::new(1.0, 0.0, 0.0),
//         Vec2::new(1.0, 0.0),
//     ),
//     Vertex::new(
//         Vec3::new(0.5, -0.5, -0.5),
//         Vec3::new(0.0, 1.0, 0.0),
//         Vec2::new(0.0, 0.0),
//     ),
//     Vertex::new(
//         Vec3::new(0.5, 0.5, -0.5),
//         Vec3::new(0.0, 0.0, 1.0),
//         Vec2::new(0.0, 1.0),
//     ),
//     Vertex::new(
//         Vec3::new(-0.5, 0.5, -0.5),
//         Vec3::new(1.0, 1.0, 1.0),
//         Vec2::new(1.0, 1.0),
//     ),
// ];
//
// const INDICES: [u16; 12] = [
//     0, 1, 2, 2, 3, 0, //
//     4, 5, 6, 6, 7, 4,
// ];

lazy_struct! {
/**Temporary struct(possibly permanent) that manages whole application.

Following Vulkan Tutorial.*/
    //2021.11.07
    pub struct Application {
        entry: ash::Entry,
        start_time: std::time::Instant,

        -event_loop: utils::Once<winit::event_loop::EventLoop<()>>,
        -window: winit::window::Window,

        -instance: ash::Instance,
        -debug_utils_loader: ash::extensions::ext::DebugUtils,
        debug_messenger: vk::DebugUtilsMessengerEXT,
        -surface_loader: ash::extensions::khr::Surface,
        surface: vk::SurfaceKHR,

        physical_device: vk::PhysicalDevice,
        msaa_samples: vk::SampleCountFlags,
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
        descriptor_set_layout: vk::DescriptorSetLayout,
        pipeline_layout: vk::PipelineLayout,
        graphics_pipeline: vk::Pipeline,

        command_pool: vk::CommandPool,

        color_image: vk::Image,
        color_image_memory: vk::DeviceMemory,
        color_image_view: vk::ImageView,

        depth_image: vk::Image,
        depth_image_memory: vk::DeviceMemory,
        depth_image_view: vk::ImageView,

        mip_levels: u32,
        texture_image: vk::Image,
        texture_image_memory: vk::DeviceMemory,
        texture_image_view: vk::ImageView,
        texture_sampler: vk::Sampler,

        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        vertex_buffer: vk::Buffer,
        vertex_buffer_memory: vk::DeviceMemory,
        index_buffer: vk::Buffer,
        index_buffer_memory: vk::DeviceMemory,

        uniform_buffers: Vec<vk::Buffer>,
        uniform_buffers_memory: Vec<vk::DeviceMemory>,

        descriptor_pool: vk::DescriptorPool,
        descriptor_sets: Vec<vk::DescriptorSet>,

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
                start_time: std::time::Instant::now(),

                event_loop,
                window,

                instance,
                debug_utils_loader,
                debug_messenger: vk::DebugUtilsMessengerEXT::null(),
                surface_loader,
                surface: vk::SurfaceKHR::null(),

                physical_device: vk::PhysicalDevice::null(),
                msaa_samples: vk::SampleCountFlags::TYPE_1,
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
                descriptor_set_layout: vk::DescriptorSetLayout::null(),
                pipeline_layout: vk::PipelineLayout::null(),
                graphics_pipeline: vk::Pipeline::null(),

                command_pool: vk::CommandPool::null(),

                color_image: vk::Image::null(),
                color_image_memory: vk::DeviceMemory::null(),
                color_image_view: vk::ImageView::null(),

                depth_image: vk::Image::null(),
                depth_image_memory: vk::DeviceMemory::null(),
                depth_image_view: vk::ImageView::null(),

                mip_levels: 0,
                texture_image: vk::Image::null(),
                texture_image_memory: vk::DeviceMemory::null(),
                texture_image_view: vk::ImageView::null(),
                texture_sampler: vk::Sampler::null(),

                vertices: Vec::<Vertex>::new(),
                indices: Vec::<u32>::new(),
                vertex_buffer: vk::Buffer::null(),
                vertex_buffer_memory: vk::DeviceMemory::null(),
                index_buffer: vk::Buffer::null(),
                index_buffer_memory: vk::DeviceMemory::null(),

                uniform_buffers:Vec::<vk::Buffer>::new(),
                uniform_buffers_memory:Vec::<vk::DeviceMemory>::new(),

                descriptor_pool: vk::DescriptorPool::null(),
                descriptor_sets: Vec::<vk::DescriptorSet>::new(),

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
        self.create_descriptor_set_layout();
        self.create_graphics_pipeline();
        self.create_command_pool();
        self.create_color_resources();
        self.create_depth_resources();
        self.create_framebuffers();
        self.create_texture_image();
        self.create_texture_image_view();
        self.create_texture_sampler();
        self.load_model();
        self.create_vertex_buffer();
        self.create_index_buffer();
        self.create_uniform_buffers();
        self.create_descriptor_pool();
        self.create_descriptor_sets();
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
            self.device.destroy_image_view(self.color_image_view, None);
            self.device.destroy_image(self.color_image, None);
            self.device.free_memory(self.color_image_memory, None);

            self.device.destroy_image_view(self.depth_image_view, None);
            self.device.destroy_image(self.depth_image, None);
            self.device.free_memory(self.depth_image_memory, None);

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

            for uniform_buffer in self.uniform_buffers.drain(..) {
                self.device.destroy_buffer(uniform_buffer, None);
            }
            for uniform_buffer_memory in self.uniform_buffers_memory.drain(..) {
                self.device.free_memory(uniform_buffer_memory, None);
            }

            self.device
                .destroy_descriptor_pool(self.descriptor_pool, None);
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
        self.create_color_resources();
        self.create_depth_resources();
        self.create_framebuffers();
        self.create_uniform_buffers();
        self.create_descriptor_pool();
        self.create_descriptor_sets();
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
                self.msaa_samples = self.get_max_usable_sample_count();
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

        let device_features = vk::PhysicalDeviceFeatures::builder().sampler_anisotropy(true);

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
            self.swapchain_image_views.push(self.create_image_view(
                *swapchain_image,
                self.swapchain_image_format,
                vk::ImageAspectFlags::COLOR,
                1,
            ));
        }
    }

    fn create_render_pass(&mut self) {
        let color_attachment = vk::AttachmentDescription::builder()
            .format(self.swapchain_image_format)
            .samples(self.msaa_samples)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let depth_attachment = vk::AttachmentDescription::builder()
            .format(self.find_depth_format())
            .samples(self.msaa_samples)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let color_attachment_resolve = vk::AttachmentDescription::builder()
            .format(self.swapchain_image_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::DONT_CARE)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let color_attachment_resolve_ref = [vk::AttachmentReference {
            attachment: 2,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpass = [vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_ref)
            .depth_stencil_attachment(&depth_attachment_ref)
            .resolve_attachments(&color_attachment_resolve_ref)
            .build()];

        let dependency = [vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            )
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                    | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            )
            .dst_access_mask(
                vk::AccessFlags::COLOR_ATTACHMENT_WRITE
                    | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            )
            .build()];

        let attachments = [
            *color_attachment,
            *depth_attachment,
            *color_attachment_resolve,
        ];
        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpass)
            .dependencies(&dependency);

        self.render_pass = unsafe {
            self.device
                .create_render_pass(&render_pass_info, None)
                .expect("failed to create render pass!")
        };
    }

    fn create_descriptor_set_layout(&mut self) {
        let ubo_layout_binding = vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: std::ptr::null(),
        };

        let sampler_layout_binding = vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            p_immutable_samplers: std::ptr::null(),
            stage_flags: vk::ShaderStageFlags::FRAGMENT,
        };

        let bindings = [ubo_layout_binding, sampler_layout_binding];
        let layout_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);

        self.descriptor_set_layout = unsafe {
            self.device
                .create_descriptor_set_layout(&layout_info, None)
                .expect("failed to create descriptor set layout!")
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
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(self.msaa_samples);

        let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false);

        let color_blend_attachment = [vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .build()];

        let color_blending =
            vk::PipelineColorBlendStateCreateInfo::builder().attachments(&color_blend_attachment);

        let descriptor_set_layout = [self.descriptor_set_layout];
        let pipeline_layout_info =
            vk::PipelineLayoutCreateInfo::builder().set_layouts(&descriptor_set_layout);

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
            .depth_stencil_state(&depth_stencil)
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
            let attachments = [
                self.color_image_view,
                self.depth_image_view,
                *swapchain_image_view,
            ];

            let framebuffer_info = vk::FramebufferCreateInfo::builder()
                .render_pass(self.render_pass)
                .attachments(&attachments)
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

    fn create_color_resources(&mut self) {
        let color_format = self.swapchain_image_format;

        let (image, image_memory) = self.create_image(
            self.swapchain_extent.width,
            self.swapchain_extent.height,
            1,
            self.msaa_samples,
            color_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::TRANSIENT_ATTACHMENT | vk::ImageUsageFlags::COLOR_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        self.color_image = image;
        self.color_image_memory = image_memory;
        self.color_image_view = self.create_image_view(
            self.color_image,
            color_format,
            vk::ImageAspectFlags::COLOR,
            1,
        );
    }

    fn create_depth_resources(&mut self) {
        let depth_format = self.find_depth_format();

        let (image, image_memory) = self.create_image(
            self.swapchain_extent.width,
            self.swapchain_extent.height,
            1,
            self.msaa_samples,
            depth_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        self.depth_image = image;
        self.depth_image_memory = image_memory;
        self.depth_image_view = self.create_image_view(
            self.depth_image,
            depth_format,
            vk::ImageAspectFlags::DEPTH,
            1,
        );
    }

    fn find_supported_format(
        &self,
        candidates: &Vec<vk::Format>,
        tiling: vk::ImageTiling,
        features: vk::FormatFeatureFlags,
    ) -> vk::Format {
        for format in candidates {
            let props = unsafe {
                self.instance
                    .get_physical_device_format_properties(self.physical_device, *format)
            };

            if tiling == vk::ImageTiling::LINEAR
                && (props.linear_tiling_features & features) == features
            {
                return *format;
            } else if tiling == vk::ImageTiling::OPTIMAL
                && (props.optimal_tiling_features & features) == features
            {
                return *format;
            }
        }

        panic!("failed to find supported format!");
    }

    fn find_depth_format(&self) -> vk::Format {
        self.find_supported_format(
            &vec![
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        )
    }

    fn create_texture_image(&mut self) {
        let pixels = image::open(TEXTURE_PATH)
            .expect("failed to load texture image!")
            .into_rgba8();
        let (tex_width, tex_height) = pixels.dimensions();
        let image_size = pixels.len();
        self.mip_levels = (tex_width.max(tex_height) as f32).log2().floor() as u32 + 1;

        let (staging_buffer, staging_buffer_memory) = self.create_buffer(
            image_size as vk::DeviceSize,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        unsafe {
            let data = self
                .device
                .map_memory(
                    staging_buffer_memory,
                    0,
                    image_size as vk::DeviceSize,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap();
            std::ptr::copy_nonoverlapping(pixels.as_raw().as_ptr(), data as *mut u8, image_size);
            self.device.unmap_memory(staging_buffer_memory);
        }

        drop(pixels);

        let (image, image_memory) = self.create_image(
            tex_width,
            tex_height,
            self.mip_levels,
            vk::SampleCountFlags::TYPE_1,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::TRANSFER_SRC
                | vk::ImageUsageFlags::TRANSFER_DST
                | vk::ImageUsageFlags::SAMPLED,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        self.texture_image = image;
        self.texture_image_memory = image_memory;

        self.transition_image_layout(
            self.texture_image,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            self.mip_levels,
        );
        self.copy_buffer_to_image(staging_buffer, self.texture_image, tex_width, tex_height);
        // self.transition_image_layout(
        //     self.texture_image,
        //     vk::Format::R8G8B8A8_SRGB,
        //     vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        //     vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        // );

        unsafe {
            self.device.destroy_buffer(staging_buffer, None);
            self.device.free_memory(staging_buffer_memory, None);
        }

        self.generate_mipmaps(
            self.texture_image,
            vk::Format::R8G8B8A8_SRGB,
            tex_width,
            tex_height,
            self.mip_levels,
        );
    }

    fn generate_mipmaps(
        &self,
        image: vk::Image,
        image_format: vk::Format,
        tex_width: u32,
        tex_height: u32,
        mip_levels: u32,
    ) {
        let format_properties = unsafe {
            self.instance
                .get_physical_device_format_properties(self.physical_device, image_format)
        };

        if !format_properties
            .linear_tiling_features
            .contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR)
        {
            panic!("texture image format does not support linear blitting!");
        }

        let command_buffer = self.begin_single_time_commands();

        let mut barrier = vk::ImageMemoryBarrier::builder()
            .image(image)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_array_layer: 0,
                layer_count: 1,
                base_mip_level: 0,
                level_count: 1,
            })
            .build();

        let mut mip_width = tex_width as i32;
        let mut mip_height = tex_height as i32;

        for i in 1..mip_levels {
            barrier.subresource_range.base_mip_level = i - 1;
            barrier.old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
            barrier.new_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
            barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
            barrier.dst_access_mask = vk::AccessFlags::TRANSFER_READ;

            unsafe {
                self.device.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[barrier],
                );
            }

            let blit = vk::ImageBlit {
                src_offsets: [
                    vk::Offset3D { x: 0, y: 0, z: 0 },
                    vk::Offset3D {
                        x: mip_width,
                        y: mip_height,
                        z: 1,
                    },
                ],
                src_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: i - 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                dst_offsets: [
                    vk::Offset3D { x: 0, y: 0, z: 0 },
                    vk::Offset3D {
                        x: if mip_width > 1 { mip_width / 2 } else { 1 },
                        y: if mip_height > 1 { mip_height / 2 } else { 1 },
                        z: 1,
                    },
                ],
                dst_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: i,
                    base_array_layer: 0,
                    layer_count: 1,
                },
            };

            unsafe {
                self.device.cmd_blit_image(
                    command_buffer,
                    image,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[blit],
                    vk::Filter::LINEAR,
                );
            }

            barrier.old_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
            barrier.new_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
            barrier.src_access_mask = vk::AccessFlags::TRANSFER_READ;
            barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;

            unsafe {
                self.device.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[barrier],
                );
            }

            if mip_width > 1 {
                mip_width /= 2;
            }
            if mip_height > 1 {
                mip_height /= 2;
            }
        }

        barrier.subresource_range.base_mip_level = mip_levels - 1;
        barrier.old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
        barrier.new_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
        barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
        barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;

        unsafe {
            self.device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[barrier],
            );
        }

        self.end_single_time_commands(command_buffer);
    }

    fn get_max_usable_sample_count(&self) -> vk::SampleCountFlags {
        let physical_device_properties = unsafe {
            self.instance
                .get_physical_device_properties(self.physical_device)
        };

        let counts = physical_device_properties
            .limits
            .framebuffer_color_sample_counts
            & physical_device_properties
                .limits
                .framebuffer_depth_sample_counts;

        if counts.contains(vk::SampleCountFlags::TYPE_64) {
            return vk::SampleCountFlags::TYPE_64;
        }
        if counts.contains(vk::SampleCountFlags::TYPE_32) {
            return vk::SampleCountFlags::TYPE_32;
        }
        if counts.contains(vk::SampleCountFlags::TYPE_16) {
            return vk::SampleCountFlags::TYPE_16;
        }
        if counts.contains(vk::SampleCountFlags::TYPE_8) {
            return vk::SampleCountFlags::TYPE_8;
        }
        if counts.contains(vk::SampleCountFlags::TYPE_4) {
            return vk::SampleCountFlags::TYPE_4;
        }
        if counts.contains(vk::SampleCountFlags::TYPE_2) {
            return vk::SampleCountFlags::TYPE_2;
        }

        vk::SampleCountFlags::TYPE_1
    }

    fn create_texture_image_view(&mut self) {
        self.texture_image_view = self.create_image_view(
            self.texture_image,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageAspectFlags::COLOR,
            self.mip_levels,
        );
    }

    fn create_texture_sampler(&mut self) {
        let properties = unsafe {
            self.instance
                .get_physical_device_properties(self.physical_device)
        };

        let sampler_info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(properties.limits.max_sampler_anisotropy)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .min_lod(0.0)
            .max_lod(self.mip_levels as f32)
            .mip_lod_bias(0.0);

        self.texture_sampler = unsafe {
            self.device
                .create_sampler(&sampler_info, None)
                .expect("failed to create texture sampler!")
        };
    }

    fn create_image_view(
        &self,
        image: vk::Image,
        format: vk::Format,
        aspect_flags: vk::ImageAspectFlags,
        mip_levels: u32,
    ) -> vk::ImageView {
        let view_info = vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: aspect_flags,
                base_mip_level: 0,
                level_count: mip_levels,
                base_array_layer: 0,
                layer_count: 1,
            });

        unsafe {
            self.device
                .create_image_view(&view_info, None)
                .expect("failed to create texture image view!")
        }
    }

    fn create_image(
        &self,
        width: u32,
        height: u32,
        mip_levels: u32,
        num_samples: vk::SampleCountFlags,
        format: vk::Format,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> (vk::Image, vk::DeviceMemory) {
        let image_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .extent(vk::Extent3D {
                width,
                height,
                depth: 1,
            })
            .mip_levels(mip_levels)
            .array_layers(1)
            .format(format)
            .tiling(tiling)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .samples(num_samples);

        let image = unsafe {
            self.device
                .create_image(&image_info, None)
                .expect("failed to create image!")
        };

        let mem_requirements = unsafe { self.device.get_image_memory_requirements(image) };

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_requirements.size)
            .memory_type_index(
                self.find_memory_type(mem_requirements.memory_type_bits, properties),
            );

        let image_memory = unsafe {
            self.device
                .allocate_memory(&alloc_info, None)
                .expect("failed to allocate image memory!")
        };

        unsafe {
            self.device
                .bind_image_memory(image, image_memory, 0)
                .unwrap();
        }

        (image, image_memory)
    }

    fn transition_image_layout(
        &self,
        image: vk::Image,
        format: vk::Format,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
        mip_levels: u32,
    ) {
        let command_buffer = self.begin_single_time_commands();

        let mut barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(image)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: mip_levels,
                base_array_layer: 0,
                layer_count: 1,
            });

        let source_stage: vk::PipelineStageFlags;
        let destination_stage: vk::PipelineStageFlags;

        if old_layout == vk::ImageLayout::UNDEFINED
            && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
        {
            barrier = barrier
                .src_access_mask(vk::AccessFlags::empty())
                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE);

            source_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
            destination_stage = vk::PipelineStageFlags::TRANSFER;
        } else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
            && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        {
            barrier = barrier
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ);

            source_stage = vk::PipelineStageFlags::TRANSFER;
            destination_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
        } else {
            panic!("unsupported layout transition!");
        }

        unsafe {
            self.device.cmd_pipeline_barrier(
                command_buffer,
                source_stage,
                destination_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[*barrier],
            );
        }

        self.end_single_time_commands(command_buffer);
    }

    fn copy_buffer_to_image(&self, buffer: vk::Buffer, image: vk::Image, width: u32, height: u32) {
        let command_buffer = self.begin_single_time_commands();

        let region = vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
            image_extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
        };

        unsafe {
            self.device.cmd_copy_buffer_to_image(
                command_buffer,
                buffer,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[region],
            );
        }

        self.end_single_time_commands(command_buffer);
    }

    fn load_model(&mut self) {
        let (models, _) = tobj::load_obj(
            MODEL_PATH,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )
        .unwrap();

        let mut unique_vertices = std::collections::HashMap::<Vertex, u32>::new();

        for model in models {
            for index in model.mesh.indices {
                let index = index as usize;
                let vertex = Vertex {
                    pos: Vec3::new(
                        model.mesh.positions[3 * index + 0],
                        model.mesh.positions[3 * index + 1],
                        model.mesh.positions[3 * index + 2],
                    ),
                    tex_coord: Vec2::new(
                        model.mesh.texcoords[2 * index + 0],
                        1.0 - model.mesh.texcoords[2 * index + 1],
                    ),
                    color: Vec3::new(1.0, 1.0, 1.0),
                };

                if !unique_vertices.contains_key(&vertex) {
                    unique_vertices.insert(vertex, self.vertices.len() as u32);
                    self.vertices.push(vertex);
                }

                self.indices.push(unique_vertices[&vertex].clone());
            }
        }
    }

    fn create_vertex_buffer(&mut self) {
        let buffer_size =
            (std::mem::size_of_val(&self.vertices[0]) * self.vertices.len()) as vk::DeviceSize;

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
                &self.vertices[0] as *const Vertex,
                data as *mut Vertex,
                self.vertices.len(),
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

    fn create_index_buffer(&mut self) {
        let buffer_size =
            (std::mem::size_of_val(&self.indices[0]) * self.indices.len()) as vk::DeviceSize;

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
                &self.indices[0] as *const u32,
                data as *mut u32,
                self.indices.len(),
            );
            self.device.unmap_memory(staging_buffer_memory);
        }

        let (buffer, buffer_memory) = self.create_buffer(
            buffer_size,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        self.index_buffer = buffer;
        self.index_buffer_memory = buffer_memory;

        self.copy_buffer(staging_buffer, self.index_buffer, buffer_size);

        unsafe {
            self.device.destroy_buffer(staging_buffer, None);
            self.device.free_memory(staging_buffer_memory, None);
        }
    }

    fn create_uniform_buffers(&mut self) {
        let buffer_size = std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize;

        for _ in 0..self.swapchain_images.len() {
            let (buffer, buffer_memory) = self.create_buffer(
                buffer_size,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );
            self.uniform_buffers.push(buffer);
            self.uniform_buffers_memory.push(buffer_memory);
        }
    }

    fn create_descriptor_pool(&mut self) {
        let pool_sizes = [
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: self.swapchain_images.len() as u32,
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: self.swapchain_images.len() as u32,
            },
        ];

        let pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(self.swapchain_images.len() as u32);

        self.descriptor_pool = unsafe {
            self.device
                .create_descriptor_pool(&pool_info, None)
                .expect("failed to create descriptor pool!")
        };
    }

    fn create_descriptor_sets(&mut self) {
        let mut layouts = Vec::<vk::DescriptorSetLayout>::new();
        layouts.resize(
            self.swapchain_images.len() as usize,
            self.descriptor_set_layout,
        );
        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&layouts);

        self.descriptor_sets = unsafe {
            self.device
                .allocate_descriptor_sets(&alloc_info)
                .expect("failed to allocate descriptor sets!")
        };

        for i in 0..self.swapchain_images.len() as usize {
            let buffer_info = [vk::DescriptorBufferInfo {
                buffer: self.uniform_buffers[i],
                offset: 0,
                range: std::mem::size_of::<UniformBufferObject>() as vk::DeviceSize,
            }];

            let image_info = [vk::DescriptorImageInfo {
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                image_view: self.texture_image_view,
                sampler: self.texture_sampler,
            }];

            let descriptor_writes = [
                vk::WriteDescriptorSet::builder()
                    .dst_set(self.descriptor_sets[i])
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&buffer_info)
                    .build(),
                vk::WriteDescriptorSet::builder()
                    .dst_set(self.descriptor_sets[i])
                    .dst_binding(1)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&image_info)
                    .build(),
            ];

            unsafe {
                self.device.update_descriptor_sets(&descriptor_writes, &[]);
            }
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

    fn begin_single_time_commands(&self) -> vk::CommandBuffer {
        let alloc_info = vk::CommandBufferAllocateInfo::builder()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(self.command_pool)
            .command_buffer_count(1);

        let command_buffer =
            unsafe { self.device.allocate_command_buffers(&alloc_info).unwrap()[0] };

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device
                .begin_command_buffer(command_buffer, &begin_info)
                .unwrap();
        }

        command_buffer
    }

    fn end_single_time_commands(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device.end_command_buffer(command_buffer).unwrap();
        }
        let command_buffer = [command_buffer];

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

    fn copy_buffer(&self, src_buffer: vk::Buffer, dst_buffer: vk::Buffer, size: vk::DeviceSize) {
        let command_buffer = self.begin_single_time_commands();

        let copy_region = vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size,
        };
        unsafe {
            self.device
                .cmd_copy_buffer(command_buffer, src_buffer, dst_buffer, &[copy_region]);
        }

        self.end_single_time_commands(command_buffer);
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

            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];
            render_pass_info = render_pass_info.clear_values(&clear_values);

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

                self.device.cmd_bind_index_buffer(
                    self.command_buffers[i],
                    self.index_buffer,
                    0,
                    vk::IndexType::UINT32,
                );

                self.device.cmd_bind_descriptor_sets(
                    self.command_buffers[i],
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline_layout,
                    0,
                    &self.descriptor_sets[i..=i],
                    &[],
                );

                self.device.cmd_draw_indexed(
                    self.command_buffers[i],
                    self.indices.len() as u32,
                    1,
                    0,
                    0,
                    0,
                );

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

    fn update_uniform_buffer(&self, current_image: usize) {
        let time = self.start_time.elapsed().as_secs_f32();

        let ubo = UniformBufferObject {
            model: Mat4::IDENTITY.rotate(time * 90f32.to_radians(), Vec3::UNIT_Z),
            view: Mat4::look_at(
                Vec3::new(2.0, 2.0, 2.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            proj: Mat4::perspective(
                45f32.to_radians(),
                self.swapchain_extent.width as f32 / self.swapchain_extent.height as f32,
                0.1,
                10.0,
            ),
        };

        let buffer_size = std::mem::size_of_val(&ubo) as vk::DeviceSize;
        unsafe {
            let data = self
                .device
                .map_memory(
                    self.uniform_buffers_memory[current_image],
                    0,
                    buffer_size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap();
            std::ptr::copy_nonoverlapping(
                &ubo as *const UniformBufferObject,
                data as *mut UniformBufferObject,
                1,
            );
            self.device
                .unmap_memory(self.uniform_buffers_memory[current_image]);
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

        self.update_uniform_buffer(image_index);

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

        let supported_features = unsafe { self.instance.get_physical_device_features(device) };

        indices.is_complete()
            && extensions_supported
            && swapchain_adequate
            && supported_features.sampler_anisotropy > 0
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
            self.device.destroy_sampler(self.texture_sampler, None);
            self.device
                .destroy_image_view(self.texture_image_view, None);

            self.device.destroy_image(self.texture_image, None);
            self.device.free_memory(self.texture_image_memory, None);

            self.device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.device.destroy_buffer(self.index_buffer, None);
            self.device.free_memory(self.index_buffer_memory, None);

            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);

            for semaphore in self.render_finished_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore, None);
            }
            for semaphore in self.image_available_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore, None);
            }
            for fence in self.in_flight_fences.drain(..) {
                self.device.destroy_fence(fence, None);
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
