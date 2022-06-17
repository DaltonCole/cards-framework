use crate::cards::Card;

pub trait PokerPlayer {
    /// Creates a new player with an empty hand and starting_chips chips
    fn new(starting_chips: u32) -> Self
    where
        Self: Sized;

    /// Gives N cards to the player
    fn get_cards(&mut self, new_cards: &mut Vec<Card>);

    /// Number of cards the player is currently holding
    fn num_cards(&self) -> usize;

    /// Number of chips this player currently has
    fn num_chips(&self) -> u32;

    /// Makes bet
    fn make_bet(&self) -> u32;
}
