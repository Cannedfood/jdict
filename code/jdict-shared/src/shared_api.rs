use std::sync::atomic::AtomicBool;
use std::sync::OnceLock;
use std::time::Duration;

use serde::Serialize;

use crate::database::{Database, DictData};

use crate::jmdict::Entry;
use crate::kanjidic::Character;
use crate::kanjivg::Kanji;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult<'db_lifetime> {
    pub kanji: Vec<&'db_lifetime Character>,
    pub kanjivg: Vec<&'db_lifetime Kanji>,
    pub results: Vec<&'db_lifetime Entry>,
    pub results_total: usize,
    pub time: String,
}

pub static DB_LOADING: AtomicBool = AtomicBool::new(false);
pub static DB: OnceLock<Database> = OnceLock::new();

pub fn load_db(config: &crate::database::Config) {
    DB_LOADING.store(true, std::sync::atomic::Ordering::SeqCst);
    DB.set(Database::load(config))
        .unwrap_or_else(|_| panic!("Failed setting db singleton"));
    DB_LOADING.store(false, std::sync::atomic::Ordering::SeqCst);
}
pub fn parse_db(data: DictData) {
    DB_LOADING.store(true, std::sync::atomic::Ordering::SeqCst);
    DB.set(Database::from_bytes(data))
        .unwrap_or_else(|_| panic!("Failed setting db singleton"));
    DB_LOADING.store(false, std::sync::atomic::Ordering::SeqCst);
}
pub fn load_db_async(config: crate::database::Config) {
    std::thread::spawn(move || load_db(&config));
}
pub fn parse_db_async(data: DictData<'static>) {
    std::thread::spawn(move || parse_db(data));
}
pub fn get_db_sync(total_timeout: Duration, subdivisions: u32) -> &'static Database {
    let timeout_per_subdivision = total_timeout / subdivisions;

    for _ in 0..subdivisions {
        if let Some(db) = DB.get() {
            return db;
        }

        if !DB_LOADING.load(std::sync::atomic::Ordering::Relaxed) {
            panic!("Called get_db_sync() before DB calling load_db() or load_db_async(). This is most likely a bug, since the DB should at least START loading before any requests are made.");
        }
        std::thread::sleep(timeout_per_subdivision);
    }

    panic!("DB not ready after {} seconds", total_timeout.as_secs());
}

pub fn search(search_term: &str, take: Option<u32>, skip: Option<u32>) -> SearchResult<'static> {
    search_internal(
        get_db_sync(Duration::from_secs(5), 20),
        search_term,
        take,
        skip,
    )
}

pub fn search_internal<'a>(
    db: &'a Database,
    search_term: &str,
    take: Option<u32>,
    skip: Option<u32>,
) -> SearchResult<'a> {
    let start_time = std::time::Instant::now();

    let all_results = db.search(search_term);

    let results_total = all_results.len();
    let paged_results = all_results
        .into_iter()
        .skip(skip.unwrap_or(0) as usize)
        .take(take.unwrap_or(1024) as usize)
        .collect();

    let (kanji, kanjivg) = db.contained_kanji_chars(search_term);

    SearchResult {
        kanji,
        kanjivg,
        results: paged_results,
        results_total,
        time: format!("{:?}", start_time.elapsed()),
    }
}
