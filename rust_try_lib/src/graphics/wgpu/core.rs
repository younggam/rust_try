use crate::graphics::window::Window;
use crate::graphics::GraphicsCore;

pub struct GraphicsCoreWgpu {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,

    device: wgpu::Device,
    queue: wgpu::Queue,
    // render_pipeline: wgpu::RenderPipeline,
    // obj_model: model::Model,
    // camera: Camera,
    // camera_controller: CameraController,
    // camera_uniform: CameraUniform,
    // camera_buffer: wgpu::Buffer,
    // camera_bind_group: wgpu::BindGroup,
    // instances: Vec<Instance>,
    // instance_buffer: wgpu::Buffer,
    // depth_texture: texture::Texture,
    // light_uniform: LightUniform,
    // light_buffer: wgpu::Buffer,
    // light_bind_group: wgpu::BindGroup,
    // light_render_pipeline: wgpu::RenderPipeline,
}

impl GraphicsCoreWgpu {
    pub async fn new<W: Window>(window: &W) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        Self {
            surface,
            surface_config,

            device,
            queue,
            // render_pipeline,
        }
    }
}

impl GraphicsCore for GraphicsCoreWgpu {}
