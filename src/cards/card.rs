use super::Suit;
use super::Value;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn value(&self) -> u8 {
        (*self).value.value()
    }
}

use std::fmt;
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.value)
    }
}
