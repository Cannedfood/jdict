#![feature(generators, generator_trait)]

use server_state::ServerState;

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
    let state = ServerState::new();

    rocket::build()
    .configure(rocket::Config::figment().merge(("port", 8000)))
    .manage(state)
    .mount("/", rocket::routes![api::search])
    .mount("/", rocket::fs::FileServer::new("../dist/", rocket::fs::Options::Index))
}
