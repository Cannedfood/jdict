// k_ele
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Kanji {
    // keb
    pub text: compact_str::CompactString,
    // ke_inf
    pub info: Vec<KanjiInfo>,
    // ke_pri
    #[serde(skip)]
    pub prio: Vec<KanjiPrio>,
}

// ke_inf
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, serde::Serialize, serde::Deserialize,
)]
pub enum KanjiInfo {
    /// ateji (phonetic) reading
    #[strum(serialize = "ateji")]
    Ateji,
    /// word containing irregular kana usage
    #[strum(serialize = "ik")]
    IrregularKana,
    /// word containing irregular kanji usage
    #[strum(serialize = "iK")]
    IrregularKanji,
    /// irregular okurigana usage
    #[strum(serialize = "io")]
    IrrOkurigana,
    /// word containing out-dated kanji or kanji usage
    #[strum(serialize = "oK")]
    OutdatedKanji,
    /// rarely-used kanji form
    #[strum(serialize = "rK")]
    RareKanji,
    /// search-only kanji form
    #[strum(serialize = "sK")]
    SearchOnlyKanji,
}

// ke_pri
#[derive(serde::Serialize, serde::Deserialize)]
pub enum KanjiPrio {}
