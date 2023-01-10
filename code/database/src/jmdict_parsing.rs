use std::path::Path;

use roxmltree::{Node, ParsingOptions};

use crate::{jmdict::{self, Gender}, kana::to_romaji, util::read_file};

impl jmdict::JMdict {
    pub fn parse(path: &Path) -> Self {
        let file_content = read_file(&path).unwrap();

        let document = roxmltree::Document::parse_with_options(
            &file_content,
            ParsingOptions {
                allow_dtd: true,
                ..Default::default()
            }
        ).unwrap();
        let root = document.root_element();

        assert!(root.tag_name().name() == "JMdict");

        Self {
            entries: root.children().filter(|e| e.is_element()).map(parse_entry).collect()
        }
    }
}

fn parse_entry(entry: Node) -> jmdict::Entry {
    assert!(entry.tag_name().name() == "entry");

    let mut result = jmdict::Entry::default();
    for child in entry.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "ent_seq" => result.id = child.text().unwrap().parse().unwrap(),
            "k_ele"   => result.kanji.push(parse_kanji(&child)),
            "r_ele"   => result.readings.push(parse_reading(&child)),
            "sense"   => result.senses.push(parse_sense(&child)),
            _ => panic!("Unexpected child of <entry>: {}", child.tag_name().name())
        }
    }
    return result
}

fn parse_kanji(kanji: &Node) -> jmdict::Kanji {
    let mut result: jmdict::Kanji = Default::default();

    for child in kanji.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "keb" => result.value = child.text().unwrap().to_string(),
            "ke_pri" => result.priorities.push(child.text().unwrap().parse().unwrap()),
            "ke_inf" => result.infos.push(child.text().unwrap().to_string()),
            _ => panic!("Unexpected child of <k_ele>: {}", child.tag_name().name()),
        }
    }

    result
}

fn parse_reading(reading: &Node) -> jmdict::Reading {
    let mut result = jmdict::Reading::default();
    for child in reading.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "reb" => {
                result.value = child.text().unwrap().to_string();
                result.romaji = Some(to_romaji(&result.value));
            },
            "re_inf" => result.info.push(child.text().unwrap().to_string()),
            "re_nokanji" => result.no_kanji = true,
            "re_restr" => result.restrict.push(child.text().unwrap().to_string()),
            "re_pri" => result.priority.push(child.text().unwrap().parse().unwrap()),
            _ => panic!("Unexpected child of <r_ele>: {}", child.tag_name().name()),
        }
    }
    result
}

fn parse_sense(sense: &Node) -> jmdict::Sense {
    let mut result = jmdict::Sense::default();
    for child in sense.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "stagk"   => result.restrict_to_kanji.push(child.text().unwrap().to_string()),
            "stagr"   => result.restrict_to_reading.push(child.text().unwrap().to_string()),
            "pos"     => result.part_of_speech.push(child.text().unwrap().to_string()),
            "xref"    => result.xrefs   .push(child.text().unwrap().to_string()),
            "ant"     => result.antonyms.push(child.text().unwrap().to_string()),
            "field"   => result.fields  .push(child.text().unwrap().to_string()),
            "misc"    => result.misc    .push(child.text().unwrap().to_string()),
            "s_inf"   => result.info    .push(child.text().unwrap().to_string()),
            "gloss"   => result.glosses .push(parse_gloss(&child)),
            "example" => result.examples.push(parse_example(&child)),
            "dial"    => result.dialect .push(child.text().unwrap().parse().unwrap()),
            "lsource" => { /* TODO */},
            _ => panic!("Unknown child of <sense>: {}", child.tag_name().name()),
        }
    }
    result
}

fn parse_gloss(gloss: &Node) -> jmdict::Gloss {
    jmdict::Gloss {
        value: gloss.text().unwrap().to_string(),
        lang: gloss.attribute("xml:lang").unwrap_or("eng").to_string(),
        gender: gloss.attribute("g_gend").map_or(
            Gender::None,
            |g_gend| match g_gend {
                "male" => Gender::Male,
                "female" => Gender::Female,
                "neutral" => Gender::Neutral,
                _ => panic!("Failed parsing gender: {}", g_gend),
            }
        ),
        typ:
            gloss.attribute("g_type")
            .map_or_else(
                || jmdict::GlossType::None,
                |s| match &s[..] {
                    "literal" => jmdict::GlossType::Literal,
                    "figurative" => jmdict::GlossType::Figurative,
                    _ => jmdict::GlossType::None,
                }
            ),
        highlight: gloss.children().any(|c| c.tag_name().name() == "pri"),
    }
}

fn parse_example(example: &Node) -> jmdict::Example {
    assert!(example.tag_name().name() == "example");

    Default::default() // TODO
    // jmdict::Example {

    //     // text: example.text().unwrap(),
    //     // language: example.attribute("xml:lang").unwrap_or("eng"),
    // }
}
