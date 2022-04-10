use super::elements::*;

use std::collections::HashMap;
use std::sync::Arc;

use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder, WindowId},
};

use wgpu::util::DeviceExt;

use cgmath::*;

pub struct GraphicsConfig {
    title: &'static str,
}

///title should be integrated to graphics config later.
pub struct Graphics {
    config: GraphicsConfig,

    core: Arc<GraphicsCore>,

    primary_window_id: WindowId,
    windows: HashMap<WindowId, Window>,
    surface_resources: HashMap<WindowId, SurfaceResource>,
}

pub struct GraphicsCore {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Graphics {
    pub(crate) async fn new(title: &'static str, event_loop: &EventLoopWindowTarget<()>) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

        let window = WindowBuilder::new()
            .with_title(title)
            .build(event_loop)
            .unwrap();
        let window_id = window.id();
        let window_size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };

        let mut windows = HashMap::new();
        windows.insert(window_id, window);

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

        let mut surface_resources = HashMap::new();
        surface_resources.insert(
            window_id,
            SurfaceResource {
                surface,
                surface_config,

                surface_texture: None,
                surface_texture_view: None,

                depth_texture,
            },
        );

        Self {
            config: GraphicsConfig { title },

            core: Arc::new(GraphicsCore {
                instance,
                adapter,
                device,
                queue,
            }),

            primary_window_id: window_id,
            windows,
            surface_resources,
        }
    }

    pub fn core(&self) -> &Arc<GraphicsCore> {
        &self.core
    }

    pub fn add_window(&mut self, event_loop: &EventLoopWindowTarget<()>) -> bool {
        let window = WindowBuilder::new()
            .with_title(self.config.title)
            .build(event_loop)
            .unwrap();
        let window_id = window.id();
        let window_size = window.inner_size();

        let surface = unsafe { self.core.instance.create_surface(&window) };
        let is_surface_supported = self.core.adapter.is_surface_supported(&surface);
        if is_surface_supported {
            self.windows.insert(window_id, window);

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

            self.surface_resources.insert(
                window_id,
                SurfaceResource {
                    surface,
                    surface_config,

                    surface_texture: None,
                    surface_texture_view: None,

                    depth_texture,
                },
            );
        }
        is_surface_supported
    }

    pub fn primary_window_id(&self) -> WindowId {
        self.primary_window_id
    }

    pub fn primary_window(&self) -> &Window {
        self.windows.get(&self.primary_window_id).unwrap()
    }

    pub fn primary_surface_resource(&self) -> &SurfaceResource {
        self.surface_resources.get(&self.primary_window_id).unwrap()
    }

    pub fn aspect(&self, window_id: WindowId) -> f32 {
        let size = self
            .windows
            .get(&window_id)
            .unwrap()
            .inner_size()
            .cast::<f32>();

        size.width / size.height
    }
}

impl Graphics {
    #[must_use]
    pub(crate) fn resize(
        &mut self,
        window_id: WindowId,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) -> bool {
        let window = self.windows.get(&window_id).unwrap();
        let surface_resource = self.surface_resources.get_mut(&window_id).unwrap();
        let window_size = window.inner_size();

        if new_size.width > 0 && new_size.height > 0 && window_size == new_size {
            surface_resource.surface_config.width = new_size.width;
            surface_resource.surface_config.height = new_size.height;

            surface_resource.surface_texture = None;
            surface_resource.surface_texture_view = None;

            surface_resource
                .surface
                .configure(&self.core.device, &surface_resource.surface_config);

            surface_resource.depth_texture = Texture::create_depth_texture(
                &self.core.device,
                &surface_resource.surface_config,
                "depth_texture",
            );

            return true;
        }
        false
    }

    pub fn update(&mut self) {
        for surface_resource in self.surface_resources.values_mut() {
            if let Some(_) = surface_resource.surface_texture {
                continue;
            }

            surface_resource.surface_texture = match surface_resource.surface.get_current_texture()
            {
                Ok(surface_texture) => {
                    surface_resource.surface_texture_view = Some(
                        surface_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    );

                    Some(surface_texture)
                }
                Err(wgpu::SurfaceError::Outdated) => {
                    surface_resource
                        .surface
                        .configure(&self.core.device, &surface_resource.surface_config);

                    Some(
                        surface_resource
                            .surface
                            .get_current_texture()
                            .expect("Error reconfiguring surface"),
                    )
                }
                e => panic!("Failed to acquire next swap chain texture!\n{:?}", e),
            };
        }

        for window in self.windows.values() {
            window.request_redraw();
        }
    }

