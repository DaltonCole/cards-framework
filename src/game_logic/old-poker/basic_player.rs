use super::{PlayerAction, Poker, PokerPlayer};
use crate::cards::Card;

pub struct BasicPlayer {
    hand: Vec<Card>,
}

impl PokerPlayer for BasicPlayer {
    /// Creates a new player with an empty hand and starting_chips chips
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// use cards_framework::game_logic::poker::PokerPlayer;
    /// let player = poker::BasicPlayer::new();
    /// ```
    fn new() -> BasicPlayer {
        BasicPlayer {
            hand: Vec::<Card>::new(),
        }
    }

    /// Gives N cards to the player
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// # use cards_framework::game_logic::poker::PokerPlayer;
    /// use cards_framework::cards::Deck;
    /// use cards_framework::cards::Card;
    ///
    /// let mut player = poker::BasicPlayer::new();
    /// let mut deck = Deck::new();
    /// player.get_cards(&mut deck.draw_cards(2).unwrap());
    /// ```
    fn get_cards(&mut self, new_cards: &mut Vec<Card>) {
        self.hand.append(new_cards);
    }

    /// Makes a random bet
    fn make_bet(&self, num_chips: u32, call_amount: u32) -> PlayerAction {
        // Call, can't check
        if call_amount == 0 {
            match rand::random::<u8>() % 5 {
                0 | 1 => return PlayerAction::Raise(rand::random::<u32>() % num_chips),
                2 => return PlayerAction::Fold,
                _ => return PlayerAction::Check,
            }
        }
        // Can't call, can check
        else {
            if call_amount > num_chips {
                return PlayerAction::Fold;
            } else {
                match rand::random::<u8>() % 10 {
                    0..=3 => return PlayerAction::Raise(rand::random::<u32>() % num_chips),
                    4 => return PlayerAction::Fold,
                    _ => return PlayerAction::Call,
                }
            }
        }
    }
}
