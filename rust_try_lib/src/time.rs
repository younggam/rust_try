//!Provides global time per frame.
use crate::utils::LazyManual;

use std::sync::*;
use std::time::*;

//# SAFETY
//This isn't exposed to be mutual accessible state. Has used only in main thread manually.
static mut INITIAL_INSTANT: LazyManual<Instant> = LazyManual::new();
static TIME: LazyManual<RwLock<f64>> = LazyManual::new();
static DELTA: LazyManual<RwLock<f64>> = LazyManual::new();

pub(crate) fn init() {
    unsafe { INITIAL_INSTANT.init(Instant::now()) };
    TIME.init(RwLock::new(0f64));
    DELTA.init(RwLock::new(0f64));
}

pub(crate) fn fin() {
    unsafe { INITIAL_INSTANT.fin() };
    TIME.fin();
    DELTA.fin();
}

pub(crate) fn update() {
    let mut time = TIME.write().unwrap();
    let mut delta = DELTA.write().unwrap();

    let this_instant = Instant::now();
    let past_time = *time;

    match this_instant.checked_duration_since(unsafe { INITIAL_INSTANT.clone() }) {
        Some(duration) => {
            *time = duration.as_secs_f64();
            *delta = *time - past_time;
        }
        None => {
            unsafe { *INITIAL_INSTANT = this_instant };
            *time = 0f64;
            *delta = f64::MAX - past_time;
        }
    }
}

pub fn time() -> f64 {
    *TIME.read().unwrap()
}

pub fn delta() -> f64 {
    *DELTA.read().unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn what_is_nan() {
        println!("{}", 0f32 / 0f32);
        println!("{}", f64::NAN == f64::NAN);
        println!("{}", f64::INFINITY == f64::INFINITY);
    }
}
