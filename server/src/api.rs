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

    let all_results = state.search(searchTerm);

    let paged_results =
        all_results.iter()
        .skip(skip.unwrap_or(0) as usize)
        .take(take.unwrap_or(128) as usize)
        .map(|entry| entry.clone())
        .collect::<Vec<Entry>>();

    Json(SearchResult {
        kanji: state.contained_kanji_chars(&searchTerm),
        results: paged_results,
        resultsTotal: all_results.len(),
        time: format!("{:?}", startTime.elapsed()),
    })
}

#[rocket::get("/api/kanji_in?<searchTerm>")]
pub fn search_kanji_in<'a>(searchTerm: &str, state: &State<ServerState>) -> Json<Vec<Character>> {
    Json(state.contained_kanji_chars(searchTerm))
}
