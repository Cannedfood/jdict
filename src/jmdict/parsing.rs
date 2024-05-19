use compact_str::CompactString;
use roxmltree::NodeType;

use super::entry::EntrySeq;
use super::{Entry, Gloss, Reading, SourceLanguage};
use crate::jmdict::{CrossReference, GlossType, Kanji, Sense, SenseTag};

pub fn parse_jmdict(node: roxmltree::Node) -> Vec<Entry> {
    assert_eq!(node.tag_name().name(), "JMdict");

    let mut result = Vec::new();

    for child in node.children() {
        match (child.node_type(), child.tag_name().name()) {
            (NodeType::Element, "entry") => result.push(parse_entry(child)),
            (NodeType::Text, _) => {}
            (typ, name) => panic!("Unexpected child in <JMdict>: {typ:?} {name}"),
        }
    }

    result
}

pub fn parse_entry(node: roxmltree::Node) -> Entry {
    assert_eq!(node.tag_name().name(), "entry");

    let mut result = Entry {
        ent_seq: EntrySeq(0),
        kanji:   Vec::new(),
        reading: Vec::new(),
        sense:   Vec::new(),
    };

    for child in node.children() {
        match (child.node_type(), child.tag_name().name()) {
            (NodeType::Element, "ent_seq") => {
                result.ent_seq = child.text().unwrap().parse().unwrap();
            }
            (NodeType::Element, "r_ele") => {
                result.reading.push(parse_r_ele(child));
            }
            (NodeType::Element, "k_ele") => {
                result.kanji.push(parse_k_ele(child));
            }
            (NodeType::Element, "sense") => {
                result.sense.push(parse_sense(child));
            }
            (NodeType::Text, _) => {}
            (typ, name) => panic!("Unexpected child in <entry>: {typ:?} {name}"),
        }
    }

    result
}

fn parse_r_ele(node: roxmltree::Node) -> Reading {
    assert_eq!(node.tag_name().name(), "r_ele");

    let mut result = Reading {
        text: "".into(),
        no_kanji: false,
        restrict_to_kanji: Vec::new(),
        info: Vec::new(),
        prio: Vec::new(),
    };

    for child in node.children() {
        match (child.node_type(), child.tag_name().name()) {
            (NodeType::Element, "reb") => {
                result.text = child.text().unwrap().into();
            }
            (NodeType::Element, "re_nokanji") => {
                result.no_kanji = true;
            }
            (NodeType::Element, "re_restr") => {
                result.restrict_to_kanji.push(child.text().unwrap().into());
            }
            (NodeType::Element, "re_inf") => {
                // TODO
            }
            (NodeType::Element, "re_pri") => {
                // TODO
            }
            (NodeType::Text, _) => {}
            (typ, name) => panic!("Unexpected child in <r_ele>: {typ:?} {name}"),
        }
    }

    result
}

fn parse_k_ele(node: roxmltree::Node) -> Kanji {
    assert_eq!(node.tag_name().name(), "k_ele");

    let mut result = Kanji {
        text: CompactString::default(),
        info: Vec::new(),
        prio: Vec::new(),
    };

    for child in node.children() {
        match (child.node_type(), child.tag_name().name()) {
            (NodeType::Element, "keb") => {
                result.text = child.text().unwrap().into();
            }
            (NodeType::Element, "ke_pri") => {
                // TODO
            }
            (NodeType::Element, "ke_inf") => {
                // TODO
            }
            (NodeType::Text, _) => {}
            (typ, name) => panic!("Unexpected child in <k_ele>: {typ:?} {name}"),
        }
    }

    result
}

