use super::elements::*;

use std::{collections::hash_map::*, ops::Deref, sync::Arc};

use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder, WindowId},
};

use wgpu::util::DeviceExt;

use cgmath::*;

pub struct GraphicsConfig {
    pub title: &'static str,
}

///title should be integrated to graphics config later.
pub struct Graphics {
    config: GraphicsConfig,

    core: Arc<GraphicsCore>,

    primary_window_id: Option<WindowId>,
    window_surfaces: HashMap<WindowId, WindowSurface>,
}

pub struct GraphicsCore {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
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

    pub fn core(&self) -> &Arc<GraphicsCore> {
        &self.core
    }

    pub fn primary_window_id(&self) -> Option<WindowId> {
        self.primary_window_id
    }

    pub fn set_primary_window_id(&mut self, window_id: WindowId) {
        if self.window_surfaces.contains_key(&window_id) {
            self.primary_window_id = Some(window_id);
        }
    }

    pub fn primary_window_surface(&self) -> Option<&WindowSurface> {
        match self.primary_window_id {
            Some(primary_window_id) => self.window_surfaces.get(&primary_window_id),
            _ => None,
        }
    }

    pub fn window_ids(&self) -> Keys<'_, WindowId, WindowSurface> {
        self.window_surfaces.keys()
    }

    pub fn window_surface(&self, window_id: WindowId) -> Option<&WindowSurface> {
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

pub struct WindowSurface {
    window: Window,

    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,

    surface_texture: Option<wgpu::SurfaceTexture>,
    surface_texture_view: Option<wgpu::TextureView>,

    depth_texture: Texture,
}

impl Deref for WindowSurface {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::from_cols(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, 1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 0.5, 0.0),
    vec4(0.0, 0.0, 0.5, 1.0),
);

// pub struct RenderPassColorAttachment {
//     ops: wgpu::Operations<wgpu::Color>,
// }
//
// pub struct RenderPassDepthStencilAttachment {
//     depth_ops: Option<wgpu::Operations<f32>>,
//     stencil_ops: Option<wgpu::Operations<u32>>,
// }
//
// pub struct _RenderPassDescriptor {
//     label: Option<&'static str>,
//     color_attachments: Vec<RenderPassColorAttachment>,
//     depth_stencil_attachment: Option<RenderPassDepthStencilAttachment>,
// }
//
// impl RenderPassDescriptor {
//     pub fn to_real(&self)->wgpu::RenderPassDescriptor{
//
//     }
// }

///window, surface 정보, render_pipeline 별 batch.
pub struct Renderer {
    graphics_core: Arc<GraphicsCore>,
    // render_pass_descriptor: RenderPassDescriptor,
    render_pipeline: wgpu::RenderPipeline,

    batch: Batch,

    view_projection_buffer: wgpu::Buffer,
    view_projection_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(graphics: &Graphics, target_window_id: WindowId) -> Self {
        let view_projection_bind_group_layout =
            graphics
                .core
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(""),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: Some(unsafe {
                                std::num::NonZeroU64::new_unchecked(64)
                            }),
                        },
                        count: None,
                    }],
                });

        let render_pipeline_layout =
            graphics
                .core
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&view_projection_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let shader = graphics
            .core
            .device
            .create_shader_module(&include_wgsl!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                // "/../assets/shaders/shader.wgsl"
                "/../assets/shaders/view_projection.wgsl"
            )));

        let render_pipeline =
            graphics
                .core
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[ColorVertex::buffer_layout(), Instance::buffer_layout()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[wgpu::ColorTargetState {
                            format: graphics
                                .window_surface(target_window_id)
                                .expect("Target window doesn't exist")
                                .surface_config
                                .format,
                            blend: Some(wgpu::BlendState {
                                alpha: wgpu::BlendComponent::REPLACE,
                                color: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        }],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    // If the pipeline will be used with a multiview render pass, this
                    // indicates how many array layers the attachments will have.
                    multiview: None,
                });

        let view_projection_buffer =
            graphics
                .core
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("View Projection Buffer"),
                    contents: &[0; 4 * 16 * 1],
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let view_projection_bind_group =
            graphics
                .core
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("View Projection Bind Group"),
                    layout: &view_projection_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: view_projection_buffer.as_entire_binding(),
                    }],
                });

        Self {
            graphics_core: graphics.core.clone(),
            // render_pass_descriptor,
            render_pipeline,

            batch: Batch::new(&graphics.core),

            view_projection_buffer,
            view_projection_bind_group,
        }
    }
}

