#![allow(unused)]

mod debounce;
mod pagination;
mod search_box;

use std::collections::HashSet;
use std::io::{BufReader, BufWriter, Read};
use std::mem::take;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, LazyLock, OnceLock};
use std::time::Instant;

use egui::ahash::HashMap;
use egui::{global_dark_light_mode_buttons, Pos2, Vec2};
use jdict2::jmdict::{self};
use jdict2::kana::{romaji_to, KanaType};
use jdict2::kanjivg::{self, Coord, StrokeGroup};

static DICTIONARY: OnceLock<jdict2::database::Database> = OnceLock::new();

#[derive(Default)]
struct App {
    show_settings: bool,

    search: search_box::SearchBox,
    pagination: pagination::Pagination,
    search_debounce: debounce::Debounce,

    results: Vec<(u32, u32)>,
    kanji_results: Vec<char>,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
        egui::TopBottomPanel::top("Search").show(ctx, |ui| {
            triptichon_layout(
                ui,
                |ui| {
                    ui.toggle_value(&mut self.show_settings, "\u{2699}\u{FE0F}");
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

            ui.horizontal(|ui| {
                for character in &self.kanji_results {
                    // let info = &database.kanji_dictionary[&character];
                    let strokes = &database.kanji_strokes[character];
                    draw_kanji_strokes(ui, 30.0, (1.0, egui::Color32::BLACK).into(), strokes);
                }
            });

            self.kanji_results.clear();
            self.pagination
                .show_entries(ui, &self.results, |ui, _, (entry_idx, score)| {
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
    let left_divider = (available.left() + available.width() / 6.0);
    let right_divider = (available.right() - available.width() / 6.0);

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
        for (i, kanji) in entry.kanji.iter().enumerate() {
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

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

fn draw_kanji_strokes(ui: &mut egui::Ui, size: f32, brush: egui::Stroke, kanji: &StrokeGroup) {
    let (rect, _) = ui.allocate_exact_size((size, size).into(), egui::Sense::hover());

    let time = (START_TIME.elapsed().as_secs_f32() % 5.0) / 5.0;

    let mut f = time * measure(kanji);

    ui.painter().rect_filled(rect, 3.0, egui::Color32::GRAY);

    draw_recursive(&ui.painter_at(rect.shrink(3.0)), kanji, brush, &mut f);

    fn measure(kanji: &StrokeGroup) -> f32 {
        kanji
            .subgroups
            .iter()
            .map(|child| match child {
                kanjivg::Child::Group(group) => measure(group),
                kanjivg::Child::Stroke(stroke) => stroke.path.length(),
            })
            .sum()
    }

    fn draw_recursive(
        painter: &egui::Painter,
        path: &kanjivg::StrokeGroup,
        brush: egui::Stroke,
        length_budget: &mut f32,
    ) {
        for child in &path.subgroups {
            match child {
                kanjivg::Child::Stroke(stroke) => {
                    draw_path(painter, &stroke.path, brush, length_budget);
                }
                kanjivg::Child::Group(group) => {
                    draw_recursive(painter, group, brush, length_budget);
                }
            }
        }
    }
    fn draw_path(
        painter: &egui::Painter,
        path: &kanjivg::Path,
        brush: egui::Stroke,
        length_budget: &mut f32,
    ) {
        let scale = painter
            .clip_rect()
            .width()
            .min(painter.clip_rect().height());

        let offset = painter.clip_rect().min;

        let mut brush_position = Vec2::new(0.0, 0.0);
        for cmd in &path.0 {
            match cmd {
                kanjivg::Command::MoveTo(Coord { x, y }) => {
                    brush_position = Vec2::new(*x, *y);
                }
                kanjivg::Command::LineTo(Coord { x, y }) => {
                    painter.line_segment(
                        [
                            offset + brush_position * scale,
                            offset + Vec2::new(*x, *y) * scale,
                        ],
                        brush,
                    );
                    brush_position = Vec2::new(*x, *y);
                }
                kanjivg::Command::CubicBezier(c1, c2, to) => {
                    let c1 = Vec2::new(c1.x, c1.y);
                    let c2 = Vec2::new(c2.x, c2.y);
                    let to = Vec2::new(to.x, to.y);
                    for (a, b) in [(brush_position, c1), (c1, c2), (c2, to)] {
                        painter.add(take_line_segment(
                            &painter.clip_rect(),
                            a,
                            b,
                            brush,
                            length_budget,
                        ));
                    }
                    brush_position = to;
                }
                _ => {}
            }
        }

        fn take_line_segment(
            rect: &egui::Rect,
            from: Vec2,
            to: Vec2,
            stroke: egui::Stroke,
            length_budget: &mut f32,
        ) -> egui::Shape {
            let length = (to - from).length();
            if length > *length_budget {
                *length_budget = 0.0;
                return egui::Shape::Noop;
            }

            let length = length.min(*length_budget);
            *length_budget -= length;

            let to = from + (to - from).normalized() * length;

            let from = rect.min + from * rect.width();
            let to = rect.min + to * rect.width();

            egui::Shape::line_segment([from, to], stroke)
        }
    }
}
