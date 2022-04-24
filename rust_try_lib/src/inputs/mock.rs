use super::{buttons::*, inputs::DeviceType};

use winit::event::*;

///Replacement of unfamiliar devices. Could be deleted someday.
pub struct MockDevice {
    pub(super) first_motion: Vec<Option<f32>>,
    pub(super) motion: Vec<f32>,
    pub(super) last_motion: Vec<f32>,
    pub(super) buttons: Buttons,
    pub(super) texts: String,
}

impl MockDevice {
    pub fn new() -> Self {
        Self {
            first_motion: vec![None; 3],
            motion: vec![0.0; 3],
            last_motion: vec![0.0; 3],
            buttons: Buttons::new(4),
            texts: String::with_capacity(8),
        }
    }

    pub fn first_motion(&self) -> &[Option<f32>] {
        &self.first_motion
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
        self.first_motion.fill(None);
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
                    self.first_motion.resize(axis + 1, None);
                    self.motion.resize(axis + 1, 0.0);
                    self.last_motion.resize(axis + 1, 0.0);
                }
                let value = value as f32;
                if let None = self.first_motion[axis] {
                    self.first_motion[axis] = Some(value);
                }
                self.motion[axis] += value;
                self.last_motion[axis] = value;
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
