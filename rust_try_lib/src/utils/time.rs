//!Provides global time per frame.
use std::time::*;

pub struct Time {
    base_instant: Instant,
    time: f64,
    delta: f64,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
            base_instant: Instant::now(),
            time: 0.0,
            delta: 0.0,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn delta(&self) -> f64 {
        self.delta
    }

    pub fn this(&self) -> f64 {
        self.base_instant.elapsed().as_secs_f64()
    }

    pub fn this_delta(&self) -> f64 {
        self.this() - self.time
    }
}

impl Time {
    pub(crate) fn pre_update(&mut self) {
        let this_instant = Instant::now();
        let past_time = self.time;

        match this_instant.checked_duration_since(self.base_instant) {
            Some(duration) => {
                self.time = duration.as_secs_f64();
                self.delta = self.time - past_time;
            }
            None => {
                self.base_instant = this_instant;
                self.time = 0.0;
                self.delta = 0.0;
            }
        }
    }
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
