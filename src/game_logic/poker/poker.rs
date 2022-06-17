use super::PokerPlayer;
use crate::cards::Deck;

pub struct Poker {
    players: Vec<Box<dyn PokerPlayer>>,
    deck: Deck,
}

impl Poker {
    /// Creates a new poker game given the number of players
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// use cards_framework::game_logic::poker::PokerPlayer;
    /// use cards_framework::game_logic::poker::BasicPlayer;
    /// let mut poker_players: Vec<Box<dyn PokerPlayer>> = Vec::new();
    /// // 3 Basic players
    /// let num_chips = 10000;
    /// let num_players = 5;
    /// for _ in 0..num_players {
    ///     poker_players.push(Box::new(BasicPlayer::new(num_chips)));
    /// }
    /// let poker_game = poker::Poker::new(poker_players);
    /// assert_eq!(poker_game.num_players(), num_players);
    /// ```
    pub fn new(players: Vec<Box<dyn PokerPlayer>>) -> Poker {
        Poker {
            players,
            deck: Deck::new(),
        }
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }
}
