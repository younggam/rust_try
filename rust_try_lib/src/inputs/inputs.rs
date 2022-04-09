use super::*;

use winit::event::*;

pub struct Inputs {
    keyboard: KeyBoard,
}

impl Inputs {
    pub(crate) fn new() -> Self {
        Self {
            keyboard: KeyBoard::new(),
        }
    }

    pub(crate) fn pre_update(&mut self) {
        self.keyboard.pre_update();
    }

    pub(crate) fn update(&mut self) {}

    pub(crate) fn handle_input(&mut self, input: WindowEvent) {
        match input {
            WindowEvent::KeyboardInput { input, .. } => self.keyboard.handle_input(input),
            _ => {}
        }
    }
}
