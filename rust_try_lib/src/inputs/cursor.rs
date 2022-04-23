use super::inputs::Device;

use cgmath::*;

use winit::event::*;

pub struct Cursor {
    position: Point2<f32>,
    prev_position: Point2<f32>,
    activated: bool,
    prev_activated: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: point2(0.0, 0.0),
            prev_position: point2(0.0, 0.0),
            activated: false,
            prev_activated: false,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn motion(&self) -> Vector2<f32> {
        self.position - self.prev_position
    }

    pub fn is_just_entered(&self) -> bool {
        self.activated && !self.prev_activated
    }
}

impl Cursor {
    pub(crate) fn pre_update(&mut self) {
        self.prev_position = self.position;
        self.prev_activated = self.activated;
    }

    pub(crate) fn handle_input(&mut self, input: WindowEvent) -> Option<Device> {
        match input {
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                self.position = point2(position.x as f32, -position.y as f32);
                if self.is_just_entered() {
                    self.prev_position = self.position;
                }
                Some(Device::Mouse(device_id))
            }
            WindowEvent::CursorEntered { device_id } => {
                self.activated = true;
                Some(Device::Mouse(device_id))
            }
            WindowEvent::CursorLeft { device_id } => {
                self.activated = false;
                Some(Device::Mouse(device_id))
            }
            _ => None,
        }
    }
}
