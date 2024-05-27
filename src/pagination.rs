use std::mem::take;

pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
    pub page_changed: bool,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 100,
            page_changed: false,
        }
    }
}

impl Pagination {
    pub fn show_controls(&mut self, ui: &mut egui::Ui, entries: usize) {
        let pages = entries / self.page_size;
        if pages == 0 {
            return;
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let has_next_page = self.page < pages;
            let has_prev_page = self.page > 0;

            let next_page = ui
                .add_enabled(has_next_page, egui::Button::new("→"))
                .clicked();
            ui.label(format!("Page {}/{}", self.page + 1, pages + 1));
            let prev_page = ui
                .add_enabled(has_prev_page, egui::Button::new("←"))
                .clicked();

            if next_page {
                self.page_changed = true;
                self.page += 1;
            }
            if prev_page {
                self.page_changed = true;
                self.page -= 1;
            }
        });
    }

    pub fn show_entries<T>(
        &mut self,
        ui: &mut egui::Ui,
        entries: &[T],
        mut renderer: impl FnMut(&mut egui::Ui, usize, &T),
    ) {
        let mut scroll_area = egui::ScrollArea::vertical();
        if take(&mut self.page_changed) {
            // Scroll to top when page changes
            scroll_area = scroll_area.vertical_scroll_offset(0.0);
        }

        scroll_area.show(ui, |ui| {
            for (i, entry) in entries
                .iter()
                .enumerate()
                .skip(self.page * self.page_size)
                .take(self.page_size)
            {
                renderer(ui, i, entry);
            }
        });
    }
}
