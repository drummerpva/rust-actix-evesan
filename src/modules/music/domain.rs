#[derive(Debug)]
pub struct Song {
    pub name: String,
    pub author: String,
    pub duration_ms: u16,
}

#[derive(Debug)]
pub struct Playlist {
    pub name: Song,
    pub songs: Vec<Song>,
}
