use super::elements::*;
use crate::utils::LazyManual;

use std::collections::HashMap;
use std::sync::Mutex;

use winit::window::Window;

use wgpu::util::DeviceExt;

use cgmath::*;

pub(crate) static INSTANCE: LazyManual<wgpu::Instance> = LazyManual::new();
pub(crate) static DEVICE: LazyManual<wgpu::Device> = LazyManual::new();
pub(crate) static QUEUE: LazyManual<wgpu::Queue> = LazyManual::new();

pub(crate) fn init(window: Window) -> WindowAndSurface {
    INSTANCE.init(wgpu::Instance::new(wgpu::Backends::VULKAN));
    let surface = unsafe { INSTANCE.create_surface(&window) };

    let adapter = pollster::block_on(INSTANCE.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }))
    .unwrap();

    match pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Initial Device"),
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None,
    )) {
        Ok((device, queue)) => {
            DEVICE.init(device);
            QUEUE.init(queue);
        }
        Err(e) => panic!("{:?}", e),
    };

    let size = window.inner_size();
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface
            .get_preferred_format(&adapter)
            .expect("Surface is incompatible with the adapter"),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&DEVICE, &surface_config);

    let depth_texture = Texture::create_depth_texture(&surface_config, "Depth Texture");

    WindowAndSurface {
        window,
        surface,
        surface_config: Mutex::new(surface_config),

        surface_texture: None,
        surface_texture_view: Mutex::new(None),

        depth_texture: Mutex::new(depth_texture),
    }
}

pub struct WindowAndSurface {
    window: Window,
    surface: wgpu::Surface,
    surface_config: Mutex<wgpu::SurfaceConfiguration>,

    surface_texture: Option<wgpu::SurfaceTexture>,
    surface_texture_view: Mutex<Option<wgpu::TextureView>>,

    depth_texture: Mutex<Texture>,
}

impl WindowAndSurface {
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let window_size = self.window.inner_size();

        if new_size.width > 0 && new_size.height > 0 && window_size == new_size {
            let mut surface_config = self.surface_config.lock().unwrap();
            surface_config.width = new_size.width;
            surface_config.height = new_size.height;

            self.surface_texture = None;
            *self.surface_texture_view.lock().unwrap() = None;

            self.surface.configure(&DEVICE, &surface_config);

            *self.depth_texture.lock().unwrap() =
                Texture::create_depth_texture(&surface_config, "depth_texture");
        }
    }

    pub fn update(&mut self) {
        if let Some(_) = self.surface_texture {
            return;
        }

        self.surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => {
                *self.surface_texture_view.lock().unwrap() = Some(
                    surface_texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                );

                Some(surface_texture)
            }
            Err(wgpu::SurfaceError::Outdated) => {
                self.surface
                    .configure(&DEVICE, &self.surface_config.lock().unwrap());

                Some(
                    self.surface
                        .get_current_texture()
                        .expect("Error reconfiguring surface"),
                )
            }
            err => Some(err.expect("Failed to acquire next swap chain texture!")),
        };
    }

    pub fn present(&mut self) {
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.present();
        }
        *self.surface_texture_view.lock().unwrap() = None;
    }
}

///Uses Instancing not Dynamic Batching.
pub struct Batch {
    window_and_surface: WindowAndSurface,

    render_pipeline: wgpu::RenderPipeline,

    mesh_buffers: HashMap<u32, MeshBuffer>,
    to_draw: Vec<u32>,

    instances: HashMap<u32, Vec<Instance>>,
    instance_buffer: wgpu::Buffer,

    last_mesh_id: Option<u32>,
}

impl Batch {
    pub fn new(window: Window) -> Self {
        let window_and_surface = init(window);

        let render_pipeline_layout =
            DEVICE.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let shader = DEVICE.create_shader_module(&include_wgsl!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/shaders/shader.wgsl"
        )));

        let render_pipeline = DEVICE.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                    format: window_and_surface.surface_config.lock().unwrap().format,
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

        let instance_buffer = DEVICE.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&[0; 4 * 16 * 1024]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            window_and_surface,

            render_pipeline,

            mesh_buffers: HashMap::new(),
            to_draw: Vec::new(),

            instances: HashMap::new(),
            instance_buffer,

            last_mesh_id: None,
        }
    }

    pub fn window_and_surface_mut(&mut self) -> &mut WindowAndSurface {
        &mut self.window_and_surface
    }

    pub fn window_and_surface(&self) -> &WindowAndSurface {
        &self.window_and_surface
    }

    pub fn draw(
        &mut self,
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
                    self.mesh_buffers.insert(mesh_id, mesh.to_buffer());
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

    pub fn flush(&mut self) {
        let surface_texture_view = self.window_and_surface.surface_texture_view.lock().unwrap();
        let surface_texture_view = match *surface_texture_view {
            Some(ref surface_texture_view) => surface_texture_view,
            _ => unsafe {
                debug_assert!(false, "Attempted to use None value.");
                std::hint::unreachable_unchecked()
            },
        };

        let mut encoder = DEVICE.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let depth_texture_view = &self.window_and_surface.depth_texture.lock().unwrap().view;
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
                    view: depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            let mut instance_start = 0u64;
            for mesh_id in self.to_draw.drain(..) {
                let mesh_buffer = match self.mesh_buffers.get(&mesh_id) {
                    Some(mesh_buffer) => mesh_buffer,
                    _ => unsafe {
                        debug_assert!(false, "Attempted to use empty value.");
                        std::hint::unreachable_unchecked()
                    },
                };

                let instances = match self.instances.remove(&mesh_id) {
                    Some(instances) => instances,
                    _ => unsafe {
                        debug_assert!(false, "Attempted to use empty value.");
                        std::hint::unreachable_unchecked()
                    },
                };

                QUEUE.write_buffer(
                    &self.instance_buffer,
                    instance_start as wgpu::BufferAddress,
                    bytemuck::cast_slice(&instances),
                );

                render_pass.set_index_buffer(
                    mesh_buffer.index_buffer().slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.set_vertex_buffer(0, mesh_buffer.vertex_buffer().slice(..));

                let instance_end = instance_start + instances.len() as wgpu::BufferAddress * 4 * 16;
                render_pass
                    .set_vertex_buffer(1, self.instance_buffer.slice(instance_start..instance_end));

                render_pass.draw_indexed(
                    0..mesh_buffer.indices_count() as u32,
                    0,
                    0..instances.len() as u32,
                );

                instance_start = instance_end;
            }
        }

        QUEUE.submit(std::iter::once(encoder.finish()));
        self.instances.clear();
    }

    pub fn present(&mut self) {
        self.flush();
        self.window_and_surface.present();
    }
}
