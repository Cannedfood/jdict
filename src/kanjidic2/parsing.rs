use roxmltree::NodeType;
use smallvec::SmallVec;

use super::reading_meaning::{Meaning, Reading, ReadingMeaning, ReadingMeaningGroup, ReadingType};
use super::{Character, CharacterMetadata, Header};

pub fn parse_kanjidic2(xml: roxmltree::Node) -> (Header, Vec<Character>) {
    assert_eq!(xml.tag_name().name(), "kanjidic2");

    let mut header = Header {
        file_version:     "".into(),
        database_version: "".into(),
        date_of_creation: "".into(),
    };
    let mut characters = Vec::new();

    for node in xml.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Element, "header") => header = parse_header(node),
            (NodeType::Element, "character") => characters.push(parse_character(node)),
            (NodeType::Text | NodeType::Comment, _) => (),
            (ty, name) => panic!("Unexpected child in <kanjidic2>: {:?} {}", ty, name),
        }
    }

    (header, characters)
}

fn parse_header(node: roxmltree::Node) -> Header {
    assert_eq!(node.tag_name().name(), "header");

    let mut header = Header {
        file_version:     "".into(),
        database_version: "".into(),
        date_of_creation: "".into(),
    };

    for node in node.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Element, "file_version") => {
                header.file_version = node.text().unwrap().into()
            }
            (NodeType::Element, "database_version") => {
                header.database_version = node.text().unwrap().into()
            }
            (NodeType::Element, "date_of_creation") => {
                header.date_of_creation = node.text().unwrap().into()
            }
            (NodeType::Text | NodeType::Comment, _) => (),
            (ty, name) => panic!("Unexpected child in <header>: {:?} {}", ty, name),
        }
    }

    header
}

fn parse_character(node: roxmltree::Node) -> Character {
    assert_eq!(node.tag_name().name(), "character");

    let mut character = Character {
        unicode: ' ',
        radicals: SmallVec::new(),
        radicals_nelson_c: None,
        misc: CharacterMetadata {},
        dic_number: (),
        query_code: (),
        reading_meaning: SmallVec::new(),
    };

    for node in node.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Element, "literal") => (),
            (NodeType::Element, "codepoint") => {
                for node in node.children() {
                    match (node.node_type(), node.tag_name().name()) {
                        (NodeType::Element, "cp_value") => {
                            let cp_type = node.attribute("cp_type").unwrap();
                            let value = node.text().unwrap();
                            if cp_type == "ucs" {
                                let value = u32::from_str_radix(value, 16).unwrap();
                                character.unicode = std::char::from_u32(value).unwrap()
                            }
                        }
                        (NodeType::Text, _) => (),
                        (ty, name) => {
                            panic!("Unexpected child in <codepoint>: {:?} {}", ty, name)
                        }
                    }
                }
            }
            (NodeType::Element, "radical") => {
                for node in node.children() {
                    match (node.node_type(), node.tag_name().name()) {
                        (NodeType::Element, "rad_value") => {
                            let rad_type = node.attribute("rad_type").unwrap();
                            let value = node.text().unwrap();
                            match rad_type {
                                "classical" => character.radicals.push(value.parse().unwrap()),
                                "nelson_c" => {
                                    if character.radicals_nelson_c.is_none() {
                                        character.radicals_nelson_c = Some(SmallVec::new());
                                    }
                                    character
                                        .radicals_nelson_c
                                        .get_or_insert_with(SmallVec::new)
                                        .push(value.parse().unwrap());
                                }
                                _ => panic!("Unexpected rad_type '{}'", rad_type),
                            }
                        }
                        (NodeType::Text, _) => (),
                        (ty, name) => {
                            panic!("Unexpected child in <radical>: {:?} {}", ty, name)
                        }
                    }
                }
            }
            (NodeType::Element, "misc") => character.misc = parse_character_metadata(node),
            (NodeType::Element, "dic_number") => (),
            (NodeType::Element, "query_code") => (),
            (NodeType::Element, "reading_meaning") => {
                character.reading_meaning.push(parse_reading_meaning(node))
            }
            (NodeType::Text, _) => (),
            (ty, name) => panic!("Unexpected child in <character>: {:?} {}", ty, name),
        }
    }

    character
}

fn parse_character_metadata(node: roxmltree::Node) -> CharacterMetadata {
    assert_eq!(node.tag_name().name(), "misc");

    // TODO

    CharacterMetadata {}
}

fn parse_reading_meaning(node: roxmltree::Node) -> ReadingMeaning {
    assert_eq!(node.tag_name().name(), "reading_meaning");

    let mut nanori = Vec::new();
    let mut reading_meaning_groups = Vec::new();

    for node in node.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Element, "nanori") => {
                nanori.push(node.text().unwrap().into());
            }
            (NodeType::Element, "rmgroup") => {
                reading_meaning_groups.push(parse_reading_meaning_group(node));
            }
            (NodeType::Text, _) => (),
            (ty, name) => panic!("Unexpected child in <reading_meaning>: {:?} {}", ty, name),
        }
    }

    ReadingMeaning {
        nanori,
        reading_meaning_groups,
    }
}

fn parse_reading_meaning_group(node: roxmltree::Node) -> ReadingMeaningGroup {
    assert_eq!(node.tag_name().name(), "rmgroup");

    let mut readings = Vec::new();
    let mut meanings = Vec::new();

    for node in node.children() {
        match (node.node_type(), node.tag_name().name()) {
            (NodeType::Element, "reading") => readings.push(parse_reading(node)),
            (NodeType::Element, "meaning") => meanings.push(parse_meaning(node)),
            (NodeType::Text, _) => (),
            (ty, name) => panic!("Unexpected child in <rmgroup>: {:?} {}", ty, name),
        }
    }

    ReadingMeaningGroup { readings, meanings }
}

fn parse_reading(node: roxmltree::Node) -> Reading {
    assert_eq!(node.tag_name().name(), "reading");

    let jouyou = node.attribute("r_type").map_or(false, |x| x == "ja_jlpt");
    let typ = match node.attribute("r_type") {
        Some("pinyin") => ReadingType::Pinyin,
        Some("korean_r") => ReadingType::KoreanRomanized,
        Some("korean_h") => ReadingType::Hangul,
        Some("ja_kun") => ReadingType::Kunyomi,
        Some("ja_on") => ReadingType::Onyomi(
            node.attribute("on_type")
                .map(|on_type| on_type.parse().unwrap()),
        ),
        _ => panic!("Unexpected r_type"),
    };
    let value = node.text().unwrap().into();

    Reading { jouyou, typ, value }
}

fn parse_meaning(node: roxmltree::Node) -> Meaning {
    assert_eq!(node.tag_name().name(), "meaning");

    let lang = node
        .attribute("m_lang")
        .map(|m_lang| m_lang.parse().unwrap())
        .unwrap_or(isolang::Language::Eng);
    let text = node.text().unwrap().into();

    Meaning { lang, text }
}
