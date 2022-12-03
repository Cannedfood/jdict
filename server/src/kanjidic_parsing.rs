use std::{fs::File, io::Read};

use roxmltree::{Node, ParsingOptions};

use crate::kanjidic::{Kanjidic, Header, Character, ReadingMeaningGroup, Codepoint, Radical, Misc, DicRef, QueryCode, Reading, Meaning, ReadingType, OnType, QueryCodeType, SkipMisclass, Variant, VariantType};

impl Kanjidic {
    pub fn parse(path: &str) -> Self {
        let mut file_content: String = String::new();
        File::open(path).unwrap().read_to_string(&mut file_content).unwrap();

        let document = roxmltree::Document::parse_with_options(
            &file_content,
            ParsingOptions {
                allow_dtd: true,
                ..Default::default()
            }
        ).unwrap();

        let root = document.root_element();
        if root.tag_name().name() != "kanjidic2" {
            panic!("Root element is not <kanjidic2>");
        }

        let mut result = Self::default();
        for child in root.children() {
            match child.tag_name().name() {
                "header"    => result.header = parse_header(&child),
                "character" => result.characters.push(parse_character(&child)),
                _ => (),
            }
        }

        result
    }
}

fn parse_header(element: &Node) -> Header {
    let mut header = Header::default();

    for child in element.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "file_version"     => header.file_version     = child.text().unwrap().to_string(),
            "database_version" => header.database_version = child.text().unwrap().to_string(),
            "date_of_creation" => header.date_of_creation = child.text().unwrap().to_string(),
            _ => panic!("Unknown header element: {:?}", child),
        }
    }

    header
}

fn parse_character(element: &Node) -> Character {
    let mut character = Character::default();

    for child in element.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "literal"    => character.literal    = child.text().unwrap().to_string(),
            "codepoint"  => character.codepoint  = parse_codepoint(&child),
            "radical"    => character.radical    = parse_radical(&child),
            "misc"       => character.misc       = parse_misc(&child),
            "dic_number" => character.dic_number = parse_dic_number(&child),
            "query_code" => character.query_code = parse_query_codes(&child),
            "reading_meaning" => character.reading_meaning_groups.push(parse_reading_meaning(&child)),
            _ => panic!("Unknown element <{}> in <character>", child.tag_name().name()),
        }
    }

    character
}

fn parse_reading_meaning(element: &Node) -> ReadingMeaningGroup {
    let mut rmgroup = ReadingMeaningGroup::default();

    for child in element.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "rmgroup" => {
                for rmgroup_child in child.children().filter(|n| n.is_element()) {
                    match rmgroup_child.tag_name().name() {
                        "reading" => rmgroup.readings.push(parse_reading(&rmgroup_child)),
                        "meaning" => rmgroup.meanings.push(parse_meaning(&rmgroup_child)),
                        _ => panic!("Unknown element <{}> in <rmgroup>", rmgroup_child.tag_name().name()),
                    }
                }
            },
            "nanori" => rmgroup.nanori.push(child.text().unwrap().to_string()),
            _ => panic!("Unknown element <{}> in <reading_meaning>", child.tag_name().name()),
        }
    }

    rmgroup
}

