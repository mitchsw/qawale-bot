use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Default)]
#[repr(u8)]
pub enum Stone {
    #[default]
    Neutral,
    Red,
    White,
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stone::Neutral => write!(f, "n"),
            Stone::Red => write!(f, "R"),
            Stone::White => write!(f, "W"),
        }
    }
}
