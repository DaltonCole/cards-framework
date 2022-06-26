
struct PokerPlayerAPI {

}

pub trait PokerPlayerAPI {

}
use super::{Poker, PlayerAction};
use crate::cards::Card;

use thiserror::Error;
#[derive(Error, Debug, PartialEq, Eq)]
pub enum PlayerError {
    #[error("Not enough chips for blind!")]
    NotEnoughForBlind,
}

pub trait PokerPlayerAPI {
    /// Creates a new Poker Player API Interface
    fn new() -> Self;

    /// Gives N cards to the player
    fn get_cards(&mut self, new_cards: &mut Vec<Card>);

    /// Number of cards the player is currently holding
    fn num_cards(&self) -> usize;

    /// Number of chips this player currently has. 0 if the player is out of the game
    fn num_chips(&self) -> u32;

    /// Makes bet
    fn make_bet(&self, game: &Poker, call_amount: u32) -> PlayerAction;

    /// Take blind
    fn take_blind(&mut self, blind: u32) -> Result<(), PlayerError>;
}
