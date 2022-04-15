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
}

impl Inputs {
    pub(crate) fn pre_update(&mut self) {
        self.keyboard.pre_update();
    }

    pub(crate) fn handle_input(&mut self, input: WindowEvent) {
        match input {
            WindowEvent::KeyboardInput { input, .. } => self.keyboard.handle_input(input),
            WindowEvent::CursorMoved { position, .. } => {
                println!("{:?}", position)
            }
            WindowEvent::CursorEntered { .. } => {
                println!("Entered")
            }
            WindowEvent::CursorLeft { .. } => {
                println!("Left")
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                println!("{:?} {:?}", delta, phase)
            }
            WindowEvent::MouseInput { state, button, .. } => {
                println!("{:?} {:?}", state, button)
            }
            _ => {}
        }
    }
}

impl Inputs {
    pub fn keyboard_is_signaled(&self, key: KeyCode) -> bool {
        self.keyboard.is_signaled(key)
    }

    pub fn keyboard_are_signaled(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_signaled(keys)
    }

    pub fn keyboard_is_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn keyboard_are_pressed(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_pressed(keys)
    }

    pub fn keyboard_is_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_released(key)
    }

    pub fn keyboard_are_released(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_released(keys)
    }

    pub fn keyboard_is_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_pressed(key)
    }

    pub fn keyboard_are_just_pressed(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_just_pressed(keys)
    }

    pub fn keyboard_is_just_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_released(key)
    }

    pub fn keyboard_are_just_released(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_just_released(keys)
    }
}
