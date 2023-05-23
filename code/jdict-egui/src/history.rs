pub struct History<T: Default + Clone + Eq> {
    pub history: Vec<T>,
    pub index: usize,
    pub past_depth: usize,
    pub future_depth: usize,
    has_changed: bool,
}
impl<T: Default + Clone + Eq> Default for History<T> {
    fn default() -> Self {
        Self {
            history: vec![Default::default()],
            index: 0,
            past_depth: 20,
            future_depth: 20,
            has_changed: false,
        }
    }
}
impl<T: Default + Clone + Eq> History<T> {
    // Getters
    pub fn current(&self) -> &T {
        self.history.get(self.index).unwrap()
    }

    // Updating
    pub fn push(&mut self, item: T) -> bool {
        if &item == self.current() {
            return false;
        }
        self.index += 1;
        self.history.insert(self.index, item);
        self.mark_changed();

        while self.index > self.past_depth {
            self.history.remove(0);
            self.index -= 1;
        }
        while self.history.len() > self.index + self.future_depth {
            self.history.pop();
        }

        true
    }
    pub fn replace(&mut self, item: T) {
        self.history[self.index] = item;
        self.mark_changed();
    }
    pub fn back(&mut self) -> &T {
        if self.index > 0 {
            self.index -= 1;
            self.mark_changed();
        }
        self.current()
    }
    pub fn forward(&mut self) -> &T {
        if self.index + 1 < self.history.len() {
            self.index += 1;
            self.mark_changed();
        }
        self.current()
    }

    // Detecting changes
    pub fn has_changed(&mut self) -> bool {
        let had_changed = self.has_changed;
        self.has_changed = false;
        had_changed
    }

    // Internal
    fn mark_changed(&mut self) {
        self.has_changed = true;
    }
}
