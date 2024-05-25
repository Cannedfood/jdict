#![allow(unused)]

use std::io::{BufReader, BufWriter, Read};
use std::path::Path;
use std::sync::{Arc, OnceLock};

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
    dirty: bool,

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
            dirty: true,
            text:  "".to_string(),

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
            self.dirty |= ui.text_edit_singleline(&mut self.text).changed();
        });
    }

    fn show_weight_editor(&mut self, ui: &mut egui::Ui) {
        ui.label("Weights:");

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

#[derive(Default)]
struct App {
    search:  Search,
    results: Vec<(u32, u32)>,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("weights").show(ctx, |ui| {
            self.search.show_weight_editor(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.search.show_searchbox(ui);

            let Some(database) = DICTIONARY.get()
            else {
                ui.horizontal(|ui| {
                    ui.label("Loading...");
                    ui.spinner();
                });
                return;
            };

            if std::mem::replace(&mut self.search.dirty, false) {
                let timer = std::time::Instant::now();

                self.search.apply(database, &mut self.results);

                println!(
                    "Found {} entries in {:?}",
                    self.results.len(),
                    timer.elapsed()
                );
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (entry_idx, score) in self.results.iter().take(512) {
                    let entry = &database.dictionary[*entry_idx as usize];

                    ui.horizontal(|ui| {
                        for (i, kanji) in entry.kanji.iter().enumerate() {
                            ui.label(kanji.text.as_str());
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
                }
            });
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
