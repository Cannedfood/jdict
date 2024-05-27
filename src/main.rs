#![allow(unused)]

use std::collections::HashSet;
use std::io::{BufReader, BufWriter, Read};
use std::mem::take;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use egui::ahash::HashMap;
use jdict2::jmdict::{self, Entry};
use jdict2::kana::{romaji_to, KanaType};
use jdict2::kanjivg::StrokeGroup;
use jdict2::{kanjidic2, kanjivg};

#[derive(serde::Serialize, serde::Deserialize)]
struct Database {
    dictionary: Vec<jmdict::Entry>,
    kanji_dictionary: HashMap<char, kanjidic2::Character>,
    kanji_strokes: HashMap<char, kanjivg::StrokeGroup>,
}
impl Database {
    fn load_from_source() -> Database {
        fn load_gzip_xml(path: impl AsRef<Path>, buffer: &mut String) -> roxmltree::Document<'_> {
            buffer.clear();

            flate2::read::GzDecoder::new(std::fs::File::open(path).unwrap())
                .read_to_string(buffer)
                .unwrap();

            roxmltree::Document::parse_with_options(buffer, roxmltree::ParsingOptions {
                allow_dtd: true,
                ..Default::default()
            })
            .unwrap()
        }

        let mut buffer = String::new();
        let dictionary = jmdict::parsing::parse_jmdict(
            load_gzip_xml("./res/JMdict_e.gz", &mut buffer).root_element(),
        );

        let (_, kanji_dictionary) = kanjidic2::parse_kanjidic2(
            load_gzip_xml("./res/kanjidic2.xml.gz", &mut buffer).root_element(),
        );
        let kanji_dictionary = kanji_dictionary
            .into_iter()
            .map(|entry| (entry.unicode, entry))
            .collect();

        let kanji_strokes = kanjivg::parse_kanjivg(
            load_gzip_xml("./res/kanjivg.xml.gz", &mut buffer).root_element(),
        )
        .into_iter()
        .map(|entry| (entry.element.unwrap(), entry))
        .collect();

        Database {
            dictionary,
            kanji_dictionary,
            kanji_strokes,
        }
    }

    fn save_cache(&self) {
        std::fs::write("./res/database.cache", postcard::to_allocvec(self).unwrap());
    }

    fn load_cache() -> Self {
        let file = std::fs::File::open("./res/database.cache").unwrap();
        let mem = unsafe { memmap2::Mmap::map(&file) }.unwrap();
        mem.advise(memmap2::Advice::Sequential).unwrap();
        mem.advise(memmap2::Advice::WillNeed).unwrap();
        mem.advise(memmap2::Advice::DontFork).unwrap();

        postcard::from_bytes(&mem).unwrap()
    }

    fn load() -> Self {
        if Path::new("./res/database.cache").exists() {
            println!("Loading from cache...");
            let timer = std::time::Instant::now();
            let result = Self::load_cache();
            println!("Loaded in {:?}", timer.elapsed());

            result
        }
        else {
            println!("Cache not found, loading from source...");
            let timer = std::time::Instant::now();
            let result = Self::load_from_source();
            println!("Loaded in {:?}", timer.elapsed());
            result.save_cache();

            result
        }
    }
}

static DICTIONARY: OnceLock<Database> = OnceLock::new();

struct Search {
    request_focus: bool,
    dirty: bool,
    last_search: Instant,

    text: String,

    weight_kanji: u32,
    weight_kanji_position_panelty_pct: u32,
    weight_reading: u32,
    weight_reading_position_panelty_pct: u32,
    weight_sense: u32,
    weight_sense_position_panelty_pct: u32,
    weight_gloss_position_panelty_pct: u32,

