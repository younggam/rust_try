use super::GlobalState;

#[repr(u32)]
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

///Table for keyboard's keys whether pressed
pub struct Keyboard {
    present: [KeyState; 163],
    prev: [KeyState; 163],
    changed: Vec<usize>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            present: [KeyState::Released; 163],
            prev: [KeyState::Released; 163],
            changed: Vec::new(),
        }
    }

    #[cfg(feature = "winit")]
    pub fn handle_input(&mut self, input: winit::event::KeyboardInput) {
        use winit::event::*;

        if let Some(key) = input.virtual_keycode {
            let key = key as usize;
            self.prev[key] = self.present[key];
            self.present[key] = match input.state {
                ElementState::Pressed => KeyState::Pressed,
                ElementState::Released => KeyState::Released,
            };
            self.changed.push(key);
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.present[key as usize] == KeyState::Pressed
    }

    pub fn is_released(&self, key: KeyCode) -> bool {
        self.present[key as usize] == KeyState::Released
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        let key = key as usize;
        self.present[key] == KeyState::Pressed && self.prev[key] == KeyState::Released
    }

    pub fn just_released(&self, key: KeyCode) -> bool {
        let key = key as usize;
        self.present[key] == KeyState::Released && self.prev[key] == KeyState::Pressed
    }
}

impl GlobalState for Keyboard {
    ///Updates keyboard states before polling events
    fn pre_update(&mut self) {
        for key in self.changed.drain(..) {
            self.prev[key] = self.present[key]
        }
    }
}
