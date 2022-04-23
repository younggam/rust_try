use super::buttons::*;

use winit::event::*;

pub struct MockDevice {
    motion: Vec<f32>,
    buttons: Buttons,
    texts: String,
}

impl MockDevice {
    pub fn new() -> Self {
        Self {
            motion: vec![0.0; 3],
            buttons: Buttons::new(4),
            texts: String::with_capacity(8),
        }
    }

    pub fn motion(&self) -> &[f32] {
        &self.motion
    }

    pub fn texts(&self) -> &str {
        &self.texts
    }
}

impl MockDevice {
    pub(crate) fn pre_update(&mut self) {
        self.motion.fill(0.0);
        self.texts.clear();
        self.buttons.pre_update();
    }

    pub(crate) fn handle_input(&mut self, input: winit::event::DeviceEvent) {
        match input {
            DeviceEvent::Motion { axis, value } => {
                let axis = axis as usize;
                if axis >= self.motion.len() {
                    self.motion.resize(axis + 1, 0.0);
                }
                self.motion[axis] = value as f32;
            }
            DeviceEvent::Button { button, state } => {
                self.buttons.handle_input(button as usize, state)
            }
            DeviceEvent::Text { codepoint } => self.texts.push(codepoint),
            _ => {}
        }
    }
}

impl std::ops::Deref for MockDevice {
    type Target = Buttons;

    fn deref(&self) -> &Self::Target {
        &self.buttons
    }
}
