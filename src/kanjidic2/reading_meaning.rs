use compact_str::CompactString;

// rmgroup, nanori
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReadingMeaning {
    pub nanori: Vec<CompactString>,
    pub reading_meaning_groups: Vec<ReadingMeaningGroup>,
}

// rmgroup
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReadingMeaningGroup {
    pub readings: Vec<Reading>,
    pub meanings: Vec<Meaning>,
}

// reading
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Reading {
    pub jouyou: bool,
    pub typ:    ReadingType,
    pub value:  CompactString,
}

// reading r_type,on_type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum ReadingType {
    Pinyin,
    KoreanRomanized,
    Hangul,
    Onyomi(Option<OnType>),
    Kunyomi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum OnType {
    #[strum(serialize = "kan")]     Kan,
    #[strum(serialize = "go")]      Go,
    #[strum(serialize = "tou")]     Tou,
    #[strum(serialize = "kan'you")] KanYou,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Meaning {
    pub lang: isolang::Language,
    pub text: CompactString,
}
