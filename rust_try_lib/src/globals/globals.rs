use std::sync::Mutex;

use super::*;
use crate::application::ApplicationWinit;
use crate::utils::{LazyManual, MutOnlyOnMainThread, UnsafeRef};

//No variations
lazy_static! {
    pub static ref TIME: MutOnlyOnMainThread<Time> = MutOnlyOnMainThread::new(Time::new());
    pub static ref KEYBOARD: MutOnlyOnMainThread<Keyboard> =
        MutOnlyOnMainThread::new(Keyboard::new());
    pub static ref EVENT_REGISTRY: Mutex<EventRegistry> = Mutex::new(EventRegistry::new());
}

#[cfg(feature = "winit")]
pub static APPLICATION_WINIT: LazyManual<UnsafeRef<ApplicationWinit>> = LazyManual::new();
