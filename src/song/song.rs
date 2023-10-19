use crate::song::Genre;

/// A simple song struct.
#[derive(Debug, Clone)]
pub struct Song {
    pub title: String,
    pub duration: u64,
    pub author: String,
    pub genre: Genre,
}
