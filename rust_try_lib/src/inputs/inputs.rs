use super::*;

use std::collections::HashMap;

use winit::{event::*, window::WindowId};

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
    window_inputs: HashMap<WindowId, WindowInput>,
    device_inputs: DeviceInputs,
}

impl Inputs {
    pub(crate) fn new() -> Self {
        Self {
            window_inputs: HashMap::new(),
            device_inputs: DeviceInputs::new(),
        }
    }

    pub fn window_keyboard(&self, window_id: WindowId) -> Option<&KeyBoard> {
        match self.window_inputs.get(&window_id) {
            Some(window_input) => Some(window_input.keyboard()),
            _ => None,
        }
    }

    pub fn device_keyboard(&self, device_id: Option<DeviceId>) -> Option<&KeyBoard> {
        self.device_inputs.keyboard(device_id)
    }

    pub fn cursor(&self, window_id: WindowId) -> Option<&Cursor> {
        match self.window_inputs.get(&window_id) {
            Some(window_input) => Some(window_input.cursor()),
            _ => None,
        }
    }
}

impl Inputs {
    pub(crate) fn pre_update(&mut self) {
        for window_input in self.window_inputs.values_mut() {
            window_input.pre_update();
        }
        self.device_inputs.pre_update();
    }

    pub(crate) fn handle_window_input(&mut self, window_id: WindowId, input: WindowEvent) {
        match self.window_inputs.get_mut(&window_id) {
            Some(window_input) => window_input.handle_input(input),
            _ => {
                let mut window_input = WindowInput::new();
                window_input.handle_input(input);
                self.window_inputs.insert(window_id, window_input);
            }
        }
    }

    pub(crate) fn handle_device_input(&mut self, device_id: DeviceId, input: DeviceEvent) {
        self.device_inputs.handle_input(device_id, input);
    }
}

//

pub struct WindowInput {
    keyboard: KeyBoard,
    cursor: Cursor,
}

impl WindowInput {
    pub(crate) fn new() -> Self {
        Self {
            keyboard: KeyBoard::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn keyboard(&self) -> &KeyBoard {
        &self.keyboard
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }
}

impl WindowInput {
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
            WindowEvent::MouseWheel { .. } => {
                println!("{:?}", input)
            }
            WindowEvent::MouseInput { .. } => {
                println!("{:?}", input)
            }
            _ => {}
        }
    }
}

//

pub struct DeviceInputs {
    keyboards: HashMap<DeviceId, KeyBoard>,
    primary_keyboard_id: Option<DeviceId>,
}

impl DeviceInputs {
    pub(crate) fn new() -> Self {
        Self {
            keyboards: HashMap::new(),
            primary_keyboard_id: None,
        }
    }

    pub fn keyboard(&self, device_id: Option<DeviceId>) -> Option<&KeyBoard> {
        match device_id {
            Some(device_id) => self.keyboards.get(&device_id),
            _ => match self.primary_keyboard_id {
                Some(primary_keyboard_id) => self.keyboards.get(&primary_keyboard_id),
                _ => None,
            },
        }
    }
}

impl DeviceInputs {
    pub(crate) fn pre_update(&mut self) {
        for keyboard in self.keyboards.values_mut() {
            keyboard.pre_update();
        }
    }

    pub(crate) fn handle_input(&mut self, device_id: DeviceId, input: DeviceEvent) {
        match input {
            DeviceEvent::Removed => {
                self.keyboards.remove(&device_id);
                println!("{:?}", device_id);
                self.primary_keyboard_id = self.keyboards.keys().next().copied();
            }
            DeviceEvent::Key(input) => match self.keyboards.get_mut(&device_id) {
                Some(keyboard) => keyboard.handle_input(input),
                _ => {
                    let mut keyboard = KeyBoard::new();
                    keyboard.handle_input(input);
                    if let None = self.primary_keyboard_id {
                        self.primary_keyboard_id = Some(device_id);
                    }
                    self.keyboards.insert(device_id, keyboard);
                }
            },
            _ => {}
        }
    }
}
