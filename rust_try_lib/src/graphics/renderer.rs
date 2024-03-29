use super::{elements::*, graphics::*};

use std::{collections::HashMap, num::*, sync::Arc};

use winit::window::WindowId;

use wgpu::util::DeviceExt;

use cgmath::*;

// pub trait BindGroupEntry{}
//
// pub struct BindGroup {
//     pub name: &'static str,
//     pub entries: Vec<Box<BindGroupEntry>>,
// }

pub enum BindingResource {
    Uniform {
        buffer: wgpu::Buffer,
        size: wgpu::BufferAddress,
    },
    Texture {
        texture: wgpu::Texture,
        view_desc: wgpu::TextureViewDescriptor<'static>,
    },
    Sampler(wgpu::Sampler),
}

pub struct BindGroupConfigEntry {
    pub name: &'static str,
    pub binding: u32,
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub count: Option<NonZeroU32>,
}

pub struct BindGroupConfig {
    pub name: &'static str,
    pub entries: Vec<BindGroupConfigEntry>,
}

///window, surface 정보, render_pipeline 별 batch.
pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,

    batch: Batch,

    bind_buffers: Vec<Vec<wgpu::Buffer>>,
    bind_groups: Vec<wgpu::BindGroup>,

    graphics_core: Arc<GraphicsCore>,
}

impl Renderer {
    pub fn new(
        graphics: &Graphics,
        target_window_id: WindowId,
        bind_group_configs: &[BindGroupConfig],
    ) -> Self {
        let bind_group_layouts = bind_group_configs
            .iter()
            .map(|bind_group_config| {
                graphics
                    .core
                    .device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some(&(bind_group_config.name.to_string() + " Bind Group Layout")),
                        entries: &bind_group_config
                            .entries
                            .iter()
                            .map(|entry| wgpu::BindGroupLayoutEntry {
                                binding: entry.binding,
                                visibility: entry.visibility,
                                ty: entry.ty,
                                count: entry.count,
                            })
                            .collect::<Vec<wgpu::BindGroupLayoutEntry>>(),
                    })
            })
            .collect::<Vec<_>>();

        let render_pipeline_layout =
            graphics
                .core
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &bind_group_layouts.iter().collect::<Vec<_>>(),
                    push_constant_ranges: &[],
                });

        let shader = graphics
            .core
            .device
            .create_shader_module(&include_wgsl!(concat!(
                env!("CARGO_MANIFEST_DIR"),
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
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
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
                    multiview: None,
                });

        let bind_buffers = bind_group_configs
            .iter()
            .map(|bind_group_config| {
                bind_group_config
                    .entries
                    .iter()
                    .map(|entry| {
                        graphics
                            .core
                            .device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some(&(entry.name.to_string() + " Buffer")),
                                contents: &[0; 4 * 16 * 1],
                                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                            })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let bind_groups = bind_group_configs
            .iter()
            .enumerate()
            .map(|(i, bind_group_config)| {
                graphics
                    .core
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some(&(bind_group_config.name.to_string() + " Bind Group")),
                        layout: &bind_group_layouts[i],
                        entries: &bind_buffers[i]
                            .iter()
                            .enumerate()
                            .map(|(j, buffer)| wgpu::BindGroupEntry {
                                binding: bind_group_config.entries[j].binding,
                                resource: buffer.as_entire_binding(),
                            })
                            .collect::<Vec<_>>(),
                    })
            })
            .collect::<Vec<_>>();

        Self {
            graphics_core: graphics.core.clone(),
            render_pipeline,

            batch: Batch::new(&graphics.core),

            bind_buffers,
            bind_groups,
        }
    }
}

impl Renderer {
    pub fn render(
        &mut self,
        graphics: &Graphics,
        target_window_id: WindowId,
        bind_resources: &[&[Option<Matrix4<f32>>]],
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

        for (i, bind_resources) in bind_resources.iter().enumerate() {
            for (j, bind_resource) in bind_resources.iter().enumerate() {
                if let Some(bind_resource) = bind_resource {
                    self.graphics_core.queue.write_buffer(
                        &self.bind_buffers[i][j],
                        0,
                        bytemuck::cast_slice(AsRef::<[[f32; 4]; 4]>::as_ref(bind_resource)),
                    );
                }
            }
        }

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
            for (i, bind_group) in self.bind_groups.iter().enumerate() {
                render_pass.set_bind_group(i as u32, bind_group, &[]);
            }

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
