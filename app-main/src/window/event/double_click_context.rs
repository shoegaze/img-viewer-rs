use winit::window::WindowId;

use std::time::Duration;

use crate::window::event::click_context::ClickContext;

// TODO: Replace with app config
static MAX_DOUBLE_CLICK_DURATION: Duration = Duration::from_millis(500);

type ClicksTracker = [Option<ClickContext>; 2];

#[derive(Copy, Clone, Default)]
pub struct DoubleClickContext {
    pub clicks: ClicksTracker,
}

impl DoubleClickContext {
    pub fn push_time_now(&mut self, target: WindowId) {
        let [_, click] = &self.clicks;
        let click_now = ClickContext::from_time_now(target);

        self.clicks = [click.clone(), Some(click_now)];
    }

    pub fn is_double_click(&self) -> bool {
        let [Some(old_click), Some(new_click)] = self.clicks else {
            return false;
        };

        if !old_click.is_same_target(&new_click) {
            return false;
        }

        let clicks_duration = new_click.get_time() - old_click.get_time();

        clicks_duration <= MAX_DOUBLE_CLICK_DURATION
    }

    pub fn reset_clicks(&mut self) {
        self.clicks = [None, None];
    }
}
