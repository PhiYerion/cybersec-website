use crate::Genre::Genre;

pub struct Song<'a> {
    duration: u64,
    author:   &'a str,
    genre:    Genre,
}
