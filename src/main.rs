use std::{
    sync::{Arc, Mutex},
    vec,
};

use actix_web::{web, App, HttpServer};
use rust_actix::{
    config::read_config,
    infra::endpoints,
    modules::music::{self, domain::Playlist},
    state::State,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = read_config();
    let stack: Vec<Playlist> = vec![];
    let playlists = Arc::new(Mutex::new(stack));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                playlist: playlists.clone(),
            }))
            .service(
                web::scope("/api")
                    .configure(endpoints::config)
                    .configure(music::infra::endpoints::config),
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
