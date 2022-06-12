use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

use std::fmt;
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Heart => write!(f, "H"),
            Suit::Diamond => write!(f, "D"),
            Suit::Club => write!(f, "C"),
            Suit::Spade => write!(f, "S"),
        }
    }
}
