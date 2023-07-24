#![allow(non_camel_case_types)]

use std::str::FromStr;

use crate::util::is_default;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    News1,
    News2,
    Ichi1,
    Ichi2,
    Spec1,
    Spec2,
    Gai1,
    Gai2,
    NF(u32),
}
pub type Priorities = Vec<Priority>;

#[derive(Clone, Debug)]
pub enum FromStrErr {
    UnknownValue(String),
}

impl FromStr for Priority {
    type Err = FromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "news1" => Ok(Priority::News1),
            "news2" => Ok(Priority::News2),
            "ichi1" => Ok(Priority::Ichi1),
            "ichi2" => Ok(Priority::Ichi2),
            "spec1" => Ok(Priority::Spec1),
            "spec2" => Ok(Priority::Spec2),
            "gai1" => Ok(Priority::Gai1),
            "gai2" => Ok(Priority::Gai2),
            x => match x.strip_prefix("nf") {
                Some(n) => match n.parse() {
                    Ok(n) => Ok(Priority::NF(n)),
                    Err(_) => Err(FromStrErr::UnknownValue(x.to_string())),
                },
                None => Err(FromStrErr::UnknownValue(x.to_string())),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Neutral,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    bra,  // "Brazilian"
    hob,  // "Hokkaido-ben"
    ksb,  // "Kansai-ben"
    ktb,  // "Kantou-ben"
    kyb,  // "Kyoto-ben"
    kyu,  // "Kyuushuu-ben"
    nab,  // "Nagano-ben"
    osb,  // "Osaka-ben"
    rkb,  // "Ryuukyuu-ben"
    thb,  // "Touhoku-ben"
    tsb,  // "Tosa-ben"
    tsug, // "Tsugaru-ben"
}
pub type Dialects = Vec<Dialect>;
impl FromStr for Dialect {
    type Err = FromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bra" => Ok(Dialect::bra),
            "hob" => Ok(Dialect::hob),
            "ksb" => Ok(Dialect::ksb),
            "ktb" => Ok(Dialect::ktb),
            "kyb" => Ok(Dialect::kyb),
            "kyu" => Ok(Dialect::kyu),
            "nab" => Ok(Dialect::nab),
            "osb" => Ok(Dialect::osb),
            "rkb" => Ok(Dialect::rkb),
            "thb" => Ok(Dialect::thb),
            "tsb" => Ok(Dialect::tsb),
            "tsug" => Ok(Dialect::tsug),
            // These are the ones we actually use:
            "Brazilian" => Ok(Dialect::bra),
            "Hokkaido-ben" => Ok(Dialect::hob),
            "Kansai-ben" => Ok(Dialect::ksb),
            "Kantou-ben" => Ok(Dialect::ktb),
            "Kyoto-ben" => Ok(Dialect::kyb),
            "Kyuushuu-ben" => Ok(Dialect::kyu),
            "Nagano-ben" => Ok(Dialect::nab),
            "Osaka-ben" => Ok(Dialect::osb),
            "Ryuukyuu-ben" => Ok(Dialect::rkb),
            "Touhoku-ben" => Ok(Dialect::thb),
            "Tosa-ben" => Ok(Dialect::tsb),
            "Tsugaru-ben" => Ok(Dialect::tsug),
            _ => Err(FromStrErr::UnknownValue(s.to_string())),
        }
    }
}

// <lsource>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct LanguageOrigin {
    pub lang: String,  // attribute: xml:lang
    pub word: String,  // Content
    pub partial: bool, // attribute: ls_type="partial"
    pub wasei: bool,   // attribute: ls_wasei="y"
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum GlossType {
    Literal,
    Figurative,
    Explanatory,
    Trademark,
}

// <gloss>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Gloss {
    #[serde(default, skip_serializing_if = "is_default")]
    pub value: String, // Content
    #[serde(default, skip_serializing_if = "is_default")]
    pub lang: String, // attribute: xml:lang
    #[serde(default, skip_serializing_if = "is_default")]
    pub typ: Option<GlossType>, // attribute: g_type
    #[serde(default, skip_serializing_if = "is_default")]
    pub gender: Option<Gender>, // attribute: g_gend
    #[serde(default, skip_serializing_if = "is_default")]
    pub highlight: bool, // contains: <pri>
}

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Sentence {
    pub value: String,
    pub lang: Option<String>,
}

// <example>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Example {
    pub source: String, // attribute: xml:lang
    pub form_in_example: String,
    pub sentences: Vec<Sentence>,
}

// <k_ele>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Kanji {
    #[serde(default, skip_serializing_if = "is_default")]
    pub value: String, // <keb>
    #[serde(default, skip_serializing_if = "is_default")]
    pub infos: Vec<String>, // <ke_inf>
    #[serde(default, skip_serializing_if = "is_default")]
    pub priorities: Priorities, // <ke_pri>
}

// <r_ele>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Reading {
    pub value: String, // <reb>
    #[serde(default, skip_serializing_if = "is_default")]
    pub romaji: Option<String>, // (generated, not part of the thing)
    #[serde(default, skip_serializing_if = "is_default")]
    pub info: Vec<String>, // <re_inf>
    #[serde(default, skip_serializing_if = "is_default")]
    pub restrict: Vec<String>, // <re_restr> Which kanji this reading is restricted to
    #[serde(default, skip_serializing_if = "is_default")]
    pub priority: Priorities, // <re_pri>
    #[serde(default, skip_serializing_if = "is_default")]
    pub no_kanji: bool, // <re_nokanji>
}

// <sense>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Sense {
    #[serde(default, skip_serializing_if = "is_default")]
    pub restrict_to_kanji: Vec<String>, // <stagk>
    #[serde(default, skip_serializing_if = "is_default")]
    pub restrict_to_reading: Vec<String>, // <stagr>
    #[serde(default, skip_serializing_if = "is_default")]
    pub part_of_speech: Vec<String>, // <pos>
    #[serde(default, skip_serializing_if = "is_default")]
    pub xrefs: Vec<String>, // <xref> Cross-reference to another entry; "See also"
    #[serde(default, skip_serializing_if = "is_default")]
    pub antonyms: Vec<String>, // <ant>
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<String>, // <field> Field of application
    #[serde(default, skip_serializing_if = "is_default")]
    pub misc: Vec<String>, // <misc>
    #[serde(default, skip_serializing_if = "is_default")]
    pub info: Vec<String>, // <s_inf>
    #[serde(default, skip_serializing_if = "is_default")]
    pub origin: Vec<LanguageOrigin>, // <lsource>
    #[serde(default, skip_serializing_if = "is_default")]
    pub dialect: Dialects, // <dial>
    #[serde(default, skip_serializing_if = "is_default")]
    pub glosses: Vec<Gloss>, // <gloss>
    #[serde(default, skip_serializing_if = "is_default")]
    pub examples: Vec<Example>, // <example>
}

// <entry>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Entry {
    pub id: u32, // <ent_seq>
    #[serde(default, skip_serializing_if = "is_default")]
    pub kanji: Vec<Kanji>, // <k_ele>
    #[serde(default, skip_serializing_if = "is_default")]
    pub readings: Vec<Reading>, // <r_ele>
    #[serde(default, skip_serializing_if = "is_default")]
    pub senses: Vec<Sense>, // <sense>
}

// <JMdict>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct JMdict {
    #[serde(default, skip_serializing_if = "is_default")]
    pub entries: Vec<Entry>, // <entry>
}
