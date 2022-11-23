use libxml::{parser::Parser, tree::node::Node};

use crate::kanjidic::{Kanjidic, Header, Character, ReadingMeaningGroup, Codepoint, Radical, Misc, DicRef, QueryCode, Reading, Meaning, ReadingType, OnType, DicRefType, QueryCodeType, SkipMisclass, Variant, VariantType};

impl Kanjidic {
    pub fn parse(path: &str) -> Self {
        let data = std::fs::read_to_string(path).unwrap();

        let parser = Parser::default();
        let document = parser.parse_string(&data).unwrap();
        let root = document.get_root_element().unwrap();
        assert!(root.get_name() == "kanjidic2");

        let mut result = Self::default();
        for child in root.get_child_elements() {
            match child.get_name().as_str() {
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

    for child in element.get_child_elements() {
        match child.get_name().as_str() {
            "file_version"     => header.file_version     = child.get_content(),
            "database_version" => header.database_version = child.get_content(),
            "date_of_creation" => header.date_of_creation = child.get_content(),
            _ => panic!("Unknown header element: {}", child.get_name()),
        }
    }

    header
}

fn parse_character(element: &Node) -> Character {
    let mut character = Character::default();

    for child in element.get_child_elements() {
        match child.get_name().as_str() {
            "literal"    => character.literal    = child.get_content(),
            "codepoint"  => character.codepoint  = parse_codepoint(&child),
            "radical"    => character.radical    = parse_radical(&child),
            "misc"       => character.misc       = parse_misc(&child),
            "dic_number" => character.dic_number = parse_dic_number(&child),
            "query_code" => character.query_code = parse_query_codes(&child),
            "reading_meaning" => character.reading_meaning_groups.push(parse_reading_meaning(&child)),
            _ => panic!("Unknown element <{}> in <character>", child.get_name()),
        }
    }

    character
}

fn parse_reading_meaning(element: &Node) -> ReadingMeaningGroup {
    let mut rmgroup = ReadingMeaningGroup::default();

    for child in element.get_child_elements() {
        match child.get_name().as_str() {
            "rmgroup" => {
                for rmgroup_child in child.get_child_elements() {
                    match rmgroup_child.get_name().as_str() {
                        "reading" => rmgroup.readings.push(parse_reading(&rmgroup_child)),
                        "meaning" => rmgroup.meanings.push(parse_meaning(&rmgroup_child)),
                        _ => panic!("Unknown element <{}> in <rmgroup>", rmgroup_child.get_name()),
                    }
                }
            },
            "nanori" => rmgroup.nanori.push(child.get_content()),
            _ => panic!("Unknown element <{}> in <reading_meaning>", child.get_name()),
        }
    }

    rmgroup
}

fn parse_reading(element: &Node) -> Reading {
    Reading {
        value: element.get_content(),
        typ: element.get_attribute("r_type").map_or(
            ReadingType::ja_kun,
            |r_type| match r_type.as_str() {
                "pinyin" => ReadingType::pinyin,
                "korean_r" => ReadingType::korean_r,
                "korean_h" => ReadingType::korean_h,
                "vietnam" => ReadingType::vietnam,
                "ja_on" => ReadingType::ja_on,
                "ja_kun" => ReadingType::ja_kun,
                x => panic!("Unknown reading type: {}", x),
            }
        ),
        on_type: element.get_attribute("on_type").map_or(
            OnType::none,
            |on_type| match on_type.as_str() {
                "kan" => OnType::kan,
                "go"  => OnType::go,
                "tou" => OnType::tou,
                "kan'you" => OnType::kanyou,
                x => panic!("Unknown on_type: {}", x),
            }
        ),
        approved_for_joyou_kanji: element.get_attribute("r_status").map_or(false, |r_status| r_status == "jy"),
    }
}
fn parse_meaning(element: &Node) -> Meaning {
    let m_lang = element.get_attribute("m_lang");
    Meaning {
        value: element.get_content(),
        lang: m_lang.unwrap_or_else(|| "en".to_string()),
    }
}
fn parse_codepoint(element: &Node) -> Codepoint {
    let mut result = Codepoint::default();

    for child in element.get_child_elements() {
        match child.get_name().as_str() {
            "cp_value" => match child.get_attribute("cp_type").expect("cp_value with missing cp_type attribute").as_str() {
                "ucs"    => result.ucs    = Some(child.get_content()),
                "jis208" => result.jis208 = Some(child.get_content()),
                "jis212" => result.jis212 = Some(child.get_content()),
                "jis213" => result.jis213 = Some(child.get_content()),
                _ => panic!("Unknown cp_type: {}", child.get_attribute("cp_type").unwrap()),
            }
            _ => panic!("Unknown element <{}> in <codepoint>", child.get_name()),
        }
    }

    result
}
fn parse_radical(element: &Node) -> Radical {
    let mut result = Radical::default();

    for child in element.get_child_elements() {
        match child.get_name().as_str() {
            "rad_value" => {
                let rad_type = child.get_attribute("rad_type").expect("rad_value with missing rad_type attribute");
                match rad_type.as_str() {
                    "classical" => result.classical = child.get_content().parse().expect("rad_value content cannot be parsed into a u16"),
                    "nelson_c"  => result.nelson_c  = child.get_content().parse().expect("rad_value content cannot be parsed into a u16"),
                    _ => panic!("Unknown rad_type: {}", rad_type),
                }
            },
            _ => panic!("Unknown element <{}> in <radical>", child.get_name()),
        }
    }

    result
}
fn parse_misc(element: &Node) -> Misc {
    let mut result = Misc::default();

    for child in element.get_child_elements() {
        match child.get_name().as_str() {
            "grade"        => result.grade        = child.get_content().parse().expect("grade content cannot be parsed into a u8"),
            "stroke_count" => result.stroke_count.push(child.get_content().parse().expect("stroke_count content cannot be parsed into a u8")),
            "variant"      => result.variant.push(parse_variant(&child)),
            "freq"         => result.freq         = child.get_content().parse().expect("freq content cannot be parsed into a u16"),
            "rad_name"     => result.rad_name.push(child.get_content()),
            "jlpt"         => result.jlpt         = child.get_content().parse().expect("jlpt content cannot be parsed into a u8"),
            _ => panic!("Unknown element <{}> in <misc>", child.get_name()),
        }
    }

    result
}
fn parse_dic_number(element: &Node) -> Vec<DicRef> {
    let mut result = Vec::new();

    for child in element.get_child_elements() {
        match child.get_name().as_str() {
            "dic_ref" => {
                result.push(DicRef {
                    index_number: child.get_content().parse().expect("dic_ref content cannot be parsed into a u16"),
                    moro_volume: child.get_attribute("m_vol").map(|v| v.parse().expect("m_vol content cannot be parsed into a u8")).unwrap_or(0),
                    moro_page: child.get_attribute("m_page").map(|v| v.parse().expect("m_page content cannot be parsed into a u16")).unwrap_or(0),
                    typ: match child.get_attribute("dr_type").expect("dic_ref with missing dr_type attribute").as_str() {
                        "nelson_c" => DicRefType::nelson_c,
                        "nelson_n" => DicRefType::nelson_n,
                        "halpern_njecd" => DicRefType::halpern_njecd,
                        "halpern_kkd" => DicRefType::halpern_kkd,
                        "halpern_kkld" => DicRefType::halpern_kkld,
                        "halpern_kkld_2ed" => DicRefType::halpern_kkld_2ed,
                        "heisig" => DicRefType::heisig,
                        "heisig6" => DicRefType::heisig6,
                        "gakken" => DicRefType::gakken,
                        "oneill_names" => DicRefType::oneill_names,
                        "oneill_kk" => DicRefType::oneill_kk,
                        "moro" => DicRefType::moro,
                        "henshall" => DicRefType::henshall,
                        "sh_kk" => DicRefType::sh_kk,
                        "sh_kk2" => DicRefType::sh_kk2,
                        "sakade" => DicRefType::sakade,
                        "jf_cards" => DicRefType::jf_cards,
                        "henshall3" => DicRefType::henshall3,
                        "tutt_cards" => DicRefType::tutt_cards,
                        "crowley" => DicRefType::crowley,
                        "kanji_in_context" => DicRefType::kanji_in_context,
                        "busy_people" => DicRefType::busy_people,
                        "kodansha_compact" => DicRefType::kodansha_compact,
                        "maniette" => DicRefType::maniette,
                        _ => panic!("Unknown dr_type: {}", child.get_attribute("dr_type").unwrap()),
                    }
                });
            },
            _ => panic!("Unknown element <{}> in <dic_number>", child.get_name()),
        }
    }

    result
}
fn parse_query_codes(element: &Node) -> Vec<QueryCode> {
    element.get_child_elements().iter().map(|child| {
        match child.get_name().as_str() {
            "q_code" => QueryCode {
                value: child.get_content(),
                typ: child.get_attribute("qc_type").map_or(
                    QueryCodeType::skip,
                    |qc_type| match qc_type.as_str() {
                        "skip" => QueryCodeType::skip,
                        "sh_desc" => QueryCodeType::sh_desc,
                        "four_corner" => QueryCodeType::four_corner,
                        "deroo" => QueryCodeType::deroo,
                        "misclass" => QueryCodeType::misclass,
                        _ => panic!("Unknown qc_type: {}", qc_type),
                    }
                ),
                skip_misclass: child.get_attribute("skip_misclass").map_or(
                    SkipMisclass::none,
                    |skip_misclass| match skip_misclass.as_str() {
                        "posn" => SkipMisclass::posn,
                        "stroke_count" => SkipMisclass::stroke_count,
                        "stroke_and_posn" => SkipMisclass::stroke_and_posn,
                        "stroke_diff" => SkipMisclass::stroke_diff,
                        _ => panic!("Unknown skip_misclass: {}", skip_misclass),
                    }
                ),
            },
            _ => panic!("Unknown element <{}> in <query_code>", child.get_name()),
        }
    })
    .collect()
}
fn parse_variant(element: &Node) -> Variant {
    Variant {
        value: element.get_content(),
        typ: match element.get_attribute("var_type").expect("variant with missing var_type attribute").as_str() {
            "jis208"   => VariantType::jis208,
            "jis212"   => VariantType::jis212,
            "jis213"   => VariantType::jis213,
            "deroo"    => VariantType::deroo,
            "njecd"    => VariantType::njecd,
            "s_h"      => VariantType::s_h,
            "nelson_c" => VariantType::nelson_c,
            "oneill"   => VariantType::oneill,
            "ucs"      => VariantType::ucs,
            _ => panic!("Unknown var_type: {}", element.get_attribute("var_type").unwrap()),
        },
    }
}
