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

    pub fn motion(&self) -> Vector2<f32> {
        self.position - self.prev_position
    }

    pub fn is_just_entered(&self) -> bool {
        self.activated && !self.prev_activated
    }
}

impl Cursor {
    pub fn pre_update(&mut self) {
        self.prev_position = self.position;
        self.prev_activated = self.activated;
    }

    pub fn handle_input(&mut self, input: WindowEvent) {
        match input {
            WindowEvent::CursorMoved { position, .. } => {
                self.position = point2(position.x as f32, position.y as f32);
                if self.is_just_entered() {
                    self.prev_position = self.position;
                }
            }
            WindowEvent::CursorEntered { .. } => {
                self.activated = true;
            }
            WindowEvent::CursorLeft { .. } => {
                self.activated = false;
            }
            _ => {}
        }
    }
}
