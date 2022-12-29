#![feature(generators, generator_trait)]

use rocket_async_compression::CachedCompression;
use server_state::{ServerState, Config};

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

#[rocket::launch]
fn rocket() -> _ {
    let state = ServerState::new(Config::figment().into());

    let server = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8000)))
        .mount("/", rocket::routes![api::search, api::search_kanji_in])
        .mount("/", rocket::fs::FileServer::new(state.config.public_path.clone(), rocket::fs::Options::Index))
        .manage(state);

    if cfg!(debug_assertions) {
        server
    } else {
        server.attach(CachedCompression::fairing(vec![".js", ".css", ".html", ".wasm", ".json"]))
    }
}