fn parse_reading(element: &Node) -> Reading {
    Reading {
        value: element.text().unwrap().to_string(),
        typ: element.attribute("r_type").map_or(
            ReadingType::ja_kun,
            |r_type| match r_type {
                "pinyin"   => ReadingType::pinyin,
                "korean_r" => ReadingType::korean_r,
                "korean_h" => ReadingType::korean_h,
                "vietnam"  => ReadingType::vietnam,
                "ja_on"    => ReadingType::ja_on,
                "ja_kun"   => ReadingType::ja_kun,
                x => panic!("Unknown reading type: {}", x),
            }
        ),
        on_type: element.attribute("on_type").map_or(
            OnType::none,
            |on_type| match on_type {
                "kan"     => OnType::kan,
                "go"      => OnType::go,
                "tou"     => OnType::tou,
                "kan'you" => OnType::kanyou,
                x => panic!("Unknown on_type: {}", x),
            }
        ),
        approved_for_joyou_kanji: element.attribute("r_status").map_or(false, |r_status| r_status == "jy"),
    }
}
fn parse_meaning(element: &Node) -> Meaning {
    Meaning {
        value: element.text().unwrap().to_string(),
        lang: element.attribute("m_lang").unwrap_or_else(|| "en").to_string(),
    }
}
fn parse_codepoint(element: &Node) -> Codepoint {
    let mut result = Codepoint::default();

    for child in element.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "cp_value" => match child.attribute("cp_type").expect("cp_value with missing cp_type attribute") {
                "ucs"    => result.ucs    = Some(child.text().unwrap().to_string()),
                "jis208" => result.jis208 = Some(child.text().unwrap().to_string()),
                "jis212" => result.jis212 = Some(child.text().unwrap().to_string()),
                "jis213" => result.jis213 = Some(child.text().unwrap().to_string()),
                _ => panic!("Unknown cp_type: {}", child.attribute("cp_type").unwrap()),
            }
            _ => panic!("Unknown element <{}> in <codepoint>", child.tag_name().name()),
        }
    }

    result
}
fn parse_radical(element: &Node) -> Radical {
    let mut result = Radical::default();

    for child in element.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "rad_value" => {
                match child.attribute("rad_type").expect("rad_value with missing rad_type attribute") {
                    "classical" => result.classical = child.text().unwrap().to_string().parse().expect("rad_value content cannot be parsed into a u16"),
                    "nelson_c"  => result.nelson_c  = child.text().unwrap().to_string().parse().expect("rad_value content cannot be parsed into a u16"),
                    rad_type => panic!("Unknown rad_type: {}", rad_type),
                }
            },
            _ => panic!("Unknown element <{}> in <radical>", child.tag_name().name()),
        }
    }

    result
}
fn parse_misc(element: &Node) -> Misc {
    let mut result = Misc::default();

    for child in element.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "grade"        => result.grade = child.text().unwrap().to_string().parse().expect("grade content cannot be parsed into a u8"),
            "stroke_count" => result.stroke_count.push(child.text().unwrap().to_string().parse().expect("stroke_count content cannot be parsed into a u8")),
            "variant"      => result.variant.push(parse_variant(&child)),
            "freq"         => result.freq  = child.text().unwrap().to_string().parse().expect("freq content cannot be parsed into a u16"),
            "rad_name"     => result.rad_name.push(child.text().unwrap().to_string()),
            "jlpt"         => result.jlpt  = child.text().unwrap().to_string().parse().expect("jlpt content cannot be parsed into a u8"),
            _ => panic!("Unknown element <{}> in <misc>", child.tag_name().name()),
        }
    }

    result
}
fn parse_dic_number(element: &Node) -> Vec<DicRef> {
    let mut result = Vec::new();

    for child in element.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "dic_ref" => {
                result.push(DicRef {
                    index_number: child.text().unwrap().to_string().parse().expect("dic_ref content cannot be parsed into a u16"),
                    moro_volume: child.attribute("m_vol").map(|v| v.parse().expect("m_vol content cannot be parsed into a u8")).unwrap_or(0),
                    moro_page: child.attribute("m_page").map(|v| v.parse().expect("m_page content cannot be parsed into a u16")).unwrap_or(0),
                    typ: child.attribute("dr_type").expect("dic_ref with missing dr_type attribute").parse().unwrap()
                });
            },
            _ => panic!("Unknown element <{}> in <dic_number>", child.tag_name().name()),
        }
    }

    result
}
fn parse_query_codes(element: &Node) -> Vec<QueryCode> {
    element.children().filter(|n| n.is_element()).map(|child| {
        match child.tag_name().name() {
            "q_code" => QueryCode {
                value: child.text().unwrap().to_string(),
                typ: child.attribute("qc_type").map_or(
                    QueryCodeType::skip,
                    |qc_type| match qc_type {
                        "skip" => QueryCodeType::skip,
                        "sh_desc" => QueryCodeType::sh_desc,
                        "four_corner" => QueryCodeType::four_corner,
                        "deroo" => QueryCodeType::deroo,
                        "misclass" => QueryCodeType::misclass,
                        _ => panic!("Unknown qc_type: {}", qc_type),
                    }
                ),
                skip_misclass: child.attribute("skip_misclass").map_or(
                    SkipMisclass::none,
                    |skip_misclass| match skip_misclass {
                        "posn" => SkipMisclass::posn,
                        "stroke_count" => SkipMisclass::stroke_count,
                        "stroke_and_posn" => SkipMisclass::stroke_and_posn,
                        "stroke_diff" => SkipMisclass::stroke_diff,
                        _ => panic!("Unknown skip_misclass: {}", skip_misclass),
                    }
                ),
            },
            _ => panic!("Unknown element <{}> in <query_code>", child.tag_name().name()),
        }
    })
    .collect()
}
fn parse_variant(element: &Node) -> Variant {
    Variant {
        value: element.text().unwrap().to_string(),
        typ: match element.attribute("var_type").expect("variant with missing var_type attribute") {
            "jis208"   => VariantType::jis208,
            "jis212"   => VariantType::jis212,
            "jis213"   => VariantType::jis213,
            "deroo"    => VariantType::deroo,
            "njecd"    => VariantType::njecd,
            "s_h"      => VariantType::s_h,
            "nelson_c" => VariantType::nelson_c,
            "oneill"   => VariantType::oneill,
            "ucs"      => VariantType::ucs,
            _ => panic!("Unknown var_type: {}", element.attribute("var_type").unwrap()),
        },
    }
}
