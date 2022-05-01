use super::elements::*;

use std::{collections::hash_map::*, ops::Deref, sync::Arc};

use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder, WindowId},
};

pub struct GraphicsConfig {
    pub title: &'static str,
}

pub(super) struct GraphicsCore {
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub adapter: wgpu::Adapter,
    pub instance: wgpu::Instance,
}

///title should be integrated to graphics config later.
pub struct Graphics {
    window_surfaces: HashMap<WindowId, WindowSurface>,
    primary_window_id: Option<WindowId>,

    pub(super) core: Arc<GraphicsCore>,

    config: GraphicsConfig,
}

impl Graphics {
    pub(crate) async fn new(
        config: GraphicsConfig,
        event_loop: &EventLoopWindowTarget<()>,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

        let window = WindowBuilder::new()
            .with_title(config.title)
            .build(event_loop)
            .unwrap();
        let window_id = window.id();

        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Initial Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let window_size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_preferred_format(&adapter)
                .expect("Surface is incompatible with the adapter"),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let depth_texture =
            Texture::create_depth_texture(&device, &surface_config, "Depth Texture");

        let mut window_surfaces = HashMap::new();
        window_surfaces.insert(
            window.id(),
            WindowSurface {
                window,

                surface,
                surface_config,

                surface_texture: None,
                surface_texture_view: None,

                depth_texture,
            },
        );

        Self {
            config,

            core: Arc::new(GraphicsCore {
                instance,
                adapter,
                device,
                queue,
            }),

            primary_window_id: Some(window_id),
            window_surfaces,
        }
    }

    pub fn primary_window_id(&self) -> Option<WindowId> {
        self.primary_window_id
    }

    pub fn set_primary_window_id(&mut self, window_id: WindowId) {
        if self.window_surfaces.contains_key(&window_id) {
            self.primary_window_id = Some(window_id);
        }
    }

    pub fn window_ids(&self) -> Vec<&WindowId> {
        self.window_surfaces.keys().collect()
    }

    pub(super) fn window_surface(&self, window_id: WindowId) -> Option<&WindowSurface> {
        self.window_surfaces.get(&window_id)
    }

    pub fn aspect(&self, window_id: WindowId) -> f32 {
        let size = self
            .window_surfaces
            .get(&window_id)
            .unwrap()
            .inner_size()
            .cast::<f32>();

        size.width / size.height
    }
}

impl Graphics {
    pub fn add_window(&mut self, event_loop: &EventLoopWindowTarget<()>) -> Result<WindowId, ()> {
        let window = WindowBuilder::new()
            .with_title(self.config.title)
            .build(event_loop)
            .unwrap();

        let surface = unsafe { self.core.instance.create_surface(&window) };
        let is_surface_supported = self.core.adapter.is_surface_supported(&surface);
        if !is_surface_supported {
            return Err(());
        }
        let window_size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_preferred_format(&self.core.adapter)
                .expect("Surface is incompatible with the adapter"),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&self.core.device, &surface_config);

        let depth_texture =
            Texture::create_depth_texture(&self.core.device, &surface_config, "Depth Texture");

        let window_id = window.id();
        self.window_surfaces.insert(
            window_id,
            WindowSurface {
                window,

                surface,
                surface_config,

                surface_texture: None,
                surface_texture_view: None,

                depth_texture,
            },
        );
        Ok(window_id)
    }

    pub fn remove_window(&mut self, window_id: WindowId) {
        if Some(window_id) == self.primary_window_id {
            self.primary_window_id = None;
        }
        self.window_surfaces.remove(&window_id);
    }

    ///윈도우에 맞는 resize
    pub(crate) fn resize(&mut self, window_id: WindowId, new_size: winit::dpi::PhysicalSize<u32>) {
        let window_surface = match self.window_surfaces.get_mut(&window_id) {
            Some(window_surface) => window_surface,
            None => return,
        };
        let window_size = window_surface.inner_size();

        if new_size.width > 0 && new_size.height > 0 && window_size == new_size {
            window_surface.surface_config.width = new_size.width;
            window_surface.surface_config.height = new_size.height;

            window_surface.surface_texture = None;
            window_surface.surface_texture_view = None;

            window_surface
                .surface
                .configure(&self.core.device, &window_surface.surface_config);

            window_surface.depth_texture = Texture::create_depth_texture(
                &self.core.device,
                &window_surface.surface_config,
                "depth_texture",
            );
        }
    }

    pub fn update(&mut self) {
        for window_surface in self.window_surfaces.values_mut() {
            window_surface.request_redraw();

            if let Some(_) = window_surface.surface_texture {
                continue;
            }

            window_surface.surface_texture = match window_surface.surface.get_current_texture() {
                Ok(surface_texture) => {
                    window_surface.surface_texture_view = Some(
                        surface_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    );

                    Some(surface_texture)
                }
                Err(wgpu::SurfaceError::Outdated) => {
                    window_surface
                        .surface
                        .configure(&self.core.device, &window_surface.surface_config);

                    Some(
                        window_surface
                            .surface
                            .get_current_texture()
                            .expect("Error reconfiguring surface"),
                    )
                }
                e => panic!("Failed to acquire next swap chain texture!\n{:?}", e),
            };
        }
    }

    pub fn present(&mut self) {
        for window_surface in self.window_surfaces.values_mut() {
            if let Some(surface_texture) = window_surface.surface_texture.take() {
                surface_texture.present();
            }
            window_surface.surface_texture_view = None;
        }
    }
}

pub(super) struct WindowSurface {
    pub depth_texture: Texture,

    pub surface_texture_view: Option<wgpu::TextureView>,
    pub surface_texture: Option<wgpu::SurfaceTexture>,

    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,

    pub window: Window,
}

impl Deref for WindowSurface {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
