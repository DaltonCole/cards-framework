use crate::cards::Card;

pub struct Player {
    hand: Vec<Card>,
    chips: u32,
}

impl Player {
    pub fn new(starting_chips: u32) -> Player {
        Player { hand: Vec::<Card>::new(), chips: starting_chips,
        }
    }
}
