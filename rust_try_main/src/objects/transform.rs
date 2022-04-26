use rust_try_lib::cgmath::*;

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

    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    pub fn rotation(&self) -> Quaternion<f32> {
        self.rotation
    }

    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    pub fn set_position(&mut self, new_pos: Point3<f32>) {
        self.position = new_pos;
    }

    pub fn set_rotation(&mut self, new_pos: Point3<f32>) {
        self.position = new_pos;
    }

    pub fn set_scale(&mut self, new_pos: Point3<f32>) {
        self.position = new_pos;
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
