use crate::{jmdict::{self, Gender}, kana::to_romaji};

use libxml::parser::Parser;

impl jmdict::JMdict {
    pub fn parse(path: &str) -> Self {
        let data = std::fs::read_to_string(path).unwrap();

        let parser = Parser::default();
        let document = parser.parse_string(&data).unwrap();
        let root = document.get_root_element().unwrap();

        assert!(root.get_name() == "JMdict");

        Self {
            entries: root.get_child_elements().iter().map(parse_entry).collect()
        }
    }
}

fn parse_entry(entry: &libxml::tree::Node) -> jmdict::Entry {
    assert!(entry.get_name() == "entry");

    let mut result = jmdict::Entry::default();
    for child in entry.get_child_elements() {
        match child.get_name().as_str() {
            "ent_seq" => result.id = child.get_content().parse().unwrap(),
            "k_ele"   => result.kanji.push(parse_kanji(&child)),
            "r_ele"   => result.readings.push(parse_reading(&child)),
            "sense"   => result.senses.push(parse_sense(&child)),
            _ => panic!("Unexpected child of <entry>: {}", child.get_name())
        }
    }
    return result
}

fn parse_kanji(kanji: &libxml::tree::Node) -> jmdict::Kanji {
    let mut result: jmdict::Kanji = Default::default();

    for child in kanji.get_child_elements().iter() {
        match child.get_name().as_str() {
            "keb" => result.value = child.get_content(),
            "ke_pri" => result.priorities.push(child.get_content().parse().unwrap()),
            "ke_inf" => result.infos.push(child.get_content()),
            _ => panic!("Unexpected child of <k_ele>: {}", child.get_name()),
        }
    }

    result
}

fn parse_reading(reading: &libxml::tree::Node) -> jmdict::Reading {
    let mut result = jmdict::Reading::default();
    for child in reading.get_child_elements().iter() {
        match child.get_name().as_str() {
            "reb" => {
                let value = child.get_content();
                let romaji = to_romaji(&value);
                result.value = value;
                result.romaji = Some(romaji);
            },
            "re_inf" => result.info.push(child.get_content()),
            "re_nokanji" => result.no_kanji = true,
            "re_restr" => result.restrict.push(child.get_content()),
            "re_pri" => result.priority.push(child.get_content().parse().unwrap()),
            _ => panic!("Unexpected child of <r_ele>: {}", child.get_name()),
        }
    }
    result
}

fn parse_sense(sense: &libxml::tree::Node) -> jmdict::Sense {
    let mut result = jmdict::Sense::default();
    for child in sense.get_child_elements() {
        match child.get_name().as_str() {
            "stagk"   => result.restrict_to_kanji.push(child.get_content()),
            "stagr"   => result.restrict_to_reading.push(child.get_content()),
            "pos"     => result.part_of_speech.push(child.get_content()),
            "xref"    => result.xrefs   .push(child.get_content()),
            "ant"     => result.antonyms.push(child.get_content()),
            "field"   => result.fields  .push(child.get_content()),
            "misc"    => result.misc    .push(child.get_content()),
            "s_inf"   => result.info    .push(child.get_content()),
            "gloss"   => result.glosses .push(parse_gloss(&child)),
            "example" => result.examples.push(parse_example(&child)),
            "dial"    => result.dialect .push(child.get_content().parse().unwrap()),
            "lsource" => { /* TODO */},
            _ => panic!("Unknown child of <sense>: {}", child.get_name()),
        }
    }
    result
}

fn parse_gloss(gloss: &libxml::tree::Node) -> jmdict::Gloss {
    jmdict::Gloss {
        value: gloss.get_content(),
        lang: gloss.get_attribute("xml:lang").unwrap_or_else(|| "eng".to_string()),
        gender: gloss.get_attribute("g_gend").map_or(Gender::None, |g_gend| match g_gend.as_str() {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "neutral" => Gender::Neutral,
            _ => panic!("Failed parsing gender: {}", g_gend),
        }),
        typ:
            gloss.get_attribute("g_type")
            .map_or_else(
                || jmdict::GlossType::None,
                |s| match &s[..] {
                "literal" => jmdict::GlossType::Literal,
                "figurative" => jmdict::GlossType::Figurative,
                _ => jmdict::GlossType::None,
            }),
        highlight: gloss.get_child_nodes().iter().any(|c| c.get_name() == "pri"),
    }
}

fn parse_example(example: &libxml::tree::Node) -> jmdict::Example {
    assert!(example.get_name() == "example");

    Default::default() // TODO
    // jmdict::Example {

    //     // text: example.get_content(),
    //     // language: example.get_attribute("xml:lang").unwrap_or("eng"),
    // }
}
