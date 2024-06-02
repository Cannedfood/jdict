use std::io::{BufReader, Read};
use std::path::Path;
use std::time::Instant;

use egui::ahash::HashMap;
use jdict2::jmdict::{self, Entry};
use jdict2::{kanjidic2, kanjivg};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Database {
    pub dictionary: Vec<jmdict::Entry>,
    pub kanji_dictionary: HashMap<char, kanjidic2::Character>,
    pub kanji_strokes: HashMap<char, kanjivg::StrokeGroup>,
}

impl Database {
    pub fn load_from_source() -> Database {
        pub fn load_gzip_xml(
            path: impl AsRef<Path>,
            buffer: &mut Vec<u8>,
        ) -> roxmltree::Document<'_> {
            buffer.clear();

            flate2::read::GzDecoder::new(BufReader::new(
                std::fs::File::open(path.as_ref()).unwrap(),
            ))
            .read_to_end(buffer)
            .unwrap();

            roxmltree::Document::parse_with_options(
                unsafe { std::str::from_utf8_unchecked(buffer) },
                roxmltree::ParsingOptions {
                    allow_dtd: true,
                    ..Default::default()
                },
            )
            .unwrap()
        }

        let mut buffer = Vec::new();
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

    pub fn save_cache(&self) {
        std::fs::write("./res/database.cache", postcard::to_allocvec(self).unwrap());
    }

    pub fn load_cache() -> Self {
        let file = std::fs::File::open("./res/database.cache").unwrap();
        let mem = unsafe { memmap2::Mmap::map(&file) }.unwrap();
        #[cfg(unix)]
        {
            mem.advise(memmap2::Advice::Sequential).unwrap();
            mem.advise(memmap2::Advice::WillNeed).unwrap();
            mem.advise(memmap2::Advice::DontFork).unwrap();
        }

        postcard::from_bytes(&mem).unwrap()
    }

    pub fn load() -> Self {
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
