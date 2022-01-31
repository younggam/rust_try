use crate::utils::LazyManual;

use std::sync::RwLock;

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

static PRESENT: LazyManual<RwLock<[KeyState; 163]>> = LazyManual::new();
static PREVIOUS: LazyManual<RwLock<[KeyState; 163]>> = LazyManual::new();
//# SAFETY
//This isn't exposed to be mutual accessible state. Has used only in main thread manually.
static mut QUEUE: LazyManual<Vec<(usize, KeyState)>> = LazyManual::new();

pub(crate) fn init() {
    PRESENT.init(RwLock::new([KeyState::Released; 163]));
    PREVIOUS.init(RwLock::new([KeyState::Released; 163]));
    unsafe { QUEUE.init(Vec::with_capacity(4)) };
}

pub(crate) fn fin() {
    PRESENT.fin();
    PREVIOUS.fin();
    unsafe { QUEUE.fin() };
}

//Updates keyboard states before polling events
pub(crate) fn pre_update() {
    let present = PRESENT.read().unwrap();
    let mut previous = PREVIOUS.write().unwrap();
    for (key, _) in unsafe { QUEUE.drain(..) } {
        previous[key] = present[key];
    }
}

//To prevent too frequent write lock.
pub(crate) fn enqueue(key: usize, state: KeyState) {
    unsafe { QUEUE.push((key, state)) };
}

//resolves queued input.
pub(crate) fn update() {
    let mut present = PRESENT.write().unwrap();
    let mut previous = PREVIOUS.write().unwrap();

    for (key, state) in unsafe { QUEUE.iter() } {
        previous[*key] = present[*key];
        present[*key] = *state;
    }
}

pub fn is_pressed(key: KeyCode) -> bool {
    let present = PRESENT.read().unwrap();
    present[key as usize] == KeyState::Pressed
}

pub fn are_pressed(keys: &[KeyCode]) -> bool {
    let present = PRESENT.read().unwrap();
    keys.iter()
        .all(|key| present[*key as usize] == KeyState::Pressed)
}

pub fn is_released(key: KeyCode) -> bool {
    let present = PRESENT.read().unwrap();
    present[key as usize] == KeyState::Released
}

pub fn are_released(keys: &[KeyCode]) -> bool {
    let present = PRESENT.read().unwrap();
    keys.iter()
        .all(|key| present[*key as usize] == KeyState::Released)
}

pub fn is_just_pressed(key: KeyCode) -> bool {
    let present = PRESENT.read().unwrap();
    let previous = PREVIOUS.read().unwrap();
    let key = key as usize;
    present[key] == KeyState::Pressed && previous[key] == KeyState::Released
}

pub fn are_just_pressed(keys: &[KeyCode]) -> bool {
    let present = PRESENT.read().unwrap();
    let previous = PREVIOUS.read().unwrap();
    keys.iter().all(|key| {
        let key = *key as usize;
        present[key] == KeyState::Pressed && previous[key] == KeyState::Released
    })
}

pub fn is_just_released(key: KeyCode) -> bool {
    let present = PRESENT.read().unwrap();
    let previous = PREVIOUS.read().unwrap();
    let key = key as usize;
    present[key] == KeyState::Released && previous[key] == KeyState::Pressed
}

pub fn are_just_released(keys: &[KeyCode]) -> bool {
    let present = PRESENT.read().unwrap();
    let previous = PREVIOUS.read().unwrap();
    keys.iter().all(|key| {
        let key = *key as usize;
        present[key] == KeyState::Released && previous[key] == KeyState::Pressed
    })
}