    weight_exact: u32,
    weight_word_exact: u32,
    weight_startswith: u32,
    weight_word_startswith: u32,
    weight_contains: u32,
    weight_position_panelty_pct: u32,
}
impl Default for Search {
    fn default() -> Self {
        Self {
            request_focus: true,
            dirty: true,
            last_search: Instant::now(),

            text: "".to_string(),

            weight_kanji: 3,
            weight_kanji_position_panelty_pct: 100,
            weight_reading: 2,
            weight_reading_position_panelty_pct: 100,
            weight_sense: 1,
            weight_sense_position_panelty_pct: 100,
            weight_gloss_position_panelty_pct: 100,

            weight_exact: 5,
            weight_word_exact: 4,
            weight_startswith: 3,
            weight_word_startswith: 2,
            weight_contains: 1,
            weight_position_panelty_pct: 100,
        }
    }
}
impl Search {
    fn show_searchbox(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Search:");
            let search_box = ui.text_edit_singleline(&mut self.text);
            if take(&mut self.request_focus) {
                search_box.request_focus();
            }

            self.dirty |= search_box.changed();
        });
    }

    fn show_weight_editor(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("weights").show(ui, |ui| {
            ui.label("Kanji");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_kanji))
                .changed();
            ui.end_row();

            ui.label("Kanji Position Penalty");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_kanji_position_panelty_pct).suffix("%"))
                .changed();
            ui.end_row();

            ui.label("Reading");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_reading))
                .changed();
            ui.end_row();

            ui.label("Reading Position Penalty");
            self.dirty |= ui
                .add(
                    egui::DragValue::new(&mut self.weight_reading_position_panelty_pct).suffix("%"),
                )
                .changed();
            ui.end_row();

            ui.label("Sense");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_sense))
                .changed();
            ui.end_row();

            ui.label("Sense Position Penalty");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_sense_position_panelty_pct).suffix("%"))
                .changed();
            ui.end_row();

            ui.label("Gloss Position Penalty");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_gloss_position_panelty_pct).suffix("%"))
                .changed();
            ui.end_row();

            ui.separator();
            ui.end_row();

            ui.label("Exact");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_exact))
                .changed();
            ui.end_row();

            ui.label("Word Exact");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_word_exact))
                .changed();
            ui.end_row();

            ui.label("Starts With");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_startswith))
                .changed();
            ui.end_row();

            ui.label("Word Starts With");
            self.dirty |= ui
                .add(egui::DragValue::new(&mut self.weight_word_startswith))
                .changed();
            ui.end_row();
        });
    }

    fn apply(&self, database: &Database, result: &mut Vec<(u32, u32)>) {
        result.clear();

        if self.text.trim().len() < 3 {
            return;
        }

        let mut pieces = Vec::<String>::new();
        for piece in self.text.split_whitespace() {
            pieces.push(piece.to_string());

            let (failures, hiragana) = romaji_to(KanaType::Hiragana, piece);
            if failures == 0 {
                pieces.push(hiragana);
            }

            let (failures, katakana) = romaji_to(KanaType::Katakana, piece);
            if failures == 0 {
                pieces.push(katakana);
            }
        }
        if pieces.is_empty() {
            return;
        }

        println!("Searching for {:?}", pieces);

        for (i, entry) in database.dictionary.iter().enumerate() {
            let mut score = 0;
            for piece in &pieces {
                for (kanji_idx, kanji) in entry.kanji.iter().enumerate() {
                    if let Some(match_score) = self.text_match(piece, &kanji.text) {
                        score = score.max(Self::apply_position_penalty(
                            self.weight_kanji_position_panelty_pct,
                            match_score * self.weight_kanji,
                            kanji_idx as u32,
                        ))
                    }
                }

                for (reading_idx, reading) in entry.reading.iter().enumerate() {
                    if let Some(match_score) = self.text_match(piece, &reading.text) {
                        score = score.max(Self::apply_position_penalty(
                            self.weight_reading_position_panelty_pct,
                            match_score * self.weight_reading,
                            reading_idx as u32,
                        ))
                    }
                }

                for (sense_idx, sense) in entry.sense.iter().enumerate() {
                    for (gloss_idx, gloss) in sense.glosses.iter().enumerate() {
                        if let Some(match_score) = self.text_match(piece, &gloss.text) {
                            let gloss_score = match_score * self.weight_sense;
                            let gloss_score = Self::apply_position_penalty(
                                self.weight_sense_position_panelty_pct,
                                score,
                                sense_idx as u32,
                            );
                            let gloss_score = Self::apply_position_penalty(
                                self.weight_gloss_position_panelty_pct,
                                score,
                                gloss_idx as u32,
                            );

                            score = score.max(gloss_score)
                        }
                    }
                }
            }

            if score > 0 {
                result.push((i as u32, score));
            }
        }

        result.sort_unstable_by_key(|(_, score)| std::cmp::Reverse(*score));
    }

    fn apply_position_penalty(panelty_pct: u32, score: u32, position: u32) -> u32 {
        if score == 0 {
            return 0;
        }

        score
            .saturating_sub(1 + position * panelty_pct / 100)
            .saturating_add(1)
    }

    fn text_match(&self, term: &str, text: &str) -> Option<u32> {
        fn next_char(text: &str, pos: usize) -> Option<char> { text[pos..].chars().next() }
        fn prev_char(text: &str, pos: usize) -> Option<char> { text[..pos].chars().next_back() }

        let pos = text.find(term)?;

        let exact_match = text == term;
        let starts_with = pos == 0;
        let word_starts_with = pos == 0
            || prev_char(text, pos)
                .map(|c| !c.is_alphabetic())
                .unwrap_or(false);
        let word_exact_match = word_starts_with
            && (pos + term.len() == text.len()
                || next_char(text, pos + term.len())
                    .map(|c| !c.is_alphabetic())
                    .unwrap_or(false));

        Some(if exact_match {
            self.weight_exact
        }
        else if word_exact_match {
            self.weight_word_exact
        }
        else if starts_with {
            self.weight_startswith
        }
        else if word_starts_with {
            self.weight_word_startswith
        }
        else {
            self.weight_contains
        })
    }
}

