use super::dtos::{CreatePlaylistParam, GetPlaylistParam};
use crate::{modules::music::domain::Playlist, state::State};
use actix_web::{get, post, web, Responder};

#[get("/playlist")]
async fn playlist(data: web::Data<State>) -> impl Responder {
    let playlist = data.playlist.lock().expect("Bad state");
    web::Json(playlist.clone())
}
#[get("/playlist/{id}")]
async fn get_playlist(
    param: web::Path<GetPlaylistParam>,
    data: web::Data<State>,
) -> impl Responder {
    let playlists = data.playlist.lock().expect("Bad state");
    let founded_playlist = playlists[param.id].clone();
    web::Json(founded_playlist)
}
#[post("/playlist")]
async fn create_playlist(
    param: web::Json<CreatePlaylistParam>,
    data: web::Data<State>,
) -> impl Responder {
    let mut playlists = data.playlist.lock().expect("Bad state");
    let playlist_data = Playlist {
        name: param.name.clone(),
        songs: vec![],
    };
    playlists.push(playlist_data.clone());

    web::Json(playlist_data)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playlist);
    cfg.service(get_playlist);
    cfg.service(create_playlist);
}
