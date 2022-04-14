use super::time::*;

pub struct Utils {
    time: Time,
}

impl Utils {
    pub(crate) fn new() -> Self {
        Self { time: Time::new() }
    }

    pub fn time(&self) -> f64 {
        self.time.time()
    }

    pub fn delta(&self) -> f64 {
        self.time.delta()
    }
}

impl Utils {
    pub(crate) fn update(&mut self) {
        self.time.update()
    }
}
