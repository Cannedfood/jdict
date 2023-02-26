use std::{path::Path, iter};

use roxmltree::{Node, ParsingOptions};

use crate::{kanjivg::{KanjiVG, Kanji, Stroke}, util::read_file};

impl KanjiVG {
    pub fn load(path: &Path) -> Self {
        let file_content = read_file(path).unwrap();
        Self::parse(&file_content)
    }

    pub fn parse(file_content: &str) -> Self {
        let document = roxmltree::Document::parse_with_options(
            file_content,
            ParsingOptions {
                allow_dtd: true,
                ..Default::default()
            }
        ).unwrap();

        let kanjivg = document.root_element();
        if kanjivg.tag_name().name() != "kanjivg" {
            panic!("Invalid root element <{}>, expected <kanjivg>", kanjivg.tag_name().name());
        }

        let mut result = KanjiVG::default();

        for node in kanjivg.children() {
            match node.tag_name().name() {
                "kanji" => result.kanji.push(parse_kanji(node)),
                ""      => (),                                                  // Ignore text nodes (whitespace)
                name    => panic!("Unknown element <{}> in <kanjivg>", name),
            }
        }

        result
    }

}

fn parse_kanji(node: Node) -> Kanji {
    let id = node.attribute("id").unwrap();
    let c = char::from_u32(
        u32::from_str_radix(
            &id[id.len() - 5..],
            16
        ).unwrap()
    ).unwrap();

    let mut kanji = parse_kanji_group(
        node.children().find(|c| c.has_tag_name("g")).unwrap()
    );

    kanji.kanji = String::from_iter(iter::once(c));

    kanji
}

fn parse_kanji_group(node: Node) -> Kanji {
    let mut result = Kanji::default();

    for attrib in node.attributes() {
        match attrib.name() {
            "id"          => (),
            "variant"     => result.variant = attrib.value().parse().unwrap(),
            "radical"     => result.radical = Some(attrib.value().parse().unwrap()),
            "radicalForm" => result.radical_form = attrib.value().parse().unwrap(),
            "original"    => result.original = Some(attrib.value().to_string()),
            "phon"        => result.phon = Some(attrib.value().to_string()),
            "element"     => result.kanji = attrib.value().to_string(),
            "position"    => result.position = Some(attrib.value().parse().unwrap()),
            "part"        => result.part = Some(attrib.value().parse().unwrap()),
            "partial"     => result.partial = attrib.value().parse().unwrap(),
            "number"      => result.number = Some(attrib.value().parse().unwrap()),
            "tradForm"    => result.trad_form = attrib.value().parse().unwrap(),
            name          => panic!("Unknown attribute '{}' in <g>", name),
        }
    }

    for child in node.children() {
        match child.tag_name().name() {
            "g"    => result.parts.push(parse_kanji_group(child)),
            "path" => result.strokes.push(parse_path(child)),
            ""     => (),                                                // Ignore text nodes (whitespace)
            name   => panic!("Unknown element <{}> in <kanji>", name),
        }
    }

    result
}

fn parse_path(node: Node) -> Stroke {
    let mut result = Stroke::default();

    for attrib in node.attributes() {
        match attrib.name() {
            "id"   => (),
            "d"    => result.path = attrib.value().to_string(),
            "type" => result.typ = attrib.value().to_string(),
            name   => panic!("Unknown attribute '{}' in <path>", name),
        }
    }

    result
}
