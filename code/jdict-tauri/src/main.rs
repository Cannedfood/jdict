#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use jdict_shared::database::Config;

#[tauri::command]
async fn search(search_term: String, take: Option<u32>, skip: Option<u32>) -> jdict_shared::shared_api::SearchResult {
	return jdict_shared::shared_api::search(search_term.as_str(), take, skip);
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

            jdict_shared::shared_api::load_db_async(Config {
                jmdict_file: resolve("../../res/JMdict_e.gz"),
                kanjidic_file: resolve("../../res/kanjidic2.xml.gz"),
                kanjivg_file: resolve("../../res/kanjivg.xml.gz"),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
