#![allow(unused)]

mod database;
mod debounce;
mod dictionary_search;
mod pagination;

use std::collections::HashSet;
use std::io::{BufReader, BufWriter, Read};
use std::mem::take;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use dictionary_search::SearchWeights;
use egui::ahash::HashMap;
use jdict2::jmdict::{self};
use jdict2::kana::{romaji_to, KanaType};
use jdict2::kanjivg::StrokeGroup;

static DICTIONARY: OnceLock<database::Database> = OnceLock::new();

#[derive(Default)]
struct Search {
    changed: bool,
    request_focus: bool,
    text: String,
    search_weights: SearchWeights,
}
impl Search {
    fn show_searchbox(&mut self, ui: &mut egui::Ui) {
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

    fn show_weight_editor(&mut self, ui: &mut egui::Ui) {
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

#[derive(Default)]
struct App {
    show_settings: bool,

    search: Search,
    pagination: pagination::Pagination,
    search_debounce: debounce::Debounce,

    results: Vec<(u32, u32)>,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(duration) = self.search_debounce.will_resolve_in() {
            ctx.request_repaint_after(duration);
        }

        egui::SidePanel::left("weights").show_animated(ctx, self.show_settings, |ui| {
            egui::CollapsingHeader::new("Pagination")
                .default_open(true)
                .show_unindented(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Page Size:");
                        ui.add(
                            egui::DragValue::new(&mut self.pagination.page_size)
                                .clamp_range(1..=10000),
                        );
                    });
                });
            egui::CollapsingHeader::new("Weights")
                .default_open(true)
                .show_unindented(ui, |ui| {
                    self.search.show_weight_editor(ui);
                })
        });
        egui::TopBottomPanel::top("Search").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut self.show_settings, "\u{2699}\u{FE0F}");
                self.search.show_searchbox(ui);
                self.pagination.show_controls(ui, self.results.len());
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let Some(database) = DICTIONARY.get()
            else {
                ui.horizontal(|ui| {
                    ui.label("Loading...");
                    ui.spinner();
                });
                return;
            };

            let now = Instant::now();
            if self
                .search_debounce
                .trigger_and_poll_if(take(&mut self.search.changed))
            {
                dictionary_search::search(
                    &self.search.text,
                    &self.search.search_weights,
                    &database.dictionary,
                    &mut self.results,
                );
                println!(
                    "Found {} entries in {:?}",
                    self.results.len(),
                    now.elapsed()
                );
            }

            self.pagination
                .show_entries(ui, &self.results, |ui, _, (entry_idx, score)| {
                    render_entry(ui, &database.dictionary[*entry_idx as usize]);
                    ui.separator();
                });
        });
    }
}

fn render_entry(ui: &mut egui::Ui, entry: &jmdict::Entry) {
    ui.horizontal(|ui| {
        for (i, kanji) in entry.kanji.iter().enumerate() {
            ui.label(kanji.text.as_str());
        }
    });

    ui.horizontal(|ui| {
        for reading in &entry.reading {
            ui.label(format!(
                "{}{}{}",
                if reading.no_kanji { "【" } else { "" },
                reading.text.as_str(),
                if reading.no_kanji { "】" } else { "" }
            ));
        }
    });

    for sense in &entry.sense {
        ui.horizontal(|ui| {
            let mut text = " • ".to_string();
            for (i, gloss) in sense.glosses.iter().enumerate() {
                if i != 0 {
                    text.push_str(", ");
                }
                text.push_str(&gloss.text);
            }
            ui.label(text);
        });
    }
}

fn main() {
    std::thread::spawn(|| {
        DICTIONARY.get_or_init(database::Database::load);
    });

    eframe::run_native(
        "jdict2",
        eframe::NativeOptions::default(),
        Box::new(|cx| {
            cx.egui_ctx.set_fonts({
                let mut fonts = egui::FontDefinitions::default();
                default_fonts_plus_japanese_font(&mut fonts);
                fonts
            });

            Box::new(App::default())
        }),
    )
    .unwrap();
}

fn default_fonts_plus_japanese_font(fonts: &mut egui::FontDefinitions) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "JP".into(),
        egui::FontData::from_static(include_bytes!("../res/NotoSansCJKjp-Regular.otf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("JP".into());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("JP".into());
}
