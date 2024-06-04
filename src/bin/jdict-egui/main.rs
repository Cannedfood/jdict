mod debounce;
mod pagination;
mod search_box;
mod stroke_animation;

use std::mem::take;
use std::sync::OnceLock;
use std::time::Instant;

use egui::global_dark_light_mode_buttons;
use itertools::Itertools;
use jdict2::jmdict;
use jdict2::kanjidic2::ReadingType;

static DICTIONARY: OnceLock<jdict2::database::Database> = OnceLock::new();

#[derive(Default)]
struct App {
    show_settings: bool,
    show_kanji:    bool,

    search: search_box::SearchBox,
    pagination: pagination::Pagination,
    search_debounce: debounce::Debounce,

    results: Vec<(u32, u32)>,
    kanji_results: Vec<char>,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(duration) = self.search_debounce.will_resolve_in() {
            ctx.request_repaint_after(duration);
        }

        egui::SidePanel::left("settings").show_animated(ctx, self.show_settings, |ui| {
            ui.heading("Settings");

            global_dark_light_mode_buttons(ui);

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
        egui::SidePanel::left("kanji").show_animated(ctx, self.show_kanji, |ui| {
            let Some(database) = DICTIONARY.get()
            else {
                ui.horizontal(|ui| {
                    ui.label("Loading...");
                    ui.spinner();
                });
                return;
            };

            egui::ScrollArea::vertical().show(ui, |ui| {
                for character in &self.kanji_results {
                    let info = &database.kanji_dictionary[character];
                    let strokes = &database.kanji_strokes[character];
                    ui.horizontal(|ui| {
                        stroke_animation::kanji_stroke_animation(
                            ui,
                            30.0,
                            (1.0, egui::Color32::BLACK).into(),
                            strokes,
                        );
                        ui.vertical(|ui| {
                            for rm in info.reading_meaning.iter() {
                                for rm in rm.reading_meaning_groups.iter() {
                                    let readings = rm
                                        .readings
                                        .iter()
                                        .filter(|r| {
                                            matches!(
                                                r.typ,
                                                ReadingType::Kunyomi | ReadingType::Onyomi(_)
                                            )
                                        })
                                        .map(|r| &r.value)
                                        .join(", ");
                                    ui.label(readings);

                                    let meanings = rm
                                        .meanings
                                        .iter()
                                        .filter(|m| m.lang == isolang::Language::Eng)
                                        .map(|m| &m.text)
                                        .join(", ");
                                    ui.label(meanings);
                                }
                            }
                        });
                    });
                }
            });
        });
        egui::TopBottomPanel::top("Search").show(ctx, |ui| {
            triptichon_layout(
                ui,
                |ui| {
                    ui.toggle_value(&mut self.show_settings, "\u{2699}\u{FE0F}");
                    ui.toggle_value(&mut self.show_kanji, "事");
                },
                |ui| {
                    self.pagination.show_controls(ui, self.results.len());
                },
                |ui| {
                    self.search.show_searchbox(ui);
                },
            );
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

            // draw_kanji_strokes(ui, 100.0, database.kanji_strokes.get(&'何').unwrap());

            if self
                .search_debounce
                .trigger_and_poll_if(take(&mut self.search.changed))
            {
                let timer = Instant::now();
                jdict2::dictionary_search::search(
                    &self.search.text,
                    &self.search.search_weights,
                    &database.dictionary,
                    &mut self.results,
                );
                println!(
                    "Found {} entries in {:?}",
                    self.results.len(),
                    timer.elapsed()
                );
            }

            self.kanji_results.clear();
            self.pagination
                .show_entries(ui, &self.results, |ui, _, (entry_idx, _score)| {
                    let entry = &database.dictionary[*entry_idx as usize];
                    let entry_visible = render_entry(ui, entry);
                    ui.separator();

                    if entry_visible {
                        for c in entry.kanji.iter().flat_map(|k| k.text.chars()) {
                            if database.kanji_dictionary.contains_key(&c)
                                && !self.kanji_results.contains(&c)
                            {
                                self.kanji_results.push(c);
                            }
                        }
                    }
                });
        });
    }
}

fn triptichon_layout(
    ui: &mut egui::Ui,
    left: impl FnOnce(&mut egui::Ui),
    right: impl FnOnce(&mut egui::Ui),
    center: impl FnOnce(&mut egui::Ui),
) {
    let available = ui.available_rect_before_wrap();
    let left_divider = available.left() + available.width() / 6.0;
    let right_divider = available.right() - available.width() / 6.0;

    // Center 2/3rds
    let mut center_ui = ui.child_ui(
        egui::Rect::from_x_y_ranges(left_divider..=right_divider, available.y_range()),
        egui::Layout::top_down(egui::Align::Center),
    );
    center(&mut center_ui);

    let left_divider = left_divider.min(center_ui.min_rect().left());
    let right_divider = right_divider.max(center_ui.min_rect().right());

    // Left 1/6th
    let mut left_ui = ui.child_ui(
        egui::Rect::from_x_y_ranges(available.left()..=left_divider, available.y_range()),
        egui::Layout::left_to_right(egui::Align::Center),
    );
    left(&mut left_ui);

    // Right 1/6th
    let mut right_ui = ui.child_ui(
        egui::Rect::from_x_y_ranges(right_divider..=available.right(), available.y_range()),
        egui::Layout::right_to_left(egui::Align::Center),
    );
    right(&mut right_ui);

    ui.advance_cursor_after_rect(egui::Rect::from_points(&[
        left_ui.min_rect().min,
        left_ui.min_rect().max,
        center_ui.min_rect().min,
        center_ui.min_rect().max,
        right_ui.min_rect().min,
        right_ui.min_rect().max,
    ]));
}

fn render_entry(ui: &mut egui::Ui, entry: &jmdict::Entry) -> bool {
    let mut visible = false;

    ui.horizontal(|ui| {
        for kanji in &entry.kanji {
            let res = ui.label(kanji.text.as_str());
            if ui.is_rect_visible(res.rect) {
                visible = true;
            }
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

    visible
}

fn main() {
    std::thread::spawn(|| {
        DICTIONARY.get_or_init(|| {
            postcard::from_bytes(include_bytes!("../../../res/database.blob")).unwrap()
        });
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
    fonts.font_data.insert(
        "JP".into(),
        egui::FontData::from_static(include_bytes!("../../../res/NotoSansCJKjp-Regular.otf")),
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
