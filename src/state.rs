use std::sync::{Arc, Mutex};

use crate::modules::music::domain::Playlist;

pub struct State {
    pub playlist: Arc<Mutex<Vec<Playlist>>>,
}
