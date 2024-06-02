use std::mem::take;

#[derive(Default)]
pub(crate) struct SearchBox {
    pub(crate) changed: bool,
    pub(crate) request_focus: bool,
    pub(crate) text: String,
    pub(crate) search_weights: jdict2::dictionary_search::SearchWeights,
}

impl SearchBox {
    pub(crate) fn show_searchbox(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Search:");
            let search_box = ui.text_edit_singleline(&mut self.text);

            let sent = search_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if take(&mut self.request_focus) || sent {
                search_box.request_focus();
            }

            self.changed = search_box.changed();
        });
    }

    pub(crate) fn show_weight_editor(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("weights").show(ui, |ui| {
            ui.label("Kanji");
            self.changed |= ui
                .add(egui::DragValue::new(&mut self.search_weights.kanji))
                .changed();
            ui.end_row();

            ui.label("Kanji Position Penalty");
            self.changed |= ui
                .add(
                    egui::DragValue::new(&mut self.search_weights.kanji_position_penalty_pct)
                        .suffix("%"),
                )
                .changed();
            ui.end_row();

            ui.label("Reading");
            self.changed |= ui
                .add(egui::DragValue::new(&mut self.search_weights.reading))
                .changed();
            ui.end_row();

            ui.label("Reading Position Penalty");
            self.changed |= ui
                .add(
                    egui::DragValue::new(&mut self.search_weights.reading_position_penalty_pct)
                        .suffix("%"),
                )
                .changed();
            ui.end_row();

            ui.label("Sense");
            self.changed |= ui
                .add(egui::DragValue::new(&mut self.search_weights.sense))
                .changed();
            ui.end_row();

            ui.label("Sense Position Penalty");
            self.changed |= ui
                .add(
                    egui::DragValue::new(&mut self.search_weights.sense_position_penalty_pct)
                        .suffix("%"),
                )
                .changed();
            ui.end_row();

            ui.label("Gloss Position Penalty");
            self.changed |= ui
                .add(
                    egui::DragValue::new(&mut self.search_weights.gloss_position_penalty_pct)
                        .suffix("%"),
                )
                .changed();
            ui.end_row();

            ui.separator();
            ui.end_row();

            ui.label("Exact");
            self.changed |= ui
                .add(egui::DragValue::new(&mut self.search_weights.exact))
                .changed();
            ui.end_row();

            ui.label("Word Exact");
            self.changed |= ui
                .add(egui::DragValue::new(&mut self.search_weights.word_exact))
                .changed();
            ui.end_row();

            ui.label("Starts With");
            self.changed |= ui
                .add(egui::DragValue::new(&mut self.search_weights.starts_with))
                .changed();
            ui.end_row();

            ui.label("Word Starts With");
            self.changed |= ui
                .add(egui::DragValue::new(
                    &mut self.search_weights.word_starts_with,
                ))
                .changed();
            ui.end_row();
        });
    }
}
