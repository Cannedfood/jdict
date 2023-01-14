#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::RwLock;

use jdict_shared::database::Config;
use jdict_shared::database::Database;

static DB: RwLock::<Option<Database>> = RwLock::<Option<Database>>::new(None);

#[tauri::command]
fn search<'a>(search_term: &str, take: Option<u32>, skip: Option<u32>) -> jdict_shared::shared_api::SearchResult {
    for _ in 0..100 {
        if let Some(db) = DB.read().expect("Cannot read the database because it failed to load.").as_ref() {
            return jdict_shared::shared_api::search(&db, search_term, take, skip);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    panic!("Database wasn't loaded after 10 seconds.")
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let resolve = |path: &str| {
                app.path_resolver()
                .resolve_resource(path).unwrap()
                .to_str().unwrap()
                .to_string()
            };

            let cfg = Config {
                jmdict_file: resolve("../../res/JMdict_e.gz"),
                kanjidic_file: resolve("../../res/kanjidic2.xml.gz"),
                kanjivg_file: resolve("../../res/kanjivg.xml.gz"),
            };
            std::thread::spawn(|| {
                *DB.write().unwrap() = Some(Database::load(cfg));
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
