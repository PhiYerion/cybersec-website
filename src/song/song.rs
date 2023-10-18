use crate::song::Genre;

#[derive(Debug, Clone)]
pub struct Song {
    pub title: String,
    pub duration: u64,
    pub author: String,
    pub genre: Genre,
}
