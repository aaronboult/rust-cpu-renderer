// time based structs and functions, notably deltatime

use std::time::{Duration, Instant};

pub struct Time {
    delta_time: usize,
    last_update: Instant
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: 0,
            last_update: Instant::now()
        }
    }

    pub fn update(&mut self) -> usize {
        self.last_update = Instant::now();
        self.delta_time = self.last_update.elapsed().as_millis() as usize
    }

    pub fn get_delta_time(&self) -> usize {
        self.delta_timec
    }
}