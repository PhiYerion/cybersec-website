#[derive(Clone, Copy, Debug)]

/// A basic enumeration of all possible Genres in our website
pub enum Genre {
    Pop,
    Rock,
    Jazz,
    EDM,
}

impl Genre {
    /// Convert the Genre to string representation
    pub fn to_string(self: &Self) -> String {
        match self {
            Self::Pop => "Pop".into(),
            Self::Rock => "Rock".into(),
            Self::Jazz => "Jazz".into(),
            Self::EDM => "EDM".into(),
        }
    }
}
