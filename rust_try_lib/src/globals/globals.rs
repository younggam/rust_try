use std::sync::Mutex;

use super::*;

use lazy_static::initialize;

lazy_static! {
    pub static ref EVENT_REGISTRY: Mutex<EventRegistry> = Mutex::new(EventRegistry::new());
}

///no effect on multiple calls
pub fn init_globals() {
    initialize(&EVENT_REGISTRY);
}
