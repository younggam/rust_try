use crate::*;
use application::ApplicationWinit;
use graphics::ash::GraphicsCoreAsh;
use utils::{LazyManual, MutOnlyOnMainThread, UnsafeRef};

use super::*;

use std::ops::Deref;
use std::sync::Mutex;

///abstract global states' behave. Not necessary that all global variables should impl.
pub(super) trait GlobalState {
    fn pre_update(&mut self);
}

///Should be public?
pub(crate) fn init() {
    lazy_static::initialize(&TIME);
    lazy_static::initialize(&KEYBOARD);
    lazy_static::initialize(&EVENT_REGISTRY);
    lazy_static::initialize(&GRAPHICS);
}

pub(crate) fn pre_update() {
    unsafe {
        TIME.get_mut().pre_update();
        KEYBOARD.get_mut().pre_update();
    }
}

pub(crate) fn finalize() {
    use std::ptr::drop_in_place;
    unsafe {
        drop_in_place(GRAPHICS.get_mut() as *mut _);
        drop_in_place(EVENT_REGISTRY.deref() as *const _ as *mut Mutex<EventRegistry>);
        drop_in_place(KEYBOARD.get_mut() as *mut _);
        drop_in_place(TIME.get_mut() as *mut _);
    }
}

lazy_static! {
    ///actually, RWLock is more proper but I suppose that read-on-write doesn't have much affect.
    pub static ref TIME: MutOnlyOnMainThread<Time> = MutOnlyOnMainThread::new(Time::new());
    ///I assume KEYBOARD won't be read or write on other threads. If not RWLock is more proper
    pub static ref KEYBOARD: MutOnlyOnMainThread<Keyboard> =
        MutOnlyOnMainThread::new(Keyboard::new());
    pub static ref EVENT_REGISTRY: Mutex<EventRegistry> = Mutex::new(EventRegistry::new());
    #[cfg(feature = "vulkan")]
    pub static ref GRAPHICS: MutOnlyOnMainThread<GraphicsCoreAsh> = MutOnlyOnMainThread::new(GraphicsCoreAsh::new());
}

///Maybe safe, because its address would stick inside closure.
#[cfg(feature = "winit")]
pub static APPLICATION: LazyManual<UnsafeRef<ApplicationWinit>> = LazyManual::new();
