use super::{Poker, PlayerAction};
use crate::cards::Card;

use thiserror::Error;
#[derive(Error, Debug, PartialEq, Eq)]
pub enum PlayerError {
    #[error("Not enough chips for blind!")]
    NotEnoughForBlind,
}

pub trait PokerPlayer {
    /// Creates a new player with an empty hand and starting_chips chips
    fn new() -> Self
    where
        Self: Sized;

    /// Gives N cards to the player
    fn get_cards(&mut self, new_cards: &mut Vec<Card>);

    /// Makes bet
    ///
    /// * `num_chips`: Number of chips this player currently has
    /// * `call_amount`: Amount this player needs to bet in order to call
    /// * `game`: Current game state. Use this to query game state.
    fn make_bet(&self, num_chips: u32, call_amount: u32, game: &Poker) -> PlayerAction;
}
