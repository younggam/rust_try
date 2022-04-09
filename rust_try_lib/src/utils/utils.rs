use super::time::*;

pub struct Utils {
    time: Time,
}

impl Utils {
    pub(crate) fn new() -> Self {
        Self { time: Time::new() }
    }

    pub(crate) fn update(&mut self) {
        self.time.update()
    }
}
