use super::{Action, PlayerError, PlayerInterface, Table};

pub trait Player {
    fn new() -> Self
        where
            Self: Sized;

    /// The action the player chooses to take
    ///
    /// * `call_amount`: The amount of chips required to stay in the round without going all in
    ///     * If 0, the check action may take place, otherwise, any other action is allowed
    /// * `player_info`: An immutable reference to YOU, the player
    /// * `table`: An immutable reference to the current state of the table. Use this to view the
    ///     community cards
    /// * `error`: If you tried to make an invalid move, this will be populated with the error
    fn player_action(&self, call_amount: u32, player_info: &PlayerInterface, table: &Table, error: &Option<PlayerError>) -> Action;
}
