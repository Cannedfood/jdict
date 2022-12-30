#![feature(generators, generator_trait)]

mod api;

use figment::{Figment, providers::{Toml, Format, Serialized}};
use jdict_db::database::Database;
use rocket_async_compression::CachedCompression;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ConfigSections {
    pub rocket: rocket::Config,
    pub jdict: jdict_db::database::Config,
}

#[rocket::launch]
fn rocket() -> _ {
    let cfg: ConfigSections = 
        Figment::from(Serialized::defaults(ConfigSections::default()))
        .merge(Toml::file("Config.toml"))
        .extract()
        .unwrap();

    let state = Database::new(cfg.jdict);

    let server = rocket::build()
        .configure(cfg.rocket)
        .mount("/", rocket::routes![api::search, api::search_kanji_in])
        .mount("/", rocket::fs::FileServer::new(state.config.public_path.clone(), rocket::fs::Options::Index))
        .manage(state);

    if cfg!(debug_assertions) {
        server
    } else {
        server.attach(CachedCompression::fairing(vec![".js", ".css", ".html", ".wasm", ".json"]))
    }
}
