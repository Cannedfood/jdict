use std::mem::take;
use std::time::{Duration, Instant};

pub struct Debounce {
    pub last:  Instant,
    pub dirty: bool,

    pub delay: Duration,
}

impl Default for Debounce {
    fn default() -> Self {
        Self {
            last:  Instant::now(),
            delay: Duration::from_millis(100),
            dirty: false,
        }
    }
}

impl Debounce {
    pub fn trigger(&mut self) {
        self.last = Instant::now();
        self.dirty = true;
    }

    pub fn poll(&mut self) -> bool {
        if self.last.elapsed() <= self.delay {
            return false;
        }

        take(&mut self.dirty)
    }

    pub fn trigger_and_poll_if(&mut self, condition: bool) -> bool {
        if condition {
            self.trigger();
        }

        self.poll()
    }

    pub fn will_resolve_in(&self) -> Option<Duration> {
        self.delay.checked_sub(self.last.elapsed())
    }
}
