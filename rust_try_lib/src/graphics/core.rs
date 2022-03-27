use super::elements::*;
use super::window::Window;

use std::collections::HashMap;

use wgpu::util::DeviceExt;

use cgmath::*;

///Uses Instancing not Dynamic Batching.
pub struct GraphicsCore {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,

    render_pipeline: wgpu::RenderPipeline,

    depth_texture: Texture,
    surface_texture: Option<wgpu::SurfaceTexture>,
    surface_texture_view: Option<wgpu::TextureView>,

    mesh_datas: HashMap<u32, MeshData>,
    to_draw: Vec<u32>,

    instances: HashMap<u32, Vec<Instance>>,
    instance_buffer: wgpu::Buffer,

    last_mesh_id: Option<u32>,
}

impl GraphicsCore {
    pub(crate) async fn new<W: Window>(window: &W) -> Self {
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

        let size = window.inner_size();
        println!("{:#?}", size);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_preferred_format(&adapter)
                .expect("Surface is incompatible with the adapter"),
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(&include_wgsl!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/shaders/shader.wgsl"
        )));

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                    format: surface_config.format,
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

        let depth_texture =
            Texture::create_depth_texture(&device, &surface_config, "Depth Texture");

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&[0; 4 * 16 * 1024]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            surface,
            surface_config,

            device,
            queue,

            render_pipeline,

            depth_texture,
            surface_texture: None,
            surface_texture_view: None,

            mesh_datas: HashMap::new(),
            to_draw: Vec::new(),

            instances: HashMap::new(),
            instance_buffer,

            last_mesh_id: None,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
            self.depth_texture =
                Texture::create_depth_texture(&self.device, &self.surface_config, "depth_texture");
        }
    }

    pub fn update(&mut self) {
        if let None = self.surface_texture {
            self.surface_texture = match self.surface.get_current_texture() {
                Ok(surface_texture) => {
                    self.surface_texture_view = Some(
                        surface_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    );

                    Some(surface_texture)
                }
                Err(wgpu::SurfaceError::Outdated) => {
                    self.surface.configure(&self.device, &self.surface_config);

                    Some(
                        self.surface
                            .get_current_texture()
                            .expect("Error reconfiguring surface"),
                    )
                }
                err => Some(err.expect("Failed to acquire next swap chain texture!")),
            };
        }
    }

    pub fn draw(&mut self, mesh: &Mesh, position: Vector3<f32>, rotation: Quaternion<f32>) {
        let mesh_id = mesh.id();

        match self.last_mesh_id {
            Some(last_mesh_id) if last_mesh_id == mesh_id => {}
            _ => {
                if !self.mesh_datas.contains_key(&mesh_id) {
                    self.mesh_datas.insert(
                        mesh_id,
                        MeshData::new(&self.device, mesh.vertices(), mesh.indices()),
                    );
                }
            }
        }

        let instance = Instance::new(position, rotation);
        if let Some(value) = self.instances.get_mut(&mesh_id) {
            value.push(instance);
        } else {
            self.instances.insert(mesh_id, vec![instance]);
            self.to_draw.push(mesh_id);
        }

        self.last_mesh_id = Some(mesh_id);
    }

    pub fn flush(&mut self) {
        let surface_texture_view = match self.surface_texture_view {
            Some(ref mut surface_texture_view) => surface_texture_view,
            _ => unsafe { std::hint::unreachable_unchecked() },
        };

        let mut encoder = self
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
                    view: &self.depth_texture.view,
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
                let mesh_data = match self.mesh_datas.get(&mesh_id) {
                    Some(mesh_data) => mesh_data,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let instances = match self.instances.remove(&mesh_id) {
                    Some(instances) => instances,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                self.queue.write_buffer(
                    &self.instance_buffer,
                    instance_start as wgpu::BufferAddress,
                    bytemuck::cast_slice(&instances),
                );

                render_pass
                    .set_index_buffer(mesh_data.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                render_pass.set_vertex_buffer(0, mesh_data.vertex_buffer.slice(..));

                let instance_end = instance_start + instances.len() as wgpu::BufferAddress * 4 * 16;
                render_pass
                    .set_vertex_buffer(1, self.instance_buffer.slice(instance_start..instance_end));

                render_pass.draw_indexed(
                    0..mesh_data.indices_count as u32,
                    0,
                    0..instances.len() as u32,
                );

                instance_start = instance_end;
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.instances.clear();
    }

    pub fn render(&mut self) {
        self.flush();
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.present();
        }
        self.surface_texture_view = None;
    }
}

pub struct MeshData {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices_count: usize,
}

impl MeshData {
    pub fn new(device: &wgpu::Device, vertices: &[ColorVertex], indices: &[u32]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            indices_count: indices.len(),
        }
    }
}
