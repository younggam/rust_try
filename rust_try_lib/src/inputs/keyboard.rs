#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,
    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

pub struct KeyBoard {
    present: [KeyState; 163],
    previous: [KeyState; 163],
}

impl KeyBoard {
    pub(crate) fn new() -> Self {
        Self {
            present: [KeyState::Released; 163],
            previous: [KeyState::Released; 163],
        }
    }

    //Updates keyboard states before polling events
    pub(crate) fn pre_update(&mut self) {
        self.previous = self.present;
    }

    pub(crate) fn handle_input(&mut self, keyboard_input: winit::event::KeyboardInput) {
        if let Some(key) = keyboard_input.virtual_keycode {
            let key = key as usize;
            let state = match keyboard_input.state {
                winit::event::ElementState::Pressed => KeyState::Pressed,
                winit::event::ElementState::Released => KeyState::Released,
            };
            self.present[key] = state;
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.present[key as usize] == KeyState::Pressed
    }

    pub fn are_pressed(&self, keys: &[KeyCode]) -> bool {
        keys.iter()
            .all(|key| self.present[*key as usize] == KeyState::Pressed)
    }

    pub fn is_released(&self, key: KeyCode) -> bool {
        self.present[key as usize] == KeyState::Released
    }

    pub fn are_released(&self, keys: &[KeyCode]) -> bool {
        keys.iter()
            .all(|key| self.present[*key as usize] == KeyState::Released)
    }

    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        let key = key as usize;
        self.present[key] == KeyState::Pressed && self.previous[key] == KeyState::Released
    }

    pub fn are_just_pressed(&self, keys: &[KeyCode]) -> bool {
        keys.iter().all(|key| {
            let key = *key as usize;
            self.present[key] == KeyState::Pressed && self.previous[key] == KeyState::Released
        })
    }

    pub fn is_just_released(&self, key: KeyCode) -> bool {
        let key = key as usize;
        self.present[key] == KeyState::Released && self.previous[key] == KeyState::Pressed
    }

    pub fn are_just_released(&self, keys: &[KeyCode]) -> bool {
        keys.iter().all(|key| {
            let key = *key as usize;
            self.present[key] == KeyState::Released && self.previous[key] == KeyState::Pressed
        })
    }
}
