use std::sync::atomic::AtomicBool;
use std::sync::{RwLock, Arc};
use std::time::Duration;

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

pub static DB_LOADING: AtomicBool = AtomicBool::new(false);
pub static DB: RwLock<Option<Arc<Database>>> = RwLock::<Option<Arc<Database>>>::new(None);

pub fn load_db(config: crate::database::Config) {
	DB_LOADING.store(true, std::sync::atomic::Ordering::SeqCst);
	*DB.write().unwrap() = Some(Arc::new(Database::load(config)));
	DB_LOADING.store(false, std::sync::atomic::Ordering::SeqCst);
}
pub fn load_db_async(config: crate::database::Config) {
	std::thread::spawn(move || load_db(config));
}
pub fn get_db_sync(total_timeout: Duration, subdivisions: u32) -> Arc<Database> {
	let timeout_per_subdivision = total_timeout / subdivisions;

	for _ in 0..subdivisions {
		if let Some(db) = DB.try_read().unwrap().as_ref() {
			return db.clone();
		}

		if !DB_LOADING.load(std::sync::atomic::Ordering::Relaxed) {
			panic!("Called get_db_sync() before DB calling load_db() or load_db_async(). This is most likely a bug, since the DB should at least START loading before any requests are made.");
		}
		std::thread::sleep(timeout_per_subdivision);
	}

	panic!("DB not ready after {} seconds", total_timeout.as_secs());
}

pub fn search(search_term: &str, take: Option<u32>, skip: Option<u32>) -> SearchResult {
	search_internal(&get_db_sync(Duration::from_secs(5), 20), search_term, take, skip)
}

pub fn search_internal(db: &Database, search_term: &str, take: Option<u32>, skip: Option<u32>) -> SearchResult {
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
