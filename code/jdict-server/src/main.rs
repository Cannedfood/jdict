#![feature(generators, generator_trait)]

use figment::{Figment, providers::{Toml, Format, Serialized}};
use jdict_shared::database::Database;
use rocket::serde::json::Json;
use rocket_async_compression::CachedCompression;
use serde::{Deserialize, Serialize};

// Api
#[allow(non_snake_case)]

#[rocket::get("/api/search?<searchTerm>&<take>&<skip>")]
pub fn search<'a>(
    searchTerm: &str,
    take: Option<u32>,
    skip: Option<u32>,
    db: &rocket::State<Database>
) -> Json<jdict_shared::shared_api::SearchResult>
{
    Json(jdict_shared::shared_api::search(&db, searchTerm, take, skip))
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

    let db = Database::load(cfg.jdict);

    let server = rocket::build()
        .configure(&cfg.rocket)
        .mount("/", rocket::routes![search])
        .mount("/", rocket::fs::FileServer::new(cfg.jdict_server.public_path, rocket::fs::Options::Index))
        .manage(db);

    if let Err(e) = opener::open_browser(format!("http://localhost:{}", cfg.rocket.port)) {
        println!("Failed to open browser: {}", e);
    }

    if cfg!(debug_assertions) {
        server
    } else {
        server.attach(CachedCompression::fairing(vec![".js", ".css", ".html", ".wasm", ".json"]))
    }
}
