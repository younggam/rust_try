use crate::objects::camera::*;

use rust_try_lib::{
    application::{Application, Scene},
    cgmath::*,
    graphics::elements::*,
    graphics::*,
    inputs::*,
    utils::Utils,
    wgpu, winit,
    winit::window::WindowId,
};

use std::num::NonZeroU64;

pub struct InitialScene {
    // gui_renderer: Renderer,
    renderer: Renderer,
    target_window_id: WindowId,

    camera: Camera,
}

impl InitialScene {
    pub fn new(app: &Application) -> Self {
        let target_window_id = app.graphics().primary_window_id().unwrap();

        let camera = Camera::new(
            app.graphics().aspect(target_window_id),
            point3(0.0, 0.0, 5.0),
            vec3(0.0, 0.0, -1.0),
            1.0,
            Deg(0.1),
        );

        Self {
            // gui_renderer: Renderer::new(
            //     &app.graphics(),
            //     target_window_id,
            //     BindGroupConfig {
            //         name: "View Projection",
            //         entries: vec![BindGroupConfigEntry {
            //             binding: 0,
            //             visibility: wgpu::ShaderStages::VERTEX,
            //             ty: wgpu::BindingType::Buffer {
            //                 ty: wgpu::BufferBindingType::Uniform,
            //                 has_dynamic_offset: false,
            //                 min_binding_size: Some(unsafe { NonZeroU64::new_unchecked(64) }),
            //             },
            //             count: None,
            //         }],
            //     },
            // ),
            renderer: Renderer::new(
                &app.graphics(),
                target_window_id,
                &[BindGroupConfig {
                    name: "View Projection",
                    entries: vec![BindGroupConfigEntry {
                        name: "View Projection Matrix",
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: Some(unsafe { NonZeroU64::new_unchecked(64) }),
                        },
                        count: None,
                    }],
                }],
            ),
            target_window_id,

            camera,
        }
    }
}

impl InitialScene {
    fn handle_input(&mut self, utils: &Utils, inputs: &Inputs) {
        self.camera
            .handle_input(self.target_window_id, utils, inputs);
    }
}

impl Scene for InitialScene {
    fn enter(&mut self) {}

    fn resize(&mut self, window_id: WindowId, new_size: winit::dpi::PhysicalSize<u32>) {
        if window_id == self.target_window_id {
            self.camera.resize(new_size);
        }
    }

    fn update(&mut self, utils: &Utils, inputs: &Inputs) {
        self.handle_input(utils, inputs);
    }

    fn render(&mut self, graphics: &Graphics) {
        let colored_triangle = Mesh::new(
            vec![
                ColorVertex::new([0.0, 0.5, 0.0, 5.0], [0.0, 1.0, 0.0, 1.0]),
                ColorVertex::new([-0.5, -0.5, 0.0, 5.0], [1.0, 0.0, 0.0, 1.0]),
                ColorVertex::new([0.5, -0.5, 0.0, 5.0], [0.0, 0.0, 1.0, 1.0]),
            ],
            vec![0, 1, 2],
        );
        let black_triangle = Mesh::new(
            vec![
                ColorVertex::new([0.0, 0.5, 0.0, 5.0], [0.0, 0.0, 0.0, 1.0]),
                ColorVertex::new([-0.5, -0.5, 0.0, 5.0], [0.0, 0.0, 0.0, 1.0]),
                ColorVertex::new([0.5, -0.5, 0.0, 5.0], [0.0, 0.0, 0.0, 1.0]),
            ],
            vec![0, 1, 2],
        );
        let axis: Vector3<f32> = vec3(1.0, 1.0, 1.0).normalize();
        for i in 0..10 {
            for j in 0..10 {
                let k = (i * 10 + j * 100) as f32 * std::f32::consts::PI / 360.0;
                self.renderer.batch(
                    if (i + j) % 2 == 0 {
                        &colored_triangle
                    } else {
                        &black_triangle
                    },
                    point3(0.9 - 0.2 * i as f32, 0.9 - 0.2 * j as f32, 0.5),
                    Quaternion::from_sv(k.cos(), k.sin() * axis),
                    vec3(1.0, 1.0, 1.0),
                );
            }
        }
        let _ = self.renderer.render(
            &graphics,
            self.target_window_id,
            &[&[Some(self.camera.view_proj_matrix())]],
        );
    }

    fn should_exit(&self) {}

    fn exit(&mut self) -> Option<Box<dyn Scene>> {
        None
    }

    fn force_exit(&mut self) {}
}
