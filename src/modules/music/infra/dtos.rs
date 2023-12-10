use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetPlaylistParam {
    pub id: usize,
}

#[derive(Deserialize)]
pub struct CreatePlaylistParam {
    pub name: String,
}
