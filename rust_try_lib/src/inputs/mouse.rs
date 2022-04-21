use super::buttons::*;

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

    buttons: Buttons,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            motion: Vector2::zero(),
            wheel: 0.0,

            buttons: Buttons::new(3),
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
}

impl Mouse {
    pub(crate) fn pre_update(&mut self) {
        self.motion = Vector2::zero();
        self.wheel = 0.0;

        self.buttons.pre_update();
    }

    // pub(crate) fn handle_input(&mut self,)
}

impl std::ops::Deref for Mouse {
    type Target = Buttons;

    fn deref(&self) -> &Self::Target {
        &self.buttons
    }
}
