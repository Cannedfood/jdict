#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use jdict_db::database::Config;
use jdict_db::database::Database;
use tauri::Manager;

#[tauri::command]
fn search<'a>(search_term: &str, take: Option<u32>, skip: Option<u32>, db: tauri::State<Database>) -> jdict_db::shared_api::SearchResult {
    jdict_db::shared_api::search(&db, search_term, take, skip)
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

            app.manage(Database::new(
                Config {
                    jmdict_file: resolve("../../res/JMdict_e.gz"),
                    kanjidic_file: resolve("../../res/kanjidic2.xml.gz"),
                    kanjivg_file: resolve("../../res/kanjivg.xml.gz"),
                }
            ));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