    pub fn draw(
        &mut self,
        window_id: WindowId,
        render_pass_descriptor: impl Fn(&wgpu::TextureView) -> wgpu::RenderPassDescriptor,
        render_pipeline: &wgpu::RenderPipeline,
        to_draw: &[u32],
        mesh_buffers: &HashMap<u32, MeshBuffer>,
        instances: &HashMap<u32, Vec<Instance>>,
        instance_buffer: &wgpu::Buffer,
    ) {
        let surface_resource =
            if let Some(surface_resource) = self.surface_resources.get(&window_id) {
                surface_resource
            } else {
                return;
            };
        let surface_texture_view = match surface_resource.surface_texture_view {
            Some(ref surface_texture_view) => surface_texture_view,
            _ => unsafe {
                debug_assert!(false, "Attempted to use None value.");
                std::hint::unreachable_unchecked()
            },
        };

        // self.graphics_core.queue.write_buffer(
        //     &self.view_projection_buffer,
        //     0,
        //     bytemuck::cast_slice(AsRef::<[[f32; 4]; 4]>::as_ref(
        //         &(OPENGL_TO_WGPU_MATRIX
        //             * Matrix4::<f32>::from(self.projection)
        //             * self.camera.view_matrix()),
        //     )),
        // );
        //
        let mut encoder =
            self.core
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Draw Stage Encoder"),
                });

        {
            let mut render_pass =
                encoder.begin_render_pass(&render_pass_descriptor(&surface_texture_view));

            render_pass.set_pipeline(render_pipeline);
            // render_pass.set_bind_group(0, &self.view_projection_bind_group, &[]);

            let mut instance_start = 0u64;
            for mesh_id in to_draw {
                let mesh_buffer = match mesh_buffers.get(&mesh_id) {
                    Some(mesh_buffer) => mesh_buffer,
                    _ => unsafe {
                        debug_assert!(false, "Attempted to use empty value.");
                        std::hint::unreachable_unchecked()
                    },
                };

                let instances = match instances.get(&mesh_id) {
                    Some(instances) => instances,
                    _ => unsafe {
                        debug_assert!(false, "Attempted to use empty value.");
                        std::hint::unreachable_unchecked()
                    },
                };

                self.core.queue.write_buffer(
                    &instance_buffer,
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
                    .set_vertex_buffer(1, instance_buffer.slice(instance_start..instance_end));

                render_pass.draw_indexed(
                    0..mesh_buffer.indices_count() as u32,
                    0,
                    0..instances.len() as u32,
                );

                instance_start = instance_end;
            }
        }

        self.core.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn present(&mut self) {
        for surface_resource in self.surface_resources.values_mut() {
            if let Some(surface_texture) = surface_resource.surface_texture.take() {
                surface_texture.present();
            }
            surface_resource.surface_texture_view = None;
        }
    }
}

pub struct SurfaceResource {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,

    surface_texture: Option<wgpu::SurfaceTexture>,
    surface_texture_view: Option<wgpu::TextureView>,

    depth_texture: Texture,
}

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::from_cols(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, 1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 0.5, 0.0),
    vec4(0.0, 0.0, 0.5, 1.0),
);

/*
실질적 draw call 실행하는 것
이쪽은 batch(성능) 의 책임이 없다.

파이프라인,
텍스쳐,
버퍼,
직접 리소스 들고있는
draw call로 정리된(batch) 리소스를 넘겨야 할 책임이 있다.

외부 데이터와
그래픽스를 연결해주는
config 느낌의

pipeline
renderpass
texture
mesh
*/

///Uses Instancing not Dynamic Batching.
pub struct Batch {
    graphics_core: Arc<GraphicsCore>,

    render_pipeline: wgpu::RenderPipeline,

    mesh_buffers: HashMap<u32, MeshBuffer>,
    to_draw: Vec<u32>,

    instances: HashMap<u32, Vec<Instance>>,
    instance_buffer: wgpu::Buffer,

