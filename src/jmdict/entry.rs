use std::str::FromStr;

use super::{Kanji, Reading, Sense};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Entry {
    pub ent_seq: EntrySeq,
    pub kanji:   Vec<Kanji>,
    pub reading: Vec<Reading>,
    pub sense:   Vec<Sense>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EntrySeq(pub u32);
impl EntrySeq {
    pub const INVALID: EntrySeq = EntrySeq(u32::MAX);
}
impl Default for EntrySeq {
    fn default() -> Self { EntrySeq::INVALID }
}
impl FromStr for EntrySeq {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(EntrySeq(s.parse()?)) }
}
