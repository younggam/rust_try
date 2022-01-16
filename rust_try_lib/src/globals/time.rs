use super::GlobalState;

use std::time::*;

///Global time
pub struct Time {
    pivot: Instant,
    time: f64,
    delta: f64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            pivot: Instant::now(),
            time: 0f64,
            delta: 0f64,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn delta(&self) -> f64 {
        self.delta
    }
}

impl GlobalState for Time {
    ///Updates time vars
    fn pre_update(&mut self) {
        let new_pivot = Instant::now();
        let past_time = self.time;

        //Is this logic ensures to cover if-and-only-if overflow
        match new_pivot.checked_duration_since(self.pivot) {
            Some(elapsed) => {
                self.time = elapsed.as_secs_f64();
                self.delta = self.time - past_time;
            }
            None => {
                self.pivot = new_pivot;
                self.time = 0f64;
                self.delta = f64::MAX - past_time;
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
