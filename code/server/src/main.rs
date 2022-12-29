#![feature(generators, generator_trait)]

use figment::{Figment, providers::{Toml, Format, Serialized}};
use rocket_async_compression::CachedCompression;
use serde::{Deserialize, Serialize};
use server_state::ServerState;

mod util;
mod server_state;
mod api;
mod jmdict;
mod jmdict_parsing;
mod jmdict_result_rating;
mod kanjidic;
mod kanjidic_parsing;
mod fulltext_index;
mod kana;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ConfigSections {
    pub rocket: rocket::Config,
    pub jdict: server_state::Config,
}

#[rocket::launch]
fn rocket() -> _ {
    let cfg: ConfigSections = 
        Figment::from(Serialized::defaults(ConfigSections::default()))
        .merge(Toml::file("Config.toml"))
        .extract()
        .unwrap();

    let state = ServerState::new(cfg.jdict);

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
