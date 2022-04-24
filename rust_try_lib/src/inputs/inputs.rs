use super::*;

use std::collections::HashMap;

use winit::{event::*, window::WindowId};

pub struct Inputs {
    window_inputs: HashMap<WindowId, WindowInput>,
    device_inputs: DeviceInputs,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            window_inputs: HashMap::new(),
            device_inputs: DeviceInputs::new(),
        }
    }

    pub fn window_keyboard(&self, window_id: WindowId) -> Option<&Keyboard> {
        match self.window_inputs.get(&window_id) {
            Some(window_input) => Some(window_input.keyboard()),
            _ => None,
        }
    }

    pub fn device_keyboard(&self, device_id: Option<DeviceId>) -> Option<&Keyboard> {
        self.device_inputs.keyboard(device_id)
    }

    pub fn cursor(&self, window_id: WindowId) -> Option<&Cursor> {
        match self.window_inputs.get(&window_id) {
            Some(window_input) => Some(window_input.cursor()),
            _ => None,
        }
    }

    pub fn window_mouse(&self, window_id: WindowId) -> Option<&Mouse> {
        match self.window_inputs.get(&window_id) {
            Some(window_input) => Some(window_input.mouse()),
            _ => None,
        }
    }

    pub fn device_mouse(&self, device_id: Option<DeviceId>) -> Option<&Mouse> {
        self.device_inputs.mouse(device_id)
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
        let device = match self.window_inputs.get_mut(&window_id) {
            Some(window_input) => window_input.handle_input(input),
            _ => {
                let mut window_input = WindowInput::new();
                let ret = window_input.handle_input(input);
                self.window_inputs.insert(window_id, window_input);
                ret
            }
        };

        if let Some(device) = device {
            self.device_inputs.replace_mock(device.0, device.1);
        }
    }

    pub(crate) fn handle_device_input(&mut self, device_id: DeviceId, input: DeviceEvent) {
        self.device_inputs.handle_input(device_id, input);
    }
}

//

pub struct WindowInput {
    keyboard: Keyboard,
    cursor: Cursor,
    mouse: Mouse,
}

impl WindowInput {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            cursor: Cursor::new(),
            mouse: Mouse::new(),
        }
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }
}

impl WindowInput {
    pub(crate) fn pre_update(&mut self) {
        self.keyboard.pre_update();
        self.cursor.pre_update();
        self.mouse.pre_update();
    }

    pub(crate) fn handle_input(&mut self, input: WindowEvent) -> Option<(DeviceId, DeviceType)> {
        match input {
            WindowEvent::KeyboardInput {
                device_id, input, ..
            } => {
                self.keyboard.handle_input(input);
                Some((device_id, DeviceType::Keyboard))
            }
            WindowEvent::CursorMoved { .. }
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::CursorLeft { .. } => self.cursor.handle_input(input),
            WindowEvent::MouseWheel { .. } | WindowEvent::MouseInput { .. } => {
                self.mouse.handle_window_input(input)
            }
            _ => None,
        }
    }
}

//

#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Keyboard,
    Mouse,
}

//

pub struct DeviceInputs {
    keyboards: HashMap<DeviceId, Keyboard>,
    primary_keyboard_id: Option<DeviceId>,

    mouses: HashMap<DeviceId, Mouse>,
    primary_mouse_id: Option<DeviceId>,

    mocks: HashMap<DeviceId, MockDevice>,
}

impl DeviceInputs {
    pub fn new() -> Self {
        Self {
            keyboards: HashMap::new(),
            primary_keyboard_id: None,

            mouses: HashMap::new(),
            primary_mouse_id: None,

            mocks: HashMap::new(),
        }
    }

    pub fn keyboard(&self, device_id: Option<DeviceId>) -> Option<&Keyboard> {
        match device_id {
            Some(device_id) => self.keyboards.get(&device_id),
            _ => match self.primary_keyboard_id {
                Some(primary_keyboard_id) => self.keyboards.get(&primary_keyboard_id),
                _ => None,
            },
        }
    }

    pub fn mouse(&self, device_id: Option<DeviceId>) -> Option<&Mouse> {
        match device_id {
            Some(device_id) => self.mouses.get(&device_id),
            _ => match self.primary_mouse_id {
                Some(primary_mouse_id) => self.mouses.get(&primary_mouse_id),
                _ => None,
            },
        }
    }
}

impl DeviceInputs {
    fn remove_device(&mut self, device_id: DeviceId) {
        if let Some(_) = self.mocks.remove(&device_id) {
        } else if let Some(_) = self.keyboards.remove(&device_id) {
            self.primary_keyboard_id = self.keyboards.keys().next().copied();
        } else if let Some(_) = self.mouses.remove(&device_id) {
            self.primary_mouse_id = self.mouses.keys().next().copied();
        }
    }

    pub(crate) fn pre_update(&mut self) {
        for mock in self.mocks.values_mut() {
            mock.pre_update();
        }
        for keyboard in self.keyboards.values_mut() {
            keyboard.pre_update();
        }
        for mouse in self.mouses.values_mut() {
            mouse.pre_update();
        }
    }

    pub(crate) fn handle_input(&mut self, device_id: DeviceId, input: DeviceEvent) {
        match input {
            DeviceEvent::Added => {
                self.mocks.insert(device_id, MockDevice::new());
            }
            DeviceEvent::Removed => self.remove_device(device_id),
            _ => {
                if let Some(mock) = self.mocks.get_mut(&device_id) {
                    if let Some(device_type) = mock.handle_input(&input) {
                        self.replace_mock(device_id, device_type);
                    }
                }

                if let Some(keyboard) = self.keyboards.get_mut(&device_id) {
                    if let DeviceEvent::Key(input) = input {
                        keyboard.handle_input(input);
                    }
                } else if let Some(mouse) = self.mouses.get_mut(&device_id) {
                    mouse.handle_device_input(input);
                }
            }
        }
    }

    pub(crate) fn replace_mock(&mut self, device_id: DeviceId, device_type: DeviceType) {
        if let Some(mock) = self.mocks.remove(&device_id) {
            match device_type {
                DeviceType::Keyboard => {
                    if let None = self.primary_keyboard_id {
                        self.primary_keyboard_id = Some(device_id);
                    }
                    self.keyboards.insert(device_id, mock.into());
                }
                DeviceType::Mouse => {
                    if let None = self.primary_mouse_id {
                        self.primary_mouse_id = Some(device_id);
                    }
                    self.mouses.insert(device_id, mock.into());
                }
            }
        }
    }
}
