use rust_try_lib::application::Scene;
use rust_try_lib::cgmath::*;
use rust_try_lib::graphics::elements::*;
use rust_try_lib::graphics::Batch;

pub struct InitialScene {
    camera: Camera,
}

impl InitialScene {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(point3(0.0, 0.0, 10.0), vec3(0.0, 0.0, -1.0), 1.0),
        }
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

pub struct Projection {}
