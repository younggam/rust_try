use super::{buttons::*, mock::*};

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
    Key0, //10
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J, //20
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T, //30
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3, //40
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13, //50
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23, //60
    F24,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp, //70
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock, //80
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9, //90
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,
    AbntC1,
    AbntC2, //100
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma, //110
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin, //120
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert, //130
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift, //140
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown, //150
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen, //160
    Copy,
    Paste,
    Cut, //163
}

impl KeyCode {
    const LEN: usize = 163;
}

impl From<KeyCode> for usize {
    fn from(key: KeyCode) -> Self {
        key as usize
    }
}

pub struct Keyboard {
    buttons: Buttons,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            buttons: Buttons::new(KeyCode::LEN),
        }
    }
}

impl Keyboard {
    pub(crate) fn pre_update(&mut self) {
        self.buttons.pre_update();
    }

    pub(crate) fn handle_input(&mut self, keyboard_input: winit::event::KeyboardInput) {
        if let Some(key) = keyboard_input.virtual_keycode {
            self.buttons
                .handle_input(key as usize, keyboard_input.state);
        }
    }
}

impl std::ops::Deref for Keyboard {
    type Target = Buttons;

    fn deref(&self) -> &Self::Target {
        &self.buttons
    }
}

impl From<MockDevice> for Keyboard {
    fn from(_: MockDevice) -> Self {
        Self::new()
    }
}
