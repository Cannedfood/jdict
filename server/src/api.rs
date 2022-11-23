#![allow(non_snake_case, unused_variables)]

use rocket::{serde::json::Json, State};

use crate::{jmdict::Entry, server_state::ServerState};

#[derive(serde::Serialize)]
pub struct SearchResult {
    pub kanji: Vec<()>,
    pub results: Vec<Entry>,
    pub resultsTotal: usize,
    time: String,
}

#[rocket::get("/api/search?<searchTerm>&<take>&<skip>")]
pub fn search<'a>(searchTerm: &str, take: Option<u32>, skip: Option<u32>, state: &State<ServerState>) -> Json<SearchResult> {
    let startTime = std::time::Instant::now();

    let all_results = state.search(searchTerm);

    Json(SearchResult {
        kanji: vec![],
        results:
            all_results.iter()
            .map(|entry| entry.clone())
            .skip(skip.unwrap_or(0) as usize)
            .take(take.unwrap_or(128) as usize)
            .collect(),
        resultsTotal: all_results.len(),
        time: format!("{:?}", startTime.elapsed()),
    })
}
