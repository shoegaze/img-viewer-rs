use winit::window::WindowId;

use std::time::Instant;

#[derive(Copy, Clone)]
pub struct ClickContext {
    target: WindowId,
    time: Instant,
}

impl ClickContext {
    pub fn new(target: WindowId, time: Instant) -> Self {
        ClickContext { target, time }
    }

    pub fn from_time_now(target: WindowId) -> Self {
        let time_now = Instant::now();

        ClickContext::new(target, time_now)
    }

    pub fn is_same_target(&self, other: &Self) -> bool {
        self.target == other.target
    }

    pub fn get_time(&self) -> Instant {
        self.time
    }
}