impl Renderer {
    pub fn render(
        &mut self,
        graphics: &Graphics,
        target_window_id: WindowId,
        view_proj_matrix: Matrix4<f32>,
    ) -> Result<(), ()> {
        let window_surface = match graphics.window_surface(target_window_id) {
            Some(window_surface) => window_surface,
            _ => return Err(()),
        };

        let surface_texture_view = match window_surface.surface_texture_view {
            Some(ref surface_texture_view) => surface_texture_view,
            _ => unsafe {
                debug_assert!(false, "Attempted to use None value.");
                std::hint::unreachable_unchecked()
            },
        };

        self.graphics_core.queue.write_buffer(
            &self.view_projection_buffer,
            0,
            bytemuck::cast_slice(AsRef::<[[f32; 4]; 4]>::as_ref(
                &(OPENGL_TO_WGPU_MATRIX * view_proj_matrix),
            )),
        );

        let mut encoder =
            self.graphics_core
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.05,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &window_surface.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.view_projection_bind_group, &[]);

            let mut instance_start = 0u64;
            for mesh_id in self.batch.to_draw.drain(..) {
                let mesh_buffer = match self.batch.mesh_buffers.get(&mesh_id) {
                    Some(mesh_buffer) => mesh_buffer,
                    _ => unsafe {
                        debug_assert!(false, "Attempted to use empty value.");
                        std::hint::unreachable_unchecked()
                    },
                };

                let instances = match self.batch.instances.remove(&mesh_id) {
                    Some(instances) => instances,
                    _ => unsafe {
                        debug_assert!(false, "Attempted to use empty value.");
                        std::hint::unreachable_unchecked()
                    },
                };

                self.graphics_core.queue.write_buffer(
                    &self.batch.instance_buffer,
                    instance_start as wgpu::BufferAddress,
                    bytemuck::cast_slice(&instances),
                );

                render_pass.set_index_buffer(
                    mesh_buffer.index_buffer().slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.set_vertex_buffer(0, mesh_buffer.vertex_buffer().slice(..));

                let instance_end = instance_start + instances.len() as wgpu::BufferAddress * 4 * 16;
                render_pass.set_vertex_buffer(
                    1,
                    self.batch
                        .instance_buffer
                        .slice(instance_start..instance_end),
                );

                render_pass.draw_indexed(
                    0..mesh_buffer.indices_count() as u32,
                    0,
                    0..instances.len() as u32,
                );

                instance_start = instance_end;
            }
        }

        self.graphics_core
            .queue
            .submit(std::iter::once(encoder.finish()));
        self.batch.instances.clear();

        Ok(())
    }

    pub fn batch(
        &mut self,
        mesh: &Mesh,
        position: Point3<f32>,
        rotation: Quaternion<f32>,
        scale: Vector3<f32>,
    ) {
        self.batch
            .batch(&self.graphics_core, mesh, position, rotation, scale);
    }
}

///Uses Instancing not Dynamic Batching.
pub struct Batch {
    mesh_buffers: HashMap<u32, MeshBuffer>,
    to_draw: Vec<u32>,

    instances: HashMap<u32, Vec<Instance>>,
    instance_buffer: wgpu::Buffer,

    last_mesh_id: Option<u32>,
}

impl Batch {
    pub fn new(graphics_core: &GraphicsCore) -> Self {
        let instance_buffer =
            graphics_core
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(&[0; 4 * 16 * 1024]),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

        Self {
            mesh_buffers: HashMap::new(),
            to_draw: Vec::new(),

            instances: HashMap::new(),
            instance_buffer,

            last_mesh_id: None,
        }
    }
}

impl Batch {
    pub fn batch(
        &mut self,
        graphics_core: &GraphicsCore,
        mesh: &Mesh,
        position: Point3<f32>,
        rotation: Quaternion<f32>,
        scale: Vector3<f32>,
    ) {
        let mesh_id = mesh.id();

        match self.last_mesh_id {
            Some(last_mesh_id) if last_mesh_id == mesh_id => {}
            _ => {
                if !self.mesh_buffers.contains_key(&mesh_id) {
                    self.mesh_buffers
                        .insert(mesh_id, mesh.to_buffer(&graphics_core.device));
                }
            }
        }

        let instance = Instance::new(position, rotation, scale);
        if let Some(value) = self.instances.get_mut(&mesh_id) {
            value.push(instance);
        } else {
            self.instances.insert(mesh_id, vec![instance]);
            self.to_draw.push(mesh_id);
        }

        self.last_mesh_id = Some(mesh_id);
    }
}