fn parse_sense(node: roxmltree::Node) -> Sense {
    assert_eq!(node.tag_name().name(), "sense");

    let mut result = Sense {
        glosses: Vec::new(),
        tags:    Vec::new(),
    };

    for child in node.children() {
        match (child.node_type(), child.tag_name().name()) {
            (NodeType::Element, "stagk") => {
                result
                    .tags
                    .push(SenseTag::OnlyForKanji(child.text().unwrap().into()));
            }
            (NodeType::Element, "stagr") => {
                result
                    .tags
                    .push(SenseTag::OnlyForReading(child.text().unwrap().into()));
            }
            (NodeType::Element, "pos") => {
                // println!("pos: {:?}", child.text().unwrap());
                // result.tags.push(SenseTag::PartOfSpeech(
                //     child.text().unwrap().parse().unwrap(),
                // ));
            }
            (NodeType::Element, "xref") => {
                result.tags.push(SenseTag::SeeAlso(CrossReference(
                    child.text().unwrap().into(),
                )));
            }
            (NodeType::Element, "ant") => {
                result.tags.push(SenseTag::Antonym(CrossReference(
                    child.text().unwrap().into(),
                )));
            }
            (NodeType::Element, "field") => {
                result
                    .tags
                    .push(SenseTag::Field(child.text().unwrap().parse().unwrap()));
            }
            (NodeType::Element, "misc") => {
                result
                    .tags
                    .push(SenseTag::Misc(child.text().unwrap().parse().unwrap()));
            }
            (NodeType::Element, "s_inf") => {
                result
                    .tags
                    .push(SenseTag::Info(child.text().unwrap().to_string()));
            }
            (NodeType::Element, "lsource") => {
                result
                    .tags
                    .push(SenseTag::SourceLanguage(parse_lsource(child)));
            }
            (NodeType::Element, "dial") => {
                result
                    .tags
                    .push(SenseTag::Dialect(child.text().unwrap().parse().unwrap()));
            }
            (NodeType::Element, "gloss") => {
                result.glosses.push(parse_gloss(child));
            }
            (NodeType::Text, _) => {}
            (typ, name) => panic!("Unexpected child in <sense>: {typ:?} {name}"),
        }
    }

    result
}

fn parse_lsource(node: roxmltree::Node) -> SourceLanguage {
    assert_eq!(node.tag_name().name(), "lsource");

    let mut result = SourceLanguage {
        lang: isolang::Language::Und,
        text: "".into(),
    };

    for attrib in node.attributes() {
        match attrib.name() {
            "lang" => {
                result.lang = match attrib.value() {
                    "tib" => isolang::Language::Bod,
                    "cze" => isolang::Language::Ces,
                    "wel" => isolang::Language::Cym,
                    "ger" => isolang::Language::Deu,
                    "gre" => isolang::Language::Ell,
                    "baq" => isolang::Language::Eus,
                    "per" => isolang::Language::Fas,
                    "fre" => isolang::Language::Fra,
                    "arm" => isolang::Language::Hye,
                    "ice" => isolang::Language::Isl,
                    "geo" => isolang::Language::Kat,
                    "mac" => isolang::Language::Mkd,
                    "mao" => isolang::Language::Mri,
                    "may" => isolang::Language::Msa,
                    "bur" => isolang::Language::Mya,
                    "dut" => isolang::Language::Nld,
                    "rum" => isolang::Language::Ron,
                    "slo" => isolang::Language::Slk,
                    "alb" => isolang::Language::Sqi,
                    "chi" => isolang::Language::Zho,
                    "mol" => isolang::Language::Bul,
                    lang => lang.parse().unwrap_or(isolang::Language::Und),
                };
            }
            "ls_type" => {
                // TODO
            }
            "ls_wasei" => {
                // TODO
            }
            _ => {
                panic!(
                    "Unexpected attribute in <lsource>: {} ({})",
                    attrib.name(),
                    attrib.namespace().unwrap_or(""),
                )
            }
        }
    }

    result.text = node.text().map(|s| s.into()).unwrap_or_default();

    result
}

fn parse_gloss(node: roxmltree::Node) -> Gloss {
    assert_eq!(node.tag_name().name(), "gloss");

    let mut result = Gloss {
        lang: isolang::Language::Eng, // Default
        text: "".into(),
        typ: GlossType::Regular,
        gender: None,
        highlight: false,
    };

    for attrib in node.attributes() {
        match attrib.name() {
            "xml:lang" => {
                result.lang = attrib.value().parse().unwrap();
            }
            "g_type" => {
                result.typ = attrib.value().parse().unwrap();
            }
            "g_gend" => {
                result.gender = Some(attrib.value().into());
            }
            _ => panic!("Unexpected attribute in <gloss>: {}", attrib.name()),
        }
    }

    result.text = node.text().unwrap().into();

    for child in node.children() {
        match (child.node_type(), child.tag_name().name()) {
            (NodeType::Element, "pri") => {
                result.highlight = true;
            }
            (NodeType::Text, _) => {}
            (typ, name) => panic!("Unexpected child in <gloss>: {typ:?} {name}"),
        }
    }

    result
}
