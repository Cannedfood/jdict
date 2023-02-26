use serde::Serialize;

use crate::database::Database;

use crate::kanjidic::Character;
use crate::kanjivg::Kanji;
use crate::jmdict::Entry;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub kanji: Vec<Character>,
    pub kanjivg: Vec<Kanji>,
    pub results: Vec<Entry>,
    pub results_total: usize,
    pub time: String,
}

pub fn search(db: &Database, search_term: &str, take: Option<u32>, skip: Option<u32>) -> SearchResult {
    let start_time = std::time::Instant::now();

    let all_results = db.search(search_term);

    let paged_results =
        all_results.iter()
        .skip(skip.unwrap_or(0) as usize)
        .take(take.unwrap_or(128) as usize)
        .cloned() // TODO: PERFORMANCE: Use references instead of cloning
        .collect::<Vec<Entry>>();

    let (kanji, kanjivg) = db.contained_kanji_chars(search_term);

    SearchResult {
        kanji,
        kanjivg,
        results: paged_results,
        results_total: all_results.len(),
        time: format!("{:?}", start_time.elapsed()),
    }
}
