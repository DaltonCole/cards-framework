use super::{Action, PlayerError, History};
use crate::cards::Card;

pub trait Player: {
    /// The action the player chooses to take
    ///
    /// * `call_amount`: The amount of chips required to stay in the round without going all in
    ///     * If 0, the check action may take place, otherwise, any other action is allowed
    /// * `player_info`: An immutable reference to YOU, the player
    /// * `table`: An immutable reference to the current state of the table. Use this to view the
    ///     community cards
    /// * `error`: If you tried to make an invalid move, this will be populated with the error
    // TODO: pass cards into this as immutable
    fn player_action(player: Box<dyn Player>, hand: &Vec<Card>, chips: u32, gamled_chips: u32, call_amount: u32, game_history: &History, error: &Option<PlayerError>) -> Action
        where
            Self: Sized;
}