struct Pagination {
    page: usize,
    page_size: usize,
    page_changed: bool,
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

#[derive(Default)]
struct App {
    show_settings: bool,

    search: Search,
    pagination: Pagination,
    results: Vec<(u32, u32)>,
    kanji_results: Vec<char>,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
        egui::SidePanel::right("kanji").show_animated(ctx, !self.kanji_results.is_empty(), |ui| {
            ui.label("Kanji");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for k in &self.kanji_results {
                    ui.label(k.to_string());
                }
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
            if self.search.last_search + Duration::from_millis(100) < now
                && take(&mut self.search.dirty)
            {
                self.search.last_search = now;

                self.search.apply(database, &mut self.results);

                println!(
                    "Found {} entries in {:?}",
                    self.results.len(),
                    now.elapsed()
                );
            }

            let mut symbols = HashSet::new();
            let mut ordered_symbols = Vec::new();
            self.pagination
                .show_entries(ui, &self.results, |ui, _, (entry_idx, score)| {
                    let entry = &database.dictionary[*entry_idx as usize];

                    let visible = {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(ui.available_width(), 1.0),
                            egui::Sense::hover(),
                        );
                        ui.is_rect_visible(rect)
                    };

                    ui.horizontal(|ui| {
                        for (i, kanji) in entry.kanji.iter().enumerate() {
                            ui.label(kanji.text.as_str());
                            if visible {
                                for k in kanji.text.chars() {
                                    if !symbols.contains(&k) {
                                        symbols.insert(k);
                                        ordered_symbols.push(k);
                                    }
                                }
                            }
                        }
                        ui.label(format!(" ({score})"));
                    });

                    ui.horizontal(|ui| {
                        for reading in &entry.reading {
                            ui.label(format!(
                                "{}{}",
                                if reading.no_kanji { "【" } else { "" },
                                reading.text.as_str()
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
                    ui.separator();
                });

            self.kanji_results.clear();
            for k in ordered_symbols {
                if let Some(kanji) = database.kanji_dictionary.get(&k) {
                    self.kanji_results.push(k);
                }
            }
        });
    }
}

fn main() {
    std::thread::spawn(|| {
        DICTIONARY.get_or_init(Database::load);
    });

    eframe::run_native(
        "jdict2",
        eframe::NativeOptions::default(),
        Box::new(|cx| {
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

            cx.egui_ctx.set_fonts(fonts);

            Box::new(App::default())
        }),
    )
    .unwrap();
}
