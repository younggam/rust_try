use std::sync::Mutex;

use crate::system::EventRegistry;

use lazy_static::initialize;

lazy_static! {
    pub static ref EVENT_REGISTRY: Mutex<EventRegistry> = Mutex::new(EventRegistry::new());
}

pub fn init_globals() {
    initialize(&EVENT_REGISTRY);
}
