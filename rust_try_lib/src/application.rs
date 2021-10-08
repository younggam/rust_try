use std::ffi::CString;

use ash::vk;

use crate::graphics::elements::*;
use crate::utils::offset_of;

const SWAPCHIAN_IMAGE_COUNT: usize = 2;
const IMAGE_AVAILABLE_INDEX: usize = 0;
const RENDERING_DONE_INDEX: usize = 1;

pub struct Application {
    entry: ash::Entry,
    window: winit::window::Window,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    device: ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    frame_index: usize,
    command_buffers: [vk::CommandBuffer; SWAPCHIAN_IMAGE_COUNT],
    semaphores: [[vk::Semaphore; 2]; SWAPCHIAN_IMAGE_COUNT],
    fences: [vk::Fence; SWAPCHIAN_IMAGE_COUNT],
    vertex_buffer: vk::Buffer,
    vertex_device_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_device_memory: vk::DeviceMemory,
    uniform_buffers: [vk::Buffer; SWAPCHIAN_IMAGE_COUNT],
    uniform_device_memories: [vk::DeviceMemory; SWAPCHIAN_IMAGE_COUNT],
    texture_image: vk::Image,
    texture_device_memory: vk::DeviceMemory,
    texture_image_view: vk::ImageView,
    texture_sampler: vk::Sampler,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_extent: vk::Extent2D,
    swapchain_image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
    shader_modules: [vk::ShaderModule; 2],
    material_descriptor_set_layout: vk::DescriptorSetLayout,
    texture_descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    descriptor_pool: vk::DescriptorPool,
    material_descriptor_sets: [vk::DescriptorSet; SWAPCHIAN_IMAGE_COUNT],
    texture_descriptor_sets: [vk::DescriptorSet; SWAPCHIAN_IMAGE_COUNT],
}

