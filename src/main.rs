#![allow(unused)]

use std::io::{BufReader, BufWriter, Read};
use std::path::Path;
use std::sync::{Arc, OnceLock};

use egui::ahash::HashMap;
use jdict2::jmdict::{self, Entry};
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

#[derive(Default)]
struct App {
    search: String,
    search_result: Vec<&'static Entry>,
    search_dirty: bool,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                if ui.text_edit_singleline(&mut self.search).changed() {
                    self.search_dirty = true;
                }
            });

            let Some(database) = DICTIONARY.get()
            else {
                ui.horizontal(|ui| {
                    ui.label("Loading...");
                    ui.spinner();
                });
                return;
            };

            if std::mem::replace(&mut self.search_dirty, false) {
                self.search_dirty = false;

                let timer = std::time::Instant::now();

                let pieces = self.search.split_whitespace().collect::<Vec<_>>();
                print!("Searching for:");
                for (i, piece) in pieces.iter().enumerate() {
                    if i != 0 {
                        print!(" AND ");
                    }
                    print!("'{}'", piece);
                }
                println!();

                // Search sense
                let entries: Vec<u32> = database
                    .dictionary
                    .iter()
                    .enumerate()
                    .filter(|(i, entry)| {
                        pieces.iter().all(|piece| {
                            entry.sense.iter().any(|sense| {
                                sense.glosses.iter().any(|gloss| gloss.text.contains(piece))
                            })
                        })
                    })
                    .map(|(i, entry)| i as u32)
                    .take(512)
                    .collect();

                println!(
                    "Found {} entries in {:?}",
                    self.search_result.len(),
                    timer.elapsed()
                );
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.search_result {
                    ui.horizontal(|ui| {
                        for (i, kanji) in entry.kanji.iter().enumerate() {
                            ui.label(kanji.text.as_str());
                        }
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
