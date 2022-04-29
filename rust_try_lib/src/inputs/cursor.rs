use super::inputs::DeviceType;

use cgmath::*;

use winit::event::*;

pub struct Cursor {
    position: Point2<f32>,
    entered: bool,
    prev_entered: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: point2(0.0, 0.0),
            entered: false,
            prev_entered: false,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn is_entered(&self) -> bool {
        self.entered
    }

    pub fn is_just_entered(&self) -> bool {
        self.entered && !self.prev_entered
    }

    pub fn is_just_left(&self) -> bool {
        !self.entered && self.prev_entered
    }
}

impl Cursor {
    pub(crate) fn pre_update(&mut self) {
        self.prev_entered = self.entered;
    }

    pub(crate) fn handle_input(&mut self, input: WindowEvent) -> Option<(DeviceId, DeviceType)> {
        match input {
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                self.position.x = position.x as f32;
                self.position.y = position.y as f32;
                Some((device_id, DeviceType::Mouse))
            }
            WindowEvent::CursorEntered { device_id } => {
                self.entered = true;
                Some((device_id, DeviceType::Mouse))
            }
            WindowEvent::CursorLeft { device_id } => {
                self.entered = false;
                Some((device_id, DeviceType::Mouse))
            }
            _ => None,
        }
    }
}
