#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod egui_svg_path;
mod history;

use std::sync::mpsc;

use egui::{vec2, ScrollArea};
use history::History;
use itertools::Itertools;
use jdict_shared::{
    database::DictData,
    kanjidic::ReadingType,
    shared_api::{self, SearchResult, DB_LOADING},
};

struct JDictApp {
    search_field: String,

    search_history: History<String>,
    results: Option<SearchResult<'static>>,

    inbox: mpsc::Receiver<SearchResult<'static>>,
    send_results: mpsc::Sender<SearchResult<'static>>,
}
impl JDictApp {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            search_field: String::new(),
            search_history: Default::default(),
            results: None,
            inbox: rx,
            send_results: tx,
        }
    }

    fn execute_search(&mut self, search: String, ctx: egui::Context) {
        println!("Search: '{}'", search);
        self.results = None;

        if !search.is_empty() {
            let sender = self.send_results.clone();
            std::thread::spawn(move || {
                sender
                    .send(shared_api::search(&search, None, None))
                    .unwrap();
                ctx.request_repaint();
            });
        }
    }
}

impl JDictApp {
    fn draw_searchbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            let re = ui.add(egui::TextEdit::singleline(&mut self.search_field).hint_text("Search"));

            // On enter
            if re.lost_focus() && re.ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
                self.search_history.push(self.search_field.clone());
            }

            // Always focus the search bar if there's nothing else focused
            if !ui.ctx().memory(|m| m.has_focus(re.id)) {
                re.request_focus();
            }

            if let Some(results) = &self.results {
                ui.label(format!(
                    "{} results in {}",
                    results.results_total, results.time
                ));
            }

            if DB_LOADING.load(std::sync::atomic::Ordering::Relaxed) {
                ui.spinner();
                ui.label("loading dictionary...");
            }
        });
    }

    fn draw_results(&mut self, ui: &mut egui::Ui) {
        if let Some(results) = &self.results {
            ScrollArea::vertical()
                .min_scrolled_width(ui.available_width())
                .show(ui, |ui| {
                    for entry in &results.results {
                        ui.small(entry.readings.iter().map(|r| &r.value).join(", "));
                        ui.horizontal(|ui| {
                            for kanji in &entry.kanji {
                                if ui.small_button(&kanji.value).clicked() {
                                    self.search_history.push(kanji.value.clone());
                                }
                            }
                        });
                        for sense in &entry.senses {
                            ui.label(format!(
                                " - {}",
                                sense.glosses.iter().map(|g| &g.value).join(", ")
                            ));
                        }
                        ui.separator();
                    }
                });
        }
    }

    fn draw_kanji_infos(&mut self, ui: &mut egui::Ui) {
        if let Some(result) = &self.results {
            ScrollArea::vertical()
                .min_scrolled_width(ui.available_width())
                .show(ui, |ui| {
                    ui.heading("Kanji");
                    for kanji in &result.kanji {
                        let kanjivg = result.kanjivg.iter().find(|vg| vg.kanji == kanji.literal);

                        ui.separator();
                        ui.horizontal(|ui| {
                            let kanjivg_res = match kanjivg {
                                Some(kanjivg) => draw_kanjivg(ui, kanjivg, egui::Sense::click()),
                                None => ui.button(&kanji.literal),
                            };
                            if kanjivg_res.clicked() {
                                self.search_history.push(kanji.literal.clone());
                            }

                            ui.vertical(|ui| {
                                for rm in &kanji.reading_meaning_groups {
                                    let kun = rm
                                        .readings
                                        .iter()
                                        .filter(|r| r.typ == ReadingType::ja_kun)
                                        .map(|r| &r.value)
                                        .join(", ");
                                    let on = rm
                                        .readings
                                        .iter()
                                        .filter(|r| r.typ == ReadingType::ja_on)
                                        .map(|r| &r.value)
                                        .join(", ");

                                    ui.label(
                                        rm.meanings
                                            .iter()
                                            .filter(|m| m.lang == "en")
                                            .map(|m| &m.value)
                                            .join(", "),
                                    );

                                    if !kun.is_empty() {
                                        ui.small(format!("kun: {}", kun));
                                    }
                                    if !on.is_empty() {
                                        ui.small(format!("on: {}", on));
                                    }
                                }
                            });
                        });

                        // if let Some(kanjivg) = kanjivg {
                        //     draw_kanji_hierarchy(ui, kanjivg, None);
                        // }
                    }
                });
        }
    }
}

