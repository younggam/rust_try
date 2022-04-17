use crate::objects::camera::*;

use rust_try_lib::{
    application::{Application, Scene},
    cgmath::*,
    graphics::elements::*,
    graphics::{Graphics, Renderer},
    inputs::{Inputs, KeyCode},
    utils::Utils,
    winit,
    winit::window::WindowId,
};

pub struct InitialScene {
    renderer: Renderer,
    camera: Camera,
    projection: PerspectiveFov<f32>,
    target_window_id: WindowId,
}

impl InitialScene {
    pub fn new(app: &Application) -> Self {
        let camera = Camera::new(point3(0.0, 0.0, 5.0), vec3(0.0, 0.0, -1.0), 1.0, Deg(0.1));

        let target_window_id = app.graphics().primary_window_id().unwrap();

        let projection = PerspectiveFov {
            fovy: Rad(std::f32::consts::FRAC_PI_4),
            aspect: app.graphics().aspect(target_window_id),
            near: 0.1,
            far: 100.0,
        };

        Self {
            renderer: Renderer::new(&app.graphics(), target_window_id),
            camera,
            projection,
            target_window_id,
        }
    }
}

impl Scene for InitialScene {
    fn enter(&mut self) {}

    fn resize(&mut self, window_id: WindowId, new_size: winit::dpi::PhysicalSize<u32>) {
        if window_id == self.target_window_id {
            let float_new_size = new_size.cast::<f32>();
            self.projection.aspect = float_new_size.width / float_new_size.height;
        }
    }

    fn update(&mut self, utils: &Utils, inputs: &Inputs) {
        let forward = if inputs.is_key_pressed(KeyCode::W) {
            1f32
        } else {
            0f32
        };
        let backward = if inputs.is_key_pressed(KeyCode::S) {
            1f32
        } else {
            0f32
        };
        let right = if inputs.is_key_pressed(KeyCode::D) {
            1f32
        } else {
            0f32
        };
        let left = if inputs.is_key_pressed(KeyCode::A) {
            1f32
        } else {
            0f32
        };
        let up = if inputs.is_key_pressed(KeyCode::Space) {
            1f32
        } else {
            0f32
        };
        let down = if inputs.is_key_pressed(KeyCode::LShift) {
            1f32
        } else {
            0f32
        };
        self.camera.r#move(
            utils.time_delta() as f32,
            forward - backward,
            right - left,
            up - down,
        );

        let cursor_motion = inputs.cursor_motion();
        self.camera
            .rotate(cursor_motion.magnitude(), cursor_motion.x, cursor_motion.y);
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
        self.renderer.render(
            &graphics,
            self.target_window_id,
            Matrix4::<f32>::from(self.projection) * self.camera.view_matrix(),
        );
    }

    fn should_exit(&self) {}

    fn exit(&mut self) -> Option<Box<dyn Scene>> {
        None
    }

    fn force_exit(&mut self) {}
}