    last_mesh_id: Option<u32>,

    camera: Camera,
    projection: PerspectiveFov<f32>,
    view_projection_buffer: wgpu::Buffer,
    view_projection_bind_group: wgpu::BindGroup,
}

impl Batch {
    pub fn new(graphics: &Graphics) -> Self {
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
                            format: graphics.primary_surface_resource().surface_config.format,
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

        let instance_buffer =
            graphics
                .core
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(&[0; 4 * 16 * 1024]),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

        let camera = Camera::new(point3(0.0, 0.0, 5.0), vec3(0.0, 0.0, -1.0), 1.0);

        let projection = PerspectiveFov {
            fovy: Rad(std::f32::consts::FRAC_PI_4),
            aspect: graphics.aspect(graphics.primary_window_id()),
            near: 0.1,
            far: 100.0,
        };

        let view_projection_buffer =
            graphics
                .core
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("View Projection Buffer"),
                    contents: bytemuck::cast_slice(AsRef::<[[f32; 4]; 4]>::as_ref(
                        &(OPENGL_TO_WGPU_MATRIX
                            * Matrix4::<f32>::from(projection)
                            * camera.view_matrix()),
                    )),
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

            render_pipeline,

            mesh_buffers: HashMap::new(),
            to_draw: Vec::new(),

            instances: HashMap::new(),
            instance_buffer,

            last_mesh_id: None,

            camera,
            projection,
            view_projection_buffer,
            view_projection_bind_group,
        }
    }

    pub(crate) fn resize(
        &mut self,
        graphics: &mut Graphics,
        window_id: WindowId,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        if graphics.resize(window_id, new_size) {
            let new_size = new_size.cast::<f32>();
            self.projection.aspect = new_size.width / new_size.height;
        }
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
                    self.mesh_buffers
                        .insert(mesh_id, mesh.to_buffer(&self.graphics_core.device));
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

    pub fn flush(&mut self, graphics: &Graphics) {
        let surface_resource = graphics.primary_surface_resource();
        let surface_texture_view = match surface_resource.surface_texture_view {
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
                &(OPENGL_TO_WGPU_MATRIX
                    * Matrix4::<f32>::from(self.projection)
                    * self.camera.view_matrix()),
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
                    view: &surface_resource.depth_texture.view,
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

                self.graphics_core.queue.write_buffer(
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

        self.graphics_core
            .queue
            .submit(std::iter::once(encoder.finish()));
        self.instances.clear();
    }
}

//

pub struct Transform {
    position: Point3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
}

impl Transform {
    pub fn new(position: Point3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn r#move(&mut self, velocity: Vector3<f32>) {
        self.position += velocity;
    }

    pub fn rotate(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation * self.rotation;
    }

    pub fn scale_adjust(&mut self, scale: Vector3<f32>) {
        self.scale += scale;
    }
}

impl From<Point3<f32>> for Transform {
    fn from(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Quaternion::one(),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}

impl From<Quaternion<f32>> for Transform {
    fn from(rotation: Quaternion<f32>) -> Self {
        Self {
            position: point3(0.0, 0.0, 0.0),
            rotation: rotation,
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}

impl From<Vector3<f32>> for Transform {
    fn from(scale: Vector3<f32>) -> Self {
        Self {
            position: point3(0.0, 0.0, 0.0),
            rotation: Quaternion::one(),
            scale,
        }
    }
}

//

pub struct Camera {
    transform: Transform,
    speed: f32,
}

impl Camera {
    const FRONT: Vector3<f32> = vec3(1.0, 0.0, 0.0);

    pub fn new(position: Point3<f32>, front: Vector3<f32>, speed: f32) -> Self {
        Self {
            transform: Transform::new(
                position,
                Quaternion::from_arc(Self::FRONT, front, None),
                vec3(1.0, 1.0, 1.0),
            ),
            speed,
        }
    }

    pub fn position(&self) -> Point3<f32> {
        self.transform.position
    }

    pub fn rotation(&self) -> Quaternion<f32> {
        self.transform.rotation
    }

    pub fn scale(&self) -> Vector3<f32> {
        self.transform.scale
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position(),
            self.rotation().rotate_vector(Self::FRONT),
            vec3(0.0, 1.0, 0.0),
        )
    }
}
