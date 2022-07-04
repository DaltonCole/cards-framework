use super::{Action, Player, PlayerError, History};
use crate::cards::Card;

pub struct BasicPlayer {}

impl Player for BasicPlayer {
    /// Makes a random valid bet
    fn player_action(player: Box<dyn Player>, hand: &Vec<Card>, chips: u32, gamled_chips: u32, call_amount: u32, game_history: &History, error: &Option<PlayerError>) -> Action {
        // Call, can't check
        if call_amount == 0 {
            match rand::random::<u8>() % 5 {
                0 | 1 => return Action::Raise(rand::random::<u32>() % chips),
                2 => return Action::Fold,
                _ => return Action::Check,
            }
        }
        // Can't call, can check
        else {
            if call_amount > chips {
                return Action::Fold;
            } else {
                match rand::random::<u8>() % 10 {
                    0..=3 => return Action::Raise(rand::random::<u32>() % chips),
                    4 => return Action::Fold,
                    _ => return Action::Call,
                }
            }
        }
    }
}
