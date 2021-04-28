// time based structs and functions, notably deltatime

// reexport the Instant and Duration structs
pub use std::time::{Instant, Duration};

pub struct Time {
    delta_time: f32,
    last_update: Instant
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            last_update: Instant::now()
        }
    }

    pub fn update(&mut self) -> f32 {
        self.delta_time = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();
        self.delta_time
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}