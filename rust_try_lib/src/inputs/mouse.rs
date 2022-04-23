use super::{buttons::*, inputs::DeviceType, mock::*};

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
    last_motion: Vector2<f32>,
    wheel: f32,

    buttons: Buttons,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            motion: Vector2::zero(),
            last_motion: Vector2::zero(),
            wheel: 0.0,

            buttons: Buttons::new(3),
        }
    }

    pub fn motion(&self) -> Vector2<f32> {
        self.motion
    }

    pub fn last_motion(&self) -> Vector2<f32> {
        self.last_motion
    }

    pub fn wheel(&self) -> f32 {
        self.wheel
    }
}

impl Mouse {
    pub(crate) fn pre_update(&mut self) {
        self.motion.set_zero();
        self.last_motion.set_zero();
        self.wheel = 0.0;

        self.buttons.pre_update();
    }

    pub(crate) fn handle_window_input(
        &mut self,
        input: WindowEvent,
    ) -> Option<(DeviceId, DeviceType)> {
        match input {
            WindowEvent::MouseWheel {
                device_id, delta, ..
            } => {
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => self.wheel += y,
                    MouseScrollDelta::PixelDelta(pixels) => self.wheel += pixels.y as f32,
                }
                Some((device_id, DeviceType::Mouse))
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {
                self.buttons
                    .handle_input(Into::<MouseButton>::into(button), state);
                Some((device_id, DeviceType::Mouse))
            }
            _ => None,
        }
    }

    pub(crate) fn handle_device_input(&mut self, input: DeviceEvent) {
        match input {
            DeviceEvent::MouseMotion { delta } => {
                self.motion.x += delta.0 as f32;
                self.motion.y += -delta.1 as f32;
                self.last_motion.x = delta.0 as f32;
                self.last_motion.y = -delta.1 as f32;
            }
            DeviceEvent::MouseWheel { delta } => match delta {
                MouseScrollDelta::LineDelta(_, y) => self.wheel += y,
                MouseScrollDelta::PixelDelta(pixels) => self.wheel += pixels.y as f32,
            },
            DeviceEvent::Button { button, state } => {
                self.buttons.handle_input(button as usize, state)
            }
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

impl From<MockDevice> for Mouse {
    fn from(mut mock: MockDevice) -> Self {
        mock.motion.resize(2, 0.0);
        mock.last_motion.resize(2, 0.0);
        mock.buttons.resize(163);
        Self {
            motion: vec2(mock.motion[0], mock.motion[1]),
            last_motion: vec2(mock.last_motion[0], mock.last_motion[1]),
            wheel: 0.0,

            buttons: mock.buttons,
        }
    }
}
