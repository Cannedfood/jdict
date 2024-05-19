// r_ele
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Reading {
    // reb
    pub text: compact_str::CompactString,
    // re_nokanji
    pub no_kanji: bool,
    // re_restr
    pub restrict_to_kanji: Vec<String>,
    // re_inf
    pub info: Vec<ReadingInfo>,
    // re_pri
    pub prio: Vec<ReadingPrio>,
}

// re_inf
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ReadingInfo {}

// re_pri
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ReadingPrio {}
