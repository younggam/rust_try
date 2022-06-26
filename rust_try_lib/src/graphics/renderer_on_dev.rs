use super::graphics::*;

use std::{collections::HashMap, num::NonZeroU32, sync::Arc};

pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub count: Option<NonZeroU32>,
}

pub struct BindGroupLayoutDescriptor {
    pub name: &'static str,
    pub entries: Vec<BindGroupLayoutEntry>,
}

pub struct PipelineLayoutDescriptor {
    pub name: &'static str,
    pub bind_group_layouts: Vec<usize>,
    pub push_constant_ranges: Vec<wgpu::PushConstantRange>,
}

pub struct ShaderModuleDescriptor<'a> {
    pub name: &'static str,
    pub source: &'a str,
}

pub struct _VertexState{
    pub module:&'static str,
    pub entry_point:&'static str,
    // pub buffers:
}

// pub struct RenderPipelineDescriptor{
//     pub name: String,
//     pub layout: Option<usize>,
//     pub vertex: VertexState<'a>,
//     pub primitive: PrimitiveState,
//     pub depth_stencil: Option<DepthStencilState>,
//     pub multisample: MultisampleState,
//     pub fragment: Option<FragmentState<'a>>,
//     pub multiview: Option<NonZeroU32>,
// }

/*
wgpu 객체들(pipeline, buffer 등) 풀을 소유
사용자의 요청에 따라 중간에서 중개하며 반환
*/
pub struct WgpuObjectAgency {
    core: Arc<GraphicsCore>,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    pipeline_layouts: Vec<wgpu::PipelineLayout>,
    shader_modules: HashMap<&'static str, wgpu::ShaderModule>,
    render_pipeline: Vec<wgpu::RenderPipeline>,
}

impl WgpuObjectAgency {
    pub fn add_bind_group_layout(&mut self, descriptor: BindGroupLayoutDescriptor) -> usize {
        let layout = self
            .core
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&(descriptor.name.to_string() + " Bind Group Layout")),
                entries: &descriptor
                    .entries
                    .iter()
                    .map(|entry| wgpu::BindGroupLayoutEntry {
                        binding: entry.binding,
                        visibility: entry.visibility,
                        ty: entry.ty,
                        count: entry.count,
                    })
                    .collect::<Vec<wgpu::BindGroupLayoutEntry>>(),
            });
        self.bind_group_layouts.push(layout);
        self.bind_group_layouts.len() - 1
    }

    pub fn add_pipeline_layout(&mut self, descriptor: PipelineLayoutDescriptor) -> usize {
        let layout = self
            .core
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&(descriptor.name.to_string() + "Pipeline Layout")),
                bind_group_layouts: &descriptor
                    .bind_group_layouts
                    .iter()
                    .map(|id| self.bind_group_layouts.get(*id).unwrap())
                    .collect::<Vec<_>>(),
                push_constant_ranges: &descriptor.push_constant_ranges,
            });
        self.pipeline_layouts.push(layout);
        self.pipeline_layouts.len() - 1
    }

    pub fn add_shader_module(&mut self, descriptor: ShaderModuleDescriptor) {
        let shader = self
            .core
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(&(descriptor.name.to_string() + " Shader Module")),
                source: wgpu::ShaderSource::Wgsl(descriptor.source.into()),
            });
        self.shader_modules.insert(descriptor.name, shader);
    }

    // pub fn add_render_pipeline(&mut self, descriptor: RenderPipelineDescriptor) -> usize {
    //     let pipeline = self.core.device.create_render_pipeline();
    //     self.render_pipelines.push(pipeline);
    //     self.render_pipelines.len() - 1
    // }
}

pub struct _BindGroupLayouts {
    _layouts: Vec<wgpu::BindGroupLayout>,
}

pub struct PipelineLayoutPool {}

pub struct ShaderPool {}

/*
RenderPipelineDescriptor 를 사용자가 제공
내부 풀에 검색 후 없으면 Pipeline 생성, 저장 및 id 생성
id 사용자에게 반환
*/
pub struct PipelinePool {}

pub struct BindGroupPool {}

/*
미리 어느정도의 Buffer 생성
size및 index, vertex data를 사용자가 제공
새 Buffer 생성 및 초기화, Buffer id반환
Buffer id와 해당 instance count를 사용자가 제공
빈 공간 검사 후 부족해면 새 Buffer 생성, Buffer id와 offset 반환
*/
pub struct VertexBufferPool {}

/*
미리 어느정도의 Buffer 생성
size를 사용자가 제공
빈 공간 검사 후 부족하면 새 Buffer 생성, Buffer id와 offset 반환
*/
pub struct UniformBufferPool {}

/*
SamplerDescriptor 를 사용자가 제공
내부 풀에 검색 후 없으면 Sampler 생성, 저장 및 id 생성
id 사용자에게 반환
*/
pub struct SamplerPool {}

/*
TextureDescriptor 를 사용자가 제공
내부 풀에 검색 후 없으면 Texture 생성, 저장 및 id 생성
id 사용자에게 반환
Texture id, TextureViewDescriptor 를 사용자가 제공
내부 풀에 검색 후 없으면 TextureView 생성, 저장 및 id 생성
id 사용자에게 반환
TextureView는 Texture에 의존적인 id 및 생명주기 관리
*/
pub struct TexturePool {}

/*
RenderPassDescriptor 를 사용자가 제공
내부 풀에 검색 후 없으면 저장 및 id 생성
id 사용자에게 반환
*/
pub struct RenderPassPool {}
