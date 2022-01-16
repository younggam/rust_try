use std::sync::Mutex;

use super::*;
use crate::application::ApplicationWinit;
use crate::utils::{LazyManual, MutOnlyOnMainThread, UnsafeRef};

///abstract global states' behave. Not necessary that all global variables should impl.
pub(super) trait GlobalState {
    fn pre_update(&mut self);
}

///Should be public?
pub(crate) fn init() {
    lazy_static::initialize(&globals::TIME);
    lazy_static::initialize(&globals::KEYBOARD);
    lazy_static::initialize(&globals::EVENT_REGISTRY);
}

pub(crate) fn pre_update() {
    unsafe {
        TIME.get_mut().pre_update();
        KEYBOARD.get_mut().pre_update();
    }
}

lazy_static! {
    ///actually, RWLock is more proper but I suppose that read-on-write doesn't have much affect.
    pub static ref TIME: MutOnlyOnMainThread<Time> = MutOnlyOnMainThread::new(Time::new());
    ///I assume KEYBOARD won't be read or write on other threads. If not RWLock is more proper
    pub static ref KEYBOARD: MutOnlyOnMainThread<Keyboard> =
        MutOnlyOnMainThread::new(Keyboard::new());
    pub static ref EVENT_REGISTRY: Mutex<EventRegistry> = Mutex::new(EventRegistry::new());
}

#[cfg(feature = "winit")]
pub static APPLICATION_WINIT: LazyManual<UnsafeRef<ApplicationWinit>> = LazyManual::new();
