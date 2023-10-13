

#[derive(Clone, Copy, Debug)]
pub enum Genre {
    Pop,
    Rock,
    Jazz,
    EDM,
}

impl Genre {
    pub fn to_string(self: &Self) -> String {
        match self { 
            Self::Pop   => { "Pop".into() },
            Self::Rock  => { "Rock".into() },
            Self::Jazz  => { "Jazz".into() },
            Self::EDM   => { "EDM".into() }
        }
    }
}
