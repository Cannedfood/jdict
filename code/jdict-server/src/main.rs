#![feature(generators, generator_trait)]

use std::sync::RwLock;

use figment::{Figment, providers::{Toml, Format, Serialized}};
use jdict_shared::database::Database;
use rocket::serde::json::Json;
use rocket_async_compression::CachedCompression;
use serde::{Deserialize, Serialize};

static DB: RwLock::<Option<Database>> = RwLock::<Option<Database>>::new(None);

// Api
#[allow(non_snake_case)]

#[rocket::get("/api/search?<searchTerm>&<take>&<skip>")]
pub fn search<'a>(
    searchTerm: &str,
    take: Option<u32>,
    skip: Option<u32>
) -> Json<jdict_shared::shared_api::SearchResult>
{
    for _ in 0..100 {
        if let Some(db) = DB.read().expect("Cannot read the database because it failed to load.").as_ref() {
            return Json(jdict_shared::shared_api::search(db, searchTerm, take, skip));
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    panic!("Database wasn't loaded after 10 seconds.")
}

// Rocket server

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct JdictServerConfig {
    pub public_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ConfigSections {
    pub rocket: rocket::Config,
    pub jdict: jdict_shared::database::Config,
    pub jdict_server: JdictServerConfig,
}

#[rocket::launch]
fn rocket() -> _ {
    let cfg: ConfigSections =
        Figment::from(Serialized::defaults(ConfigSections::default()))
        .merge(Toml::file("Config.toml"))
        .extract()
        .unwrap();

    std::thread::spawn(|| {
        *DB.write().unwrap() = Some(Database::load(cfg.jdict));
    });

    let server = rocket::build()
        .configure(&cfg.rocket)
        .mount("/", rocket::routes![search])
        .mount("/", rocket::fs::FileServer::new(cfg.jdict_server.public_path, rocket::fs::Options::Index));

    if let Err(e) = opener::open_browser(format!("http://localhost:{}", cfg.rocket.port)) {
        println!("Failed to open browser: {}", e);
    }

    if cfg!(debug_assertions) {
        server
    } else {
        server.attach(CachedCompression::fairing(vec![".js", ".css", ".html", ".wasm", ".json"]))
    }
}
