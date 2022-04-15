use rust_try_lib::{
    application::{Application, Scene},
    cgmath::*,
    graphics::elements::*,
    graphics::{Graphics, Renderer},
    inputs::{Inputs, KeyCode},
    utils::Utils,
    winit,
};

pub struct InitialScene {
    renderer: Renderer,
    camera: Camera,
    projection: PerspectiveFov<f32>,
}

impl InitialScene {
    pub fn new(app: &Application) -> Self {
        let camera = Camera::new(point3(0.0, 0.0, 5.0), vec3(0.0, 0.0, -1.0), 1.0);

        let projection = PerspectiveFov {
            fovy: Rad(std::f32::consts::FRAC_PI_4),
            aspect: app.graphics().aspect(app.graphics().primary_window_id()),
            near: 0.1,
            far: 100.0,
        };

        Self {
            renderer: Renderer::new(&app.graphics()),
            camera,
            projection,
        }
    }
}

impl Scene for InitialScene {
    fn enter(&mut self) {}

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let float_new_size = new_size.cast::<f32>();
        self.projection.aspect = float_new_size.width / float_new_size.height;
        self.renderer.resize(new_size);
    }

    fn update(&mut self, utils: &Utils, inputs: &Inputs) {
        let forward = if inputs.keyboard_is_pressed(KeyCode::W) {
            1f32
        } else {
            0f32
        };
        let backward = if inputs.keyboard_is_pressed(KeyCode::S) {
            1f32
        } else {
            0f32
        };
        let right = if inputs.keyboard_is_pressed(KeyCode::D) {
            1f32
        } else {
            0f32
        };
        let left = if inputs.keyboard_is_pressed(KeyCode::A) {
            1f32
        } else {
            0f32
        };
        let up = if inputs.keyboard_is_pressed(KeyCode::Space) {
            1f32
        } else {
            0f32
        };
        let down = if inputs.keyboard_is_pressed(KeyCode::LShift) {
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
            Matrix4::<f32>::from(self.projection) * self.camera.view_matrix(),
        );
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
}

impl Transform {
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

impl Camera {
    pub fn r#move(&mut self, delta: f32, forward: f32, right: f32, up: f32) {
        let mut direction = self.rotation().rotate_vector(vec3(forward, 0.0, right));
        direction[2] += up;
        if direction.magnitude2() > 1f32 {
            direction = direction.normalize();
        }
        self.transform.r#move(self.speed * delta * direction);
    }
}
