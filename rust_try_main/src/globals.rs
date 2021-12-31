// use std::sync::Mutex;
//
// use rust_try_lib::system::EventRegistry;
// use rust_try_lib::utils::LazyManual;
//
// pub static EVENT_REGISTRY: LazyManual<Mutex<EventRegistry>> = LazyManual::new();
//
// pub fn init_globals() {
//     EVENT_REGISTRY.init(Mutex::new(EventRegistry::new()));
// }
