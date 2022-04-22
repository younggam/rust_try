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

impl From<MouseButton> for usize {
    fn from(button: MouseButton) -> usize {
        match button {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
            MouseButton::Other(val) => val,
        }
    }
}

use winit::event::MouseButton as WinitMouseButton;
impl From<WinitMouseButton> for MouseButton {
    fn from(button: WinitMouseButton) -> MouseButton {
        match button {
            WinitMouseButton::Left => MouseButton::Left,
            WinitMouseButton::Middle => MouseButton::Middle,
            WinitMouseButton::Right => MouseButton::Right,
            WinitMouseButton::Other(val) => MouseButton::Other(val as usize),
        }
    }
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
}

impl Mouse {
    pub(crate) fn pre_update(&mut self) {
        self.motion = Vector2::zero();
        self.wheel = 0.0;

        self.buttons.pre_update();
    }

    pub(crate) fn handle_window_input(&mut self, input: WindowEvent) {
        match input {
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(_, y) => self.wheel = y,
                MouseScrollDelta::PixelDelta(pixels) => self.wheel = pixels.y as f32,
            },
            WindowEvent::MouseInput { state, button, .. } => self
                .buttons
                .handle_input(Into::<MouseButton>::into(button), state),
            _ => {}
        }
    }
}

impl std::ops::Deref for Mouse {
    type Target = Buttons;

    fn deref(&self) -> &Self::Target {
        &self.buttons
    }
}
