#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use jdict_db::database::Config;
use jdict_db::database::Database;

#[tauri::command]
fn search<'a>(search_term: &str, take: Option<u32>, skip: Option<u32>, db: tauri::State<Database>) -> jdict_db::shared_api::SearchResult {
    jdict_db::shared_api::search(&db, search_term, take, skip)
}

fn main() {
    let database = Database::new(
        Config {
            jmdict_file: "res/JMdict_e.gz".to_string(),
            kanjidic_file: "res/kanjidic2.xml.gz".to_string(),
            kanjivg_file: "res/kanjivg.xml.gz".to_string(),
        }
    );

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