fn draw_kanjivg(
    ui: &mut egui::Ui,
    kanji: &jdict_shared::kanjivg::Kanji,
    sense: egui::Sense,
) -> egui::Response {
    let (rect, res) = ui.allocate_exact_size(vec2(50.0, 50.0), sense);

    let painter = ui.painter_at(rect);
    let style = ui.style().interact(&res);
    painter.rect_filled(rect, 0.0, style.weak_bg_fill);

    draw_strokes_recursive(kanji, rect, &painter, style.fg_stroke);

    fn draw_strokes_recursive(
        kanjivg: &jdict_shared::kanjivg::Kanji,
        rect: egui::Rect,
        painter: &egui::Painter,
        stroke_style: egui::Stroke,
    ) {
        for stroke in kanjivg.strokes.iter() {
            for segment in egui_svg_path::egui_svg_path(
                &stroke.path,
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(109.0)),
                rect,
                stroke_style,
            ) {
                painter.add(segment);
            }
        }

        for child in kanjivg.parts.iter() {
            draw_strokes_recursive(child, rect, painter, stroke_style);
        }
    }

    res
}

fn draw_kanji_hierarchy(
    ui: &mut egui::Ui,
    kanji: &jdict_shared::kanjivg::Kanji,
    origin: Option<egui::Pos2>,
) {
    if kanji.parts.is_empty() {
        draw_label(ui, kanji, origin);
    } else {
        ui.horizontal(|ui| {
            let new_origin = draw_label(ui, kanji, origin);
            ui.spacing();
            ui.vertical(|ui| {
                for part in kanji.parts.iter() {
                    draw_kanji_hierarchy(ui, part, Some(new_origin));
                }
            });
        });
    }

    fn draw_label(
        ui: &mut egui::Ui,
        kanji: &jdict_shared::kanjivg::Kanji,
        origin: Option<egui::Pos2>,
    ) -> egui::Pos2 {
        let res = ui.button(&kanji.kanji);
        if let Some(origin) = origin {
            ui.painter().line_segment(
                [origin, res.rect.left_center()],
                ui.style().interact(&res).fg_stroke,
            );
        }
        res.rect.right_center()
    }
}

impl eframe::App for JDictApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(3.0);

        if let Ok(results) = self.inbox.try_recv() {
            self.results = Some(results);
        }

        ctx.input_mut(|input| {
            if input.pointer.button_clicked(egui::PointerButton::Extra1)
                || input.pointer.button_pressed(egui::PointerButton::Middle)
                || input.consume_shortcut(&egui::KeyboardShortcut::new(
                    egui::Modifiers::CTRL,
                    egui::Key::Z,
                ))
            {
                self.search_history.back();
            }

            if input.pointer.button_clicked(egui::PointerButton::Extra2) {
                self.search_history.forward();
            }
        });

        egui::TopBottomPanel::top("search_bar").show(ctx, |ui| self.draw_searchbar(ui));

        if let Some(result) = &self.results {
            if !result.kanji.is_empty() {
                egui::SidePanel::right("kanji_panel").show(ctx, |ui| {
                    self.draw_kanji_infos(ui);
                });
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.results.is_some() {
                self.draw_results(ui);
            }
            // else {
            //     ui.heading("jdict-egui");
            //     ui.label("A simple Japanese dictionary written in Rust using egui");
            // }
        });

        if self.search_history.has_changed() {
            self.search_field = self.search_history.current().clone();
            self.execute_search(self.search_history.current().clone(), ctx.clone());
        }
    }
}

fn main() {
    shared_api::parse_db_async(DictData::<'static> {
        dict: include_bytes!("../../../res/JMdict_e.gz"),
        kanjidic: include_bytes!("../../../res/kanjidic2.xml.gz"),
        kanjivg: include_bytes!("../../../res/kanjivg.xml.gz"),
    });

    eframe::run_native(
        "jdict-egui",
        eframe::NativeOptions::default(),
        Box::new(|ctx| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "JP".into(),
                egui::FontData::from_static(include_bytes!(
                    "../../../res/NotoSansCJKjp-Regular.otf"
                )),
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

            ctx.egui_ctx.set_fonts(fonts);

            Box::new(JDictApp::new())
        }),
    )
    .unwrap();
}
