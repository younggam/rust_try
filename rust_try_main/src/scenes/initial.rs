use rust_try_lib::application::Scene;
use rust_try_lib::cgmath::*;
use rust_try_lib::graphics::elements::*;
use rust_try_lib::graphics::Batch;

pub struct InitialScene {}

impl InitialScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for InitialScene {
    fn enter(&mut self) {}

    fn update(&mut self) {}

    fn draw(&self, graphics: &mut Batch) {
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
                graphics.draw(
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
    }

    fn should_exit(&self) {}

    fn exit(&mut self) -> Option<Box<dyn Scene>> {
        None
    }

    fn force_exit(&mut self) {}
}
