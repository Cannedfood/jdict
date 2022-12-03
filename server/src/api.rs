#![allow(non_snake_case, unused_variables)]

use rocket::{serde::json::Json, State};

use crate::{jmdict::Entry, server_state::ServerState, kanjidic::Character};

#[derive(serde::Serialize)]
pub struct SearchResult {
    pub kanji: Vec<Character>,
    pub results: Vec<Entry>,
    pub resultsTotal: usize,
    time: String,
}

#[rocket::get("/api/search?<searchTerm>&<take>&<skip>")]
pub fn search<'a>(searchTerm: &str, take: Option<u32>, skip: Option<u32>, state: &State<ServerState>) -> Json<SearchResult> {
    let startTime = std::time::Instant::now();

    let word_results = state.search(searchTerm);
    let kanji_results = state.search_kanji(searchTerm);

    Json(SearchResult {
        kanji: kanji_results,
        results:
            word_results.iter()
            .skip(skip.unwrap_or(0) as usize)
            .take(take.unwrap_or(128) as usize)
            .map(|entry| entry.clone())
            .collect(),
        resultsTotal: word_results.len(),
        time: format!("{:?}", startTime.elapsed()),
    })
}
