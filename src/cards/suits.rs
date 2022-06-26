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
        let RED = "\033[0;31m";
        let BLACK = "\033[0;30m";
        let NORMAL = "\033[0m";
        match *self {
            Suit::Heart => write!(f, "{}♥{}", RED, NORMAL),
            Suit::Diamond => write!(f, "{}♦{}", RED, NORMAL),
            Suit::Club => write!(f, "{}♣{}", BLACK, NORMAL),
            Suit::Spade => write!(f, "{}♠{}", BLACK, NORMAL),
        }
    }
}
