use super::buttons::ElementState;

use cgmath::*;

use winit::event::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(usize),
}

pub struct Mouse {
    motion: Vector2<f32>,
    wheel: f32,

    buttons: Vec<ElementState>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            motion: Vector2::zero(),
            wheel: 0.0,

            buttons: Vec::with_capacity(3),
        }
    }

    pub fn motion(&self) -> Vector2<f32> {
        self.motion
    }

    pub fn wheel(&self) -> f32 {
        self.wheel
    }

    fn button_to_index(button: MouseButton) -> usize {
        match button {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
            MouseButton::Other(val) => val,
        }
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool {
        match self.buttons.get(Self::button_to_index(button)) {
            Some(state) => *state == ElementState::Pressed,
            _ => false,
        }
    }
}

impl Mouse {
    pub(crate) fn pre_update(&mut self) {
        self.motion = Vector2::zero();
        self.wheel = 0.0;
    }
}
