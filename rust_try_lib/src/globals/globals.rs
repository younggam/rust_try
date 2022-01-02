use std::sync::Mutex;

use super::*;
use crate::application::ApplicationWinit;
use crate::utils::LazyManual;

//No variations
lazy_static! {
    pub static ref EVENT_REGISTRY: Mutex<EventRegistry> = Mutex::new(EventRegistry::new());
}

#[cfg(feature = "winit")]
pub static APPLICATION_WINIT: LazyManual<Mutex<ApplicationWinit>> = LazyManual::new();
