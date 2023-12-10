use super::dtos::{CreatePlaylistParam, GetPlaylistParam};
use crate::modules::music::domain::Playlist;
use actix_web::{get, post, web, Responder};

#[get("/playlist")]
async fn playlist() -> impl Responder {
    let mut playlists: Vec<Playlist> = vec![];
    let first_playlist = Playlist {
        name: "firstPlaylist".to_string(),
        songs: vec![],
    };
    playlists.push(first_playlist);
    web::Json(playlists)
}
#[get("/playlist/{id}")]
async fn get_playlist(param: web::Path<GetPlaylistParam>) -> impl Responder {
    let playlists: Vec<Playlist> = vec![Playlist {
        name: "Founded Playlist".to_string(),
        songs: vec![],
    }];
    let founded_playlist = playlists[param.id].clone();
    web::Json(founded_playlist)
}
#[post("/playlist")]
async fn create_playlist(param: web::Json<CreatePlaylistParam>) -> impl Responder {
    let playlists = Playlist {
        name: param.name.clone(),
        songs: vec![],
    };
    web::Json(playlists)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playlist);
    cfg.service(get_playlist);
    cfg.service(create_playlist);
}
