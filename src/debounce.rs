use std::mem::take;
use std::time::{Duration, Instant};

pub(crate) struct Debounce {
    pub(crate) last:  Instant,
    pub(crate) dirty: bool,

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
    pub(crate) fn trigger(&mut self) {
        self.last = Instant::now();
        self.dirty = true;
    }

    pub(crate) fn poll(&mut self) -> bool {
        if self.last.elapsed() <= self.delay {
            return false;
        }

        take(&mut self.dirty)
    }

    pub(crate) fn trigger_and_poll_if(&mut self, condition: bool) -> bool {
        if condition {
            self.trigger();
        }

        self.poll()
    }

    pub(crate) fn will_resolve_in(&self) -> Option<Duration> {
        self.delay.checked_sub(self.last.elapsed())
    }
}
