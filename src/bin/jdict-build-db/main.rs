use std::io::{BufReader, Read};
use std::path::Path;

use jdict2::database::Database;

fn main() {
    let db = load_from_source();
    std::fs::write("./res/database.blob", postcard::to_allocvec(&db).unwrap()).unwrap();
}

pub fn load_from_source() -> Database {
    pub fn load_gzip_xml(path: impl AsRef<Path>, buffer: &mut Vec<u8>) -> roxmltree::Document<'_> {
        buffer.clear();

        flate2::read::GzDecoder::new(BufReader::new(std::fs::File::open(path.as_ref()).unwrap()))
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
    let dictionary = jdict2::jmdict::parsing::parse_jmdict(
        load_gzip_xml("./res/JMdict_e.gz", &mut buffer).root_element(),
    );

    let (_, kanji_dictionary) = jdict2::kanjidic2::parse_kanjidic2(
        load_gzip_xml("./res/kanjidic2.xml.gz", &mut buffer).root_element(),
    );
    let kanji_dictionary = kanji_dictionary
        .into_iter()
        .map(|entry| (entry.unicode, entry))
        .collect();

    let kanji_strokes = jdict2::kanjivg::parse_kanjivg(
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
