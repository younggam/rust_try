use super::{elements::*, graphics::*};

use std::{collections::HashMap, sync::Arc};

use winit::window::WindowId;

use wgpu::util::DeviceExt;

use cgmath::*;

pub struct Uniform {}

impl Uniform {
    pub fn bind_group_layout_entry(binding: &mut u32) -> wgpu::BindGroupLayoutEntry {
        let ret = wgpu::BindGroupLayoutEntry {
            binding: *binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(unsafe { std::num::NonZeroU64::new_unchecked(64) }),
            },
            count: None,
        };
        *binding += 1;
        ret
    }
}

pub struct BindGroup {
    inner: wgpu::BindGroup,
    buffers: Vec<wgpu::Buffer>,
}

///window, surface 정보, render_pipeline 별 batch.
pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,

    batch: Batch,

    view_projection_buffer: wgpu::Buffer,
    view_projection_bind_group: wgpu::BindGroup,

    graphics_core: Arc<GraphicsCore>,
}

impl Renderer {
    pub fn new(graphics: &Graphics, target_window_id: WindowId) -> Self {
        let view_projection_bind_group_layout =
            graphics
                .core
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("View Projection Bind Group Layout"),
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
            bytemuck::cast_slice(AsRef::<[[f32; 4]; 4]>::as_ref(&view_proj_matrix)),
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
    pub(super) fn new(graphics_core: &GraphicsCore) -> Self {
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
    pub(super) fn batch(
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
