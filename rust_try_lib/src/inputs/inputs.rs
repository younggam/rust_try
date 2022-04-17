use super::*;

use cgmath::*;

use winit::event::*;

/*
Window에 해당하는 input 과
Device에 해당하는 input 구별 필요
WindowInputs?
    WindowId 기준 map
    각 input이 Window 마다 유일함

DeviceInputs?
    DeviceId 기준 map <DeviceId, Vec<Device>>
    각 input이 Device 마다 유일함 -> but 그게 중요한가?
    windowid는 사용자 측에서 관리할 이유가 많지만 DeviceId는 사용자측에서 굳이? -> primary 기능
*/
pub struct Inputs {
    keyboard: KeyBoard,
    cursor: Cursor,
}

impl Inputs {
    pub(crate) fn new() -> Self {
        Self {
            keyboard: KeyBoard::new(),
            cursor: Cursor::new(),
        }
    }
}

impl Inputs {
    pub(crate) fn pre_update(&mut self) {
        self.keyboard.pre_update();
        self.cursor.pre_update();
    }

    pub(crate) fn handle_input(&mut self, input: WindowEvent) {
        match input {
            WindowEvent::KeyboardInput { input, .. } => self.keyboard.handle_input(input),
            WindowEvent::CursorMoved { .. }
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::CursorLeft { .. } => self.cursor.handle_input(input),
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
    pub fn is_key_signaled(&self, key: KeyCode) -> bool {
        self.keyboard.is_signaled(key)
    }

    pub fn are_keys_signaled(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_signaled(keys)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn are_keys_pressed(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_pressed(keys)
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_released(key)
    }

    pub fn are_keys_released(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_released(keys)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_pressed(key)
    }

    pub fn are_keys_just_pressed(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_just_pressed(keys)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_released(key)
    }

    pub fn are_keys_just_released(&self, keys: &[KeyCode]) -> bool {
        self.keyboard.are_just_released(keys)
    }
}

impl Inputs {
    pub fn cursor_motion(&self) -> Vector2<f32> {
        self.cursor.motion()
    }
}
