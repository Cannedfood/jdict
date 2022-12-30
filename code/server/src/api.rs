#![allow(non_snake_case, unused_variables)]

use jdict_db::{kanjidic::Character, jmdict::Entry, database::Database};
use rocket::{serde::json::Json, State};

#[derive(serde::Serialize)]
pub struct SearchResult {
    pub kanji: Vec<Character>,
    pub results: Vec<Entry>,
    pub resultsTotal: usize,
    pub time: String,
}

#[rocket::get("/api/search?<searchTerm>&<take>&<skip>")]
pub fn search<'a>(searchTerm: &str, take: Option<u32>, skip: Option<u32>, db: &State<Database>) -> Json<SearchResult> {
    let startTime = std::time::Instant::now();

    let all_results = db.search(searchTerm);

    let paged_results =
        all_results.iter()
        .skip(skip.unwrap_or(0) as usize)
        .take(take.unwrap_or(128) as usize)
        .map(|entry| entry.clone())
        .collect::<Vec<Entry>>();

    Json(SearchResult {
        kanji: db.contained_kanji_chars(&searchTerm),
        results: paged_results,
        resultsTotal: all_results.len(),
        time: format!("{:?}", startTime.elapsed()),
    })
}

#[rocket::get("/api/kanji_in?<searchTerm>")]
pub fn search_kanji_in<'a>(searchTerm: &str, db: &State<Database>) -> Json<Vec<Character>> {
    Json(db.contained_kanji_chars(searchTerm))
}
