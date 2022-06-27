use super::{Action, Player, PlayerError, PlayerInterface, Table};

pub struct BasicPlayer {}

impl Player for BasicPlayer {
    /// Creates a new player
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// let player = poker::BasicPlayer::new();
    /// ```
    fn new() -> BasicPlayer {
        BasicPlayer {}
    }

    /// Makes a random valid bet
    fn player_action(&self, call_amount: u32, player_info: &PlayerInterface, _table: &Table, error: &Option<PlayerError>) -> Action {
        // Call, can't check
        if call_amount == 0 {
            match rand::random::<u8>() % 5 {
                0 | 1 => return Action::Raise(rand::random::<u32>() % player_info.num_chips()),
                2 => return Action::Fold,
                _ => return Action::Check,
            }
        }
        // Can't call, can check
        else {
            if call_amount > player_info.num_chips() {
                return Action::Fold;
            } else {
                match rand::random::<u8>() % 10 {
                    0..=3 => return Action::Raise(rand::random::<u32>() % player_info.num_chips()),
                    4 => return Action::Fold,
                    _ => return Action::Call,
                }
            }
        }
    }
}
