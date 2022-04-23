use super::{buttons::*, inputs::DeviceType};

use winit::event::*;

///Replacement of unfamiliar devices. Could be deleted someday.
pub struct MockDevice {
    pub(super) motion: Vec<f32>,
    pub(super) last_motion: Vec<f32>,
    pub(super) buttons: Buttons,
    pub(super) texts: String,
}

impl MockDevice {
    pub fn new() -> Self {
        Self {
            motion: vec![0.0; 3],
            last_motion: vec![0.0; 3],
            buttons: Buttons::new(4),
            texts: String::with_capacity(8),
        }
    }

    pub fn motion(&self) -> &[f32] {
        &self.motion
    }

    pub fn last_motion(&self) -> &[f32] {
        &self.last_motion
    }

    pub fn texts(&self) -> &str {
        &self.texts
    }
}

impl MockDevice {
    pub(crate) fn pre_update(&mut self) {
        self.motion.fill(0.0);
        self.last_motion.fill(0.0);
        self.texts.clear();
        self.buttons.pre_update();
    }

    pub(crate) fn handle_input(&mut self, input: &DeviceEvent) -> Option<DeviceType> {
        match *input {
            DeviceEvent::MouseMotion { .. } => Some(DeviceType::Mouse),
            DeviceEvent::MouseWheel { .. } => Some(DeviceType::Mouse),
            DeviceEvent::Key(_) => Some(DeviceType::Keyboard),
            DeviceEvent::Motion { axis, value } => {
                let axis = axis as usize;
                if axis >= self.motion.len() {
                    self.motion.resize(axis + 1, 0.0);
                    self.last_motion.resize(axis + 1, 0.0);
                }
                self.motion[axis] += value as f32;
                self.last_motion[axis] = value as f32;
                None
            }
            DeviceEvent::Button { button, state } => {
                self.buttons.handle_input(button as usize, state);
                None
            }
            DeviceEvent::Text { codepoint } => {
                self.texts.push(codepoint);
                None
            }
            _ => None,
        }
    }
}

impl std::ops::Deref for MockDevice {
    type Target = Buttons;

    fn deref(&self) -> &Self::Target {
        &self.buttons
    }
}
