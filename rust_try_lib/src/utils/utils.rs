use super::time::*;

pub struct Utils {
    time: Time,
}

impl Utils {
    pub(crate) fn new() -> Self {
        Self { time: Time::new() }
    }
}

impl Utils {
    pub(crate) fn pre_update(&mut self) {
        self.time.pre_update()
    }
}

impl Utils{
    pub fn time(&self) -> f64 {
        self.time.time()
    }

    pub fn time_delta(&self) -> f64 {
        self.time.delta()
    }
}