impl Application {
    pub fn new(window: winit::window::Window) -> Self {
        let entry = unsafe { ash::Entry::new().expect("Vulkan functions loading error") };
        let instance = Self::create_instance(&entry);
        let physical_device = Self::create_physical_device(&instance);
        let physical_device_memory_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };
        let queue_family_index = Self::find_queue_family_index(&instance, &physical_device);
        let device = Self::create_device(&instance, &queue_family_index, &physical_device);
        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let command_pool = Self::create_command_pool(&device, &queue_family_index);
        let command_buffers = Self::allocate_command_buffers(&device, &command_pool);
        let semaphores = Self::create_semaphores(&device);
        let fences = Self::create_fences(&device);
        let (vertex_buffer, vertex_device_memory) =
            Self::create_vertex_resources(&device, &physical_device_memory_properties);
        let (index_buffer, index_device_memory) = Self::init_index_resources(
            &device,
            &physical_device_memory_properties,
            &command_pool,
            &queue,
        );
        let (uniform_buffers, uniform_device_memories) =
            Self::create_uniform_resources(&device, &physical_device_memory_properties);
        let (texture_image, texture_device_memory, texture_image_view, texture_sampler) =
            Self::init_texture_resources(
                &device,
                &physical_device_memory_properties,
                &command_pool,
                &queue_family_index,
                &queue,
            );
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);

        Self {
            entry,
            window,
            instance,
            physical_device,
            queue_family_index,
            device,
            queue,
            command_pool,
            frame_index: 0,
            command_buffers,
            semaphores,
            fences,
            vertex_buffer,
            vertex_device_memory,
            index_buffer,
            index_device_memory,
            uniform_buffers,
            uniform_device_memories,
            texture_image,
            texture_device_memory,
            texture_image_view,
            texture_sampler,
            surface_loader,
            surface: vk::SurfaceKHR::null(),
            swapchain_loader,
            swapchain: vk::SwapchainKHR::null(),
            swapchain_images: Vec::<vk::Image>::new(),
            swapchain_image_extent: vk::Extent2D {
                width: 0,
                height: 0,
            },
            swapchain_image_views: Vec::<vk::ImageView>::new(),
            render_pass: vk::RenderPass::null(),
            framebuffers: Vec::<vk::Framebuffer>::new(),
            shader_modules: [vk::ShaderModule::null(); 2],
            material_descriptor_set_layout: vk::DescriptorSetLayout::null(),
            texture_descriptor_set_layout: vk::DescriptorSetLayout::null(),
            pipeline_layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
            descriptor_pool: vk::DescriptorPool::null(),
            material_descriptor_sets: [vk::DescriptorSet::null(); SWAPCHIAN_IMAGE_COUNT],
            texture_descriptor_sets: [vk::DescriptorSet::null(); SWAPCHIAN_IMAGE_COUNT],
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
            #[cfg(target_os = "windows")]
            CString::new("VK_KHR_win32_surface").unwrap(),
            #[cfg(target_os = "macos")]
            CString::new("VK_MVK_macos_surface").unwrap(),
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
        assert_ne!(result, usize::MAX);
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

        let device_extension_names = [CString::new("VK_KHR_swapchain").unwrap()];
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
                .expect("Command pool creation error")
        }
    }

    fn allocate_command_buffers(
        device: &ash::Device,
        command_pool: &vk::CommandPool,
    ) -> [vk::CommandBuffer; SWAPCHIAN_IMAGE_COUNT] {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(*command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(SWAPCHIAN_IMAGE_COUNT as u32)
            .build();

        let command_buffers = unsafe {
            device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Command buffers allocation error")
        };

        [command_buffers[0], command_buffers[1]]
    }

    fn create_semaphores(device: &ash::Device) -> [[vk::Semaphore; 2]; SWAPCHIAN_IMAGE_COUNT] {
        let semaphore_create_info = vk::SemaphoreCreateInfo::builder().build();
        let mut semaphores = [[vk::Semaphore::null(); 2]; SWAPCHIAN_IMAGE_COUNT];

        unsafe {
            for i in 0..SWAPCHIAN_IMAGE_COUNT {
                for j in 0..SWAPCHIAN_IMAGE_COUNT {
                    semaphores[i][j] = device
                        .create_semaphore(&semaphore_create_info, None)
                        .expect("Semaphore creation error");
                }
            }
        }

        semaphores
    }

    fn create_fences(device: &ash::Device) -> [vk::Fence; SWAPCHIAN_IMAGE_COUNT] {
        let fence_create_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED)
            .build();

        let mut fences = [vk::Fence::null(); SWAPCHIAN_IMAGE_COUNT];

        unsafe {
            for i in 0..SWAPCHIAN_IMAGE_COUNT {
                fences[i] = device
                    .create_fence(&fence_create_info, None)
                    .expect("Fence creation error")
            }
        }

        fences
    }

    fn find_memory_index(
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        buffer_memory_requirements: &vk::MemoryRequirements,
        memory_property_flags: vk::MemoryPropertyFlags,
    ) -> u32 {
        for (i, memory_type) in physical_device_memory_properties
            .memory_types
            .iter()
            .enumerate()
        {
            if (buffer_memory_requirements.memory_type_bits & (1 << i)) != 0
                && memory_type.property_flags.contains(memory_property_flags)
            {
                return i as u32;
            }
        }

        panic!("Memory index finding error");
    }

    fn create_vertex_resources(
        device: &ash::Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        let vertices = [
            //          위치            색상           텍스쳐 좌표
            Vertex::new(0.0, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0),
            Vertex::new(0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0),
            Vertex::new(-0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
        ];

        let buffer_create_info = vk::BufferCreateInfo::builder()
            .size(std::mem::size_of_val(&vertices) as vk::DeviceSize)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .build();

        let vertex_buffer = unsafe {
            device
                .create_buffer(&buffer_create_info, None)
                .expect("Buffer creation error")
        };

        let buffer_memory_requirements =
            unsafe { device.get_buffer_memory_requirements(vertex_buffer) };

        let memory_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(buffer_memory_requirements.size)
            .memory_type_index(Self::find_memory_index(
                physical_device_memory_properties,
                &buffer_memory_requirements,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ))
            .build();

        let vertex_device_memory = unsafe {
            device
                .allocate_memory(&memory_allocate_info, None)
                .expect("Memory allocation error")
        };

        unsafe {
            device
                .bind_buffer_memory(vertex_buffer, vertex_device_memory, 0)
                .expect("Buffer memory binding error");
        }

        let contents = unsafe {
            device
                .map_memory(
                    vertex_device_memory,
                    0,
                    buffer_memory_requirements.size,
                    vk::MemoryMapFlags::empty(),
                )
                .expect("Memory mapping error")
        };

        unsafe {
            let mut contents = ash::util::Align::new(
                contents,
                std::mem::align_of::<Vertex>() as vk::DeviceSize,
                buffer_memory_requirements.size,
            );
            contents.copy_from_slice(&vertices);

            device.unmap_memory(vertex_device_memory);
        }

        (vertex_buffer, vertex_device_memory)
    }

    fn init_index_resources(
        device: &ash::Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        command_pool: &vk::CommandPool,
        queue: &vk::Queue,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        let indices = [0u16, 1, 2];
        let indices_memory_size = std::mem::size_of_val(&indices) as vk::DeviceSize;

        let buffer_create_info = vk::BufferCreateInfo::builder()
            .size(indices_memory_size)
            .usage(vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST)
            .build();

        let index_buffer = unsafe {
            device
                .create_buffer(&buffer_create_info, None)
                .expect("Buffer creation error")
        };

        let buffer_memory_requirements =
            unsafe { device.get_buffer_memory_requirements(index_buffer) };

        let memory_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(buffer_memory_requirements.size)
            .memory_type_index(Self::find_memory_index(
                physical_device_memory_properties,
                &buffer_memory_requirements,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            ))
            .build();

        let index_device_memory = unsafe {
            device
                .allocate_memory(&memory_allocate_info, None)
                .expect("Memory allocation error")
        };

        unsafe {
            device
                .bind_buffer_memory(index_buffer, index_device_memory, 0)
                .expect("Buffer memory binding error");
        }

        let buffer_create_info = vk::BufferCreateInfo::builder()
            .size(indices_memory_size)
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .build();

        let staging_buffer = unsafe {
            device
                .create_buffer(&buffer_create_info, None)
                .expect("Buffer creation error")
        };

        let buffer_memory_requirements =
            unsafe { device.get_buffer_memory_requirements(staging_buffer) };

        let memory_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(buffer_memory_requirements.size)
            .memory_type_index(Self::find_memory_index(
                physical_device_memory_properties,
                &buffer_memory_requirements,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ))
            .build();

        let staging_device_memory = unsafe {
            device
                .allocate_memory(&memory_allocate_info, None)
                .expect("Memory allocation error")
        };

        unsafe {
            device
                .bind_buffer_memory(staging_buffer, staging_device_memory, 0)
                .expect("Buffer memory binding error");
        }

        let contents = unsafe {
            device
                .map_memory(
                    staging_device_memory,
                    0,
                    buffer_memory_requirements.size,
                    vk::MemoryMapFlags::empty(),
                )
                .expect("Memory mapping error")
        };

        unsafe {
            let mut contents = ash::util::Align::new(
                contents,
                std::mem::align_of::<u16>() as vk::DeviceSize,
                buffer_memory_requirements.size,
            );
            contents.copy_from_slice(&indices);

            device.unmap_memory(staging_device_memory);
        }

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(*command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1)
            .build();

        let command_buffer = unsafe {
            device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Command buffers allocating error")[0]
        };

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        unsafe {
            device
                .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                .expect("Command buffer begining error");
        }

        let buffer_copy = vk::BufferCopy::builder().size(indices_memory_size).build();

        unsafe {
            device.cmd_copy_buffer(command_buffer, staging_buffer, index_buffer, &[buffer_copy]);

            device
                .end_command_buffer(command_buffer)
                .expect("Command buffer ending error");
        }

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&[command_buffer])
            .build();

        unsafe {
            device
                .queue_submit(*queue, &[submit_info], vk::Fence::null())
                .expect("Queue submitting error");
            device.device_wait_idle().expect("Device waiting error");

            device.free_memory(staging_device_memory, None);
            device.destroy_buffer(staging_buffer, None);
        }

        (index_buffer, index_device_memory)
    }

    fn create_uniform_resources(
        device: &ash::Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
    ) -> (
        [vk::Buffer; SWAPCHIAN_IMAGE_COUNT],
        [vk::DeviceMemory; SWAPCHIAN_IMAGE_COUNT],
    ) {
        let mut uniform_buffers = [vk::Buffer::null(); SWAPCHIAN_IMAGE_COUNT];
        let mut uniform_device_memories = [vk::DeviceMemory::null(); SWAPCHIAN_IMAGE_COUNT];

        for i in 0..SWAPCHIAN_IMAGE_COUNT {
            let buffer_create_info = vk::BufferCreateInfo::builder()
                .size(std::mem::size_of::<Material>() as vk::DeviceSize)
                .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
                .build();

            uniform_buffers[i] = unsafe {
                device
                    .create_buffer(&buffer_create_info, None)
                    .expect("Buffer creation error")
            };

            let buffer_memory_requirements =
                unsafe { device.get_buffer_memory_requirements(uniform_buffers[i]) };

            let memory_allocate_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(buffer_memory_requirements.size)
                .memory_type_index(Self::find_memory_index(
                    physical_device_memory_properties,
                    &buffer_memory_requirements,
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                ))
                .build();

            uniform_device_memories[i] = unsafe {
                device
                    .allocate_memory(&memory_allocate_info, None)
                    .expect("Memory allocation error")
            };

            unsafe {
                device
                    .bind_buffer_memory(uniform_buffers[i], uniform_device_memories[i], 0)
                    .expect("Buffer memory binding error")
            }
        }

        (uniform_buffers, uniform_device_memories)
    }

    fn init_texture_resources(
        device: &ash::Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        command_pool: &vk::CommandPool,
        queue_family_index: &u32,
        queue: &vk::Queue,
    ) -> (vk::Image, vk::DeviceMemory, vk::ImageView, vk::Sampler) {
        let image_buffer = image::open("assets/logo.png")
            .expect("Cannot open image")
            .to_rgba8();
        let sample_layout = image_buffer.sample_layout();

        let image_create_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_UNORM)
            .extent(vk::Extent3D {
                width: sample_layout.width,
                height: sample_layout.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .build();

        let texture_image = unsafe {
            device
                .create_image(&image_create_info, None)
                .expect("Image creation error")
        };

        let image_memory_requirements =
            unsafe { device.get_image_memory_requirements(texture_image) };

        let memory_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(image_memory_requirements.size)
            .memory_type_index(Self::find_memory_index(
                physical_device_memory_properties,
                &image_memory_requirements,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            ))
            .build();

        let texture_device_memory = unsafe {
            device
                .allocate_memory(&memory_allocate_info, None)
                .expect("Memory allocation error")
        };

        unsafe {
            device
                .bind_image_memory(texture_image, texture_device_memory, 0)
                .expect("Image memory binding error");
        }

        let buffer_create_info = vk::BufferCreateInfo::builder()
            .size(
                (sample_layout.width * sample_layout.height * sample_layout.channels as u32)
                    as vk::DeviceSize,
            )
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .build();

        let staging_buffer = unsafe {
            device
                .create_buffer(&buffer_create_info, None)
                .expect("Buffer creation error")
        };

        let buffer_memory_requirements =
            unsafe { device.get_buffer_memory_requirements(staging_buffer) };

        let memory_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(buffer_memory_requirements.size)
            .memory_type_index(Self::find_memory_index(
                physical_device_memory_properties,
                &buffer_memory_requirements,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ))
            .build();

        let staging_device_memory = unsafe {
            device
                .allocate_memory(&memory_allocate_info, None)
                .expect("Memory allocation error")
        };

        unsafe {
            device
                .bind_buffer_memory(staging_buffer, staging_device_memory, 0)
                .expect("Buffer memory binding error")
        }

        let contents = unsafe {
            device
                .map_memory(
                    staging_device_memory,
                    0,
                    buffer_memory_requirements.size,
                    vk::MemoryMapFlags::empty(),
                )
                .expect("Memory mapping error")
        };

        unsafe {
            let mut contents = ash::util::Align::new(
                contents,
                std::mem::align_of::<u8>() as vk::DeviceSize,
                buffer_memory_requirements.size,
            );
            contents.copy_from_slice(image_buffer.as_raw());

            device.unmap_memory(staging_device_memory);
        }

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(*command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1)
            .build();

        let command_buffer = unsafe {
            device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Command buffers allocating error")[0]
        };

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        unsafe {
            device
                .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                .expect("Command buffer begining error");
        }

        let image_memory_barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_queue_family_index(*queue_family_index)
            .dst_queue_family_index(*queue_family_index)
            .image(texture_image)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .build();

        unsafe {
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        let buffer_image_copy = vk::BufferImageCopy::builder()
            .image_subresource(
                vk::ImageSubresourceLayers::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .mip_level(0)
                    .layer_count(1)
                    .build(),
            )
            .image_extent(vk::Extent3D {
                width: sample_layout.width,
                height: sample_layout.height,
                depth: 1,
            })
            .build();

        unsafe {
            device.cmd_copy_buffer_to_image(
                command_buffer,
                staging_buffer,
                texture_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[buffer_image_copy],
            );
        }

        let image_memory_barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .src_queue_family_index(*queue_family_index)
            .dst_queue_family_index(*queue_family_index)
            .image(texture_image)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .build();

        unsafe {
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
            device
                .end_command_buffer(command_buffer)
                .expect("Command buffer ending error");
        }

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&[command_buffer])
            .build();

        unsafe {
            device
                .queue_submit(*queue, &[submit_info], vk::Fence::null())
                .expect("Queue submitting error");
            device.device_wait_idle().expect("Device waiting error");

            device.free_command_buffers(*command_pool, &[command_buffer]);
            device.free_memory(staging_device_memory, None);
            device.destroy_buffer(staging_buffer, None);
        }

        let image_view_create_info = vk::ImageViewCreateInfo::builder()
            .image(texture_image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_UNORM)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .build();

        let texture_image_view = unsafe {
            device
                .create_image_view(&image_view_create_info, None)
                .expect("Image view creation error")
        };

        let sampler_create_info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .build();

        let texture_sampler = unsafe {
            device
                .create_sampler(&sampler_create_info, None)
                .expect("Sampler creation error")
        };

        (
            texture_image,
            texture_device_memory,
            texture_image_view,
            texture_sampler,
        )
    }

    fn create_surface(&mut self) {
        unsafe {
            self.surface =
                ash_window::create_surface(&self.entry, &self.instance, &self.window, None)
                    .expect("Surface creation error");

            match self.surface_loader.get_physical_device_surface_support(
                self.physical_device,
                self.queue_family_index,
                self.surface,
            ) {
                Ok(supported) if supported => {}
                Ok(_) => panic!("Suface is not supported"),
                _ => panic!("Suface support is inaccessible"),
            };
        }
    }

    fn default_surface_format(&self) -> vk::SurfaceFormatKHR {
        let surface_formats = unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(self.physical_device, self.surface)
                .expect("There is no available surface format")
        };
        surface_formats[0]
    }

    fn create_swapchain(&mut self) {
        let surface_format = self.default_surface_format();

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

        self.swapchain_image_extent = surface_capabilities.current_extent;

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
                .begin_command_buffer(
                    self.command_buffers[self.frame_index],
                    &command_buffer_begin_info,
                )
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
                self.command_buffers[self.frame_index],
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &image_memory_barriers,
            );
            self.device
                .end_command_buffer(self.command_buffers[self.frame_index])
                .expect("Command buffer ending error");
        }

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&[self.command_buffers[self.frame_index]])
            .build();

        unsafe {
            self.device
                .queue_submit(self.queue, &[submit_info], vk::Fence::null())
                .expect("Queue submitting error");
            self.device
                .device_wait_idle()
                .expect("Device waiting error");
        }
    }

    fn create_swapchain_image_views(&mut self) {
        self.swapchain_image_views.clear();

        let surface_format = self.default_surface_format();

        let mut image_view_create_info = vk::ImageViewCreateInfo::builder()
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(surface_format.format)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .build();

        for swapchain_image in &self.swapchain_images {
            image_view_create_info.image = *swapchain_image;

            self.swapchain_image_views.push(unsafe {
                self.device
                    .create_image_view(&image_view_create_info, None)
                    .expect("Image view creation error")
            });
        }
    }

    fn create_render_pass(&mut self) {
        let surface_format = self.default_surface_format();

        let attachment_description = vk::AttachmentDescription::builder()
            .format(surface_format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .initial_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();

        let color_attachment_reference = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass_description = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&[color_attachment_reference])
            .build();

        let render_pass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&[attachment_description])
            .subpasses(&[subpass_description])
            .build();

        self.render_pass = unsafe {
            self.device
                .create_render_pass(&render_pass_create_info, None)
                .expect("Render pass creation error")
        };
    }

    fn create_framebuffers(&mut self) {
        self.framebuffers.clear();

        for image_view in &self.swapchain_image_views {
            let framebuffer_create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(self.render_pass)
                .attachments(&[*image_view])
                .width(self.swapchain_image_extent.width)
                .height(self.swapchain_image_extent.height)
                .layers(1)
                .build();

            self.framebuffers.push(unsafe {
                self.device
                    .create_framebuffer(&framebuffer_create_info, None)
                    .expect("Render pass creation error")
            });
        }
    }

    fn create_shader_modules(&mut self) {
        let vertex_shader_raw = r#"
            #version 460
            precision mediump float;

            layout(location = 0) in vec2 i_pos;
            layout(location = 1) in vec3 i_col;
            layout(location = 2) in vec2 i_uv;

            layout(location = 0) out vec3 o_col;
            layout(location = 1) out vec2 o_uv;

            void main() {
                gl_Position = vec4(i_pos, 0.0, 1.0);
                o_col = i_col;
                o_uv = i_uv;
            }"#;

        let mut compiler = shaderc::Compiler::new().expect("Compiler error");

        let vertex_shader = compiler
            .compile_into_spirv(
                vertex_shader_raw,
                shaderc::ShaderKind::Vertex,
                "Vertex shader",
                "main",
                None,
            )
            .expect("Compilation error");

        let vertex_shader_module_create_info = vk::ShaderModuleCreateInfo::builder()
            .code(vertex_shader.as_binary())
            .build();

        self.shader_modules[0] = unsafe {
            self.device
                .create_shader_module(&vertex_shader_module_create_info, None)
                .expect("Shader module creation error")
        };

        let fragment_shader_raw = r#"
            #version 460
            precision mediump float;

            layout(location = 0) in vec3 i_col;
            layout(location = 1) in vec2 i_uv;

            layout(location = 0) out vec4 fragment_color0;

            layout(set = 0, binding = 0) uniform Material {
                vec3 col;
            } material;

            layout(set = 1, binding = 0) uniform sampler2D tex;

            void main() {
                vec3 col = i_col;
                col *= material.col;
                col *= texture(tex, i_uv).rgb;
                fragment_color0 = vec4(col, 1.0);
            }"#;

        let fragment_shader = compiler
            .compile_into_spirv(
                fragment_shader_raw,
                shaderc::ShaderKind::Fragment,
                "Fragment shader",
                "main",
                None,
            )
            .expect("Compilation error");

        let fragment_shader_module_create_info = vk::ShaderModuleCreateInfo::builder()
            .code(fragment_shader.as_binary())
            .build();

        self.shader_modules[1] = unsafe {
            self.device
                .create_shader_module(&fragment_shader_module_create_info, None)
                .expect("Shader module creation error")
        };
    }

    fn create_descriptor_set_layouts(&mut self) {
        let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .build();

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(&[descriptor_set_layout_binding])
            .build();

        self.material_descriptor_set_layout = unsafe {
            self.device
                .create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
                .expect("Descriptor set layout creation error")
        };

        let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .build();

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(&[descriptor_set_layout_binding])
            .build();

        self.texture_descriptor_set_layout = unsafe {
            self.device
                .create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
                .expect("Descriptor set layout creation error")
        };
    }

    fn create_pipeline_layout(&mut self) {
        let descriptor_set_layouts = [
            self.material_descriptor_set_layout,
            self.texture_descriptor_set_layout,
        ];

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(&descriptor_set_layouts)
            .build();

        self.pipeline_layout = unsafe {
            self.device
                .create_pipeline_layout(&pipeline_layout_create_info, None)
                .expect("Pipeline layout creation error")
        };
    }

    fn create_pipeline(&mut self) {
        let entry_point_name = CString::new("main").expect("CString new error");
        let pipeline_shader_stage_create_infos = [
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(self.shader_modules[0])
                .name(entry_point_name.as_c_str())
                .build(),
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(self.shader_modules[1])
                .name(entry_point_name.as_c_str())
                .build(),
        ];

        let vertex_input_binding_description = vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        };

        let vertex_input_attribute_description = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, x) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, r) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 2,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, u) as u32,
            },
        ];

        let pipeline_vertex_input_state_create_info =
            vk::PipelineVertexInputStateCreateInfo::builder()
                .vertex_binding_descriptions(&[vertex_input_binding_description])
                .vertex_attribute_descriptions(&vertex_input_attribute_description)
                .build();

        let pipeline_input_assembly_state_create_info =
            vk::PipelineInputAssemblyStateCreateInfo::builder()
                .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
                .build();

        let viewport = vk::Viewport::builder()
            .width(self.swapchain_image_extent.width as f32)
            .height(self.swapchain_image_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();

        let scissor = vk::Rect2D::builder()
            .extent(self.swapchain_image_extent)
            .build();

        let pipeline_view_port_state_create_info = vk::PipelineViewportStateCreateInfo::builder()
            .viewport_count(1)
            .viewports(&[viewport])
            .scissor_count(1)
            .scissors(&[scissor])
            .build();

        let pipeline_rasterization_state_create_info =
            vk::PipelineRasterizationStateCreateInfo::builder()
                .polygon_mode(vk::PolygonMode::FILL)
                .cull_mode(vk::CullModeFlags::NONE)
                .line_width(1.0)
                .build();

        let pipeline_multisample_state_create_info =
            vk::PipelineMultisampleStateCreateInfo::builder()
                .rasterization_samples(vk::SampleCountFlags::TYPE_1)
                .build();

        let pipeline_depth_stencil_state_create_info =
            vk::PipelineDepthStencilStateCreateInfo::builder().build();

        let pipeline_color_blend_attachment_state =
            vk::PipelineColorBlendAttachmentState::builder()
                .color_write_mask(vk::ColorComponentFlags::all())
                .build();

        let pipeline_color_blend_state_create_info =
            vk::PipelineColorBlendStateCreateInfo::builder()
                .attachments(&[pipeline_color_blend_attachment_state])
                .build();

        let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&pipeline_shader_stage_create_infos)
            .vertex_input_state(&pipeline_vertex_input_state_create_info)
            .input_assembly_state(&pipeline_input_assembly_state_create_info)
            .viewport_state(&pipeline_view_port_state_create_info)
            .rasterization_state(&pipeline_rasterization_state_create_info)
            .multisample_state(&pipeline_multisample_state_create_info)
            .depth_stencil_state(&pipeline_depth_stencil_state_create_info)
            .color_blend_state(&pipeline_color_blend_state_create_info)
            .layout(self.pipeline_layout)
            .render_pass(self.render_pass)
            .build();

        self.pipeline = unsafe {
            self.device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[graphics_pipeline_create_info],
                    None,
                )
                .expect("Graphics pipeline creation error")[0]
        };
    }

    fn create_descriptor_pool(&mut self) {
        let descriptor_pool_sizes = [
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 2,
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 2,
            },
        ];

        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo::builder()
            .max_sets(4)
            .pool_sizes(&descriptor_pool_sizes)
            .build();

        self.descriptor_pool = unsafe {
            self.device
                .create_descriptor_pool(&descriptor_pool_create_info, None)
                .expect("Descriptor pool creation error")
        };
    }

    fn create_descriptor_sets(&mut self) {
        for i in 0..SWAPCHIAN_IMAGE_COUNT {
            let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(self.descriptor_pool)
                .set_layouts(&[self.material_descriptor_set_layout])
                .build();

            self.material_descriptor_sets[i] = unsafe {
                self.device
                    .allocate_descriptor_sets(&descriptor_set_allocate_info)
                    .expect("Descriptor sets creation error")[0]
            };

            let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(self.descriptor_pool)
                .set_layouts(&[self.texture_descriptor_set_layout])
                .build();

            self.texture_descriptor_sets[i] = unsafe {
                self.device
                    .allocate_descriptor_sets(&descriptor_set_allocate_info)
                    .expect("Descriptor sets creation error")[0]
            };
        }
    }

    pub fn on_startup(&mut self) {
        self.create_surface();
        self.create_swapchain();
        self.init_swapchain_images();
        self.create_swapchain_image_views();
        self.create_render_pass();
        self.create_framebuffers();
        self.create_shader_modules();
        self.create_descriptor_set_layouts();
        self.create_pipeline_layout();
        self.create_pipeline();
        self.create_descriptor_pool();
        self.create_descriptor_sets();
    }

    pub fn on_render(&mut self) {
        let (swapchain_image_index, _is_suboptimal) = unsafe {
            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    u64::MAX,
                    self.semaphores[self.frame_index][IMAGE_AVAILABLE_INDEX],
                    vk::Fence::null(),
                )
                .expect("Next image acquiring error")
        };
        let swapchain_image_index = swapchain_image_index as usize;

        let swapchain_image = &self.swapchain_images[swapchain_image_index];

        unsafe {
            if !self
                .device
                .get_fence_status(self.fences[self.frame_index])
                .expect("Fence status getting error")
            {
                self.device
                    .wait_for_fences(&[self.fences[self.frame_index]], true, u64::MAX)
                    .expect("Fences waiting error");
            }
            self.device
                .reset_fences(&[self.fences[self.frame_index]])
                .expect("Fences reseting error");
        }

        let material_memory_size = std::mem::size_of::<Material>() as vk::DeviceSize;

        let descriptor_buffer_info = vk::DescriptorBufferInfo {
            buffer: self.uniform_buffers[self.frame_index],
            offset: 0,
            range: material_memory_size,
        };

        let write_descriptor_set = vk::WriteDescriptorSet::builder()
            .dst_set(self.material_descriptor_sets[self.frame_index])
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&[descriptor_buffer_info])
            .build();

        unsafe {
            self.device
                .update_descriptor_sets(&[write_descriptor_set], &[]);
        }

        let descriptor_image_info = vk::DescriptorImageInfo {
            sampler: self.texture_sampler,
            image_view: self.texture_image_view,
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        };

        let write_descriptor_set = vk::WriteDescriptorSet::builder()
            .dst_set(self.texture_descriptor_sets[self.frame_index])
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&[descriptor_image_info])
            .build();

        unsafe {
            self.device
                .update_descriptor_sets(&[write_descriptor_set], &[]);
        }

        unsafe {
            let material: *mut Material =
                self.device
                    .map_memory(
                        self.uniform_device_memories[self.frame_index],
                        0,
                        material_memory_size,
                        vk::MemoryMapFlags::empty(),
                    )
                    .expect("Memory mapping error") as *mut Material;

            static mut TIME: f32 = 0.0;

            let value = (TIME.cos() + 1.0) / 2.0;

            TIME += 0.05;

            (*material).r = value;
            (*material).g = value;
            (*material).b = value;

            self.device
                .unmap_memory(self.uniform_device_memories[self.frame_index]);
        }

        unsafe {
            self.device
                .reset_command_buffer(
                    self.command_buffers[self.frame_index],
                    vk::CommandBufferResetFlags::empty(),
                )
                .expect("Command buffer reseting error");
        }

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        unsafe {
            self.device
                .begin_command_buffer(
                    self.command_buffers[self.frame_index],
                    &command_buffer_begin_info,
                )
                .expect("Command buffer begining error");
        }

        let image_memory_barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
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
                self.command_buffers[self.frame_index],
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        let clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                //        R     G     B     A
                float32: [0.15, 0.15, 0.15, 1.0],
            },
        };

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[swapchain_image_index])
            .render_area(
                vk::Rect2D::builder()
                    .extent(self.swapchain_image_extent)
                    .build(),
            )
            .clear_values(&[clear_value])
            .build();

        unsafe {
            self.device.cmd_begin_render_pass(
                self.command_buffers[self.frame_index],
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            self.device.cmd_bind_vertex_buffers(
                self.command_buffers[self.frame_index],
                0,
                &[self.vertex_buffer],
                &[0],
            );

            self.device.cmd_bind_index_buffer(
                self.command_buffers[self.frame_index],
                self.index_buffer,
                0,
                vk::IndexType::UINT16,
            );

            self.device.cmd_bind_pipeline(
                self.command_buffers[self.frame_index],
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            );

            self.device.cmd_bind_descriptor_sets(
                self.command_buffers[self.frame_index],
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[self.material_descriptor_sets[self.frame_index]],
                &[],
            );

            self.device.cmd_bind_descriptor_sets(
                self.command_buffers[self.frame_index],
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                1,
                &[self.texture_descriptor_sets[self.frame_index]],
                &[],
            );

            self.device
                .cmd_draw_indexed(self.command_buffers[self.frame_index], 3, 1, 0, 0, 0);

            self.device
                .cmd_end_render_pass(self.command_buffers[self.frame_index]);
        }

        let image_memory_barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
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
                self.command_buffers[self.frame_index],
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
            self.device
                .end_command_buffer(self.command_buffers[self.frame_index])
                .expect("Command buffer ending error");
        }

        let wait_dst_stage_mask = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&self.semaphores[self.frame_index][..=IMAGE_AVAILABLE_INDEX])
            .wait_dst_stage_mask(&[wait_dst_stage_mask])
            .command_buffers(&[self.command_buffers[self.frame_index]])
            .signal_semaphores(&self.semaphores[self.frame_index][RENDERING_DONE_INDEX..])
            .build();

        unsafe {
            self.device
                .queue_submit(self.queue, &[submit_info], self.fences[self.frame_index])
                .expect("Queue submitting error");
        }

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&self.semaphores[self.frame_index][RENDERING_DONE_INDEX..])
            .swapchains(&[self.swapchain])
            .image_indices(&[swapchain_image_index as u32])
            .build();

        unsafe {
            self.swapchain_loader
                .queue_present(self.queue, &present_info)
                .expect("Queue presenting error");
        }

        self.frame_index = (1 + self.frame_index) % SWAPCHIAN_IMAGE_COUNT;
    }

    pub fn on_shutdown(&mut self) {
        unsafe {
            self.device
                .device_wait_idle()
                .expect("Device waiting error");

            self.device
                .destroy_descriptor_pool(self.descriptor_pool, None);
            self.device.destroy_pipeline(self.pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device
                .destroy_descriptor_set_layout(self.material_descriptor_set_layout, None);
            self.device
                .destroy_descriptor_set_layout(self.texture_descriptor_set_layout, None);
            for shader_module in self.shader_modules {
                self.device.destroy_shader_module(shader_module, None);
            }
            for framebuffer in &self.framebuffers {
                self.device.destroy_framebuffer(*framebuffer, None);
            }
            self.device.destroy_render_pass(self.render_pass, None);
            for image_view in &self.swapchain_image_views {
                self.device.destroy_image_view(*image_view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }

    pub fn print_instance_layer_properties(&self) {
        let layer_properties = self
            .entry
            .enumerate_instance_layer_properties()
            .expect("There is no available layer");
        layer_properties.iter().for_each(|x| println!("{:#?}", x));
        println!("\n");
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_sampler(self.texture_sampler, None);
            self.device
                .destroy_image_view(self.texture_image_view, None);
            self.device.free_memory(self.texture_device_memory, None);
            self.device.destroy_image(self.texture_image, None);
            for uniform_device_memory in self.uniform_device_memories {
                self.device.free_memory(uniform_device_memory, None);
            }
            for uniform_buffer in self.uniform_buffers {
                self.device.destroy_buffer(uniform_buffer, None);
            }
            self.device.free_memory(self.index_device_memory, None);
            self.device.destroy_buffer(self.index_buffer, None);
            self.device.free_memory(self.vertex_device_memory, None);
            self.device.destroy_buffer(self.vertex_buffer, None);
            for fence in self.fences {
                self.device.destroy_fence(fence, None);
            }
            for semaphores_ in self.semaphores {
                for semaphore in semaphores_ {
                    self.device.destroy_semaphore(semaphore, None);
                }
            }
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
