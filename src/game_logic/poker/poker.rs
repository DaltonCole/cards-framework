use super::{PokerPlayer, PlayerAction, PlayerError, PokerPlayerInterface};
use crate::cards::Deck;

pub struct Poker {
    players: Vec<PokerPlayerInterface>,
    deck: Deck,
    pot_size: u32,
    big_blind_player: usize,
    big_blind: u32,
}

impl Poker {
    /// Creates a new poker game given the number of players
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// use cards_framework::game_logic::poker::PokerPlayerInterface;
    /// use cards_framework::game_logic::poker::PokerPlayer;
    /// use cards_framework::game_logic::poker::BasicPlayer;
    /// let mut poker_players: Vec<Box<dyn PokerPlayer>> = Vec::new();
    /// // 3 Basic players
    /// let num_chips = 10000;
    /// let num_players = 5;
    /// for _ in 0..num_players {
    ///     poker_players.push(Box::new(BasicPlayer::new()));
    /// }
    /// let poker_game = poker::Poker::new(poker_players, num_chips);
    /// assert_eq!(poker_game.num_players(), num_players);
    /// ```
    pub fn new(players: Vec<Box<dyn PokerPlayer>>, starting_chips: u32) -> Poker {
        let mut poker_players = Vec::new();
        for player in players {
            poker_players.push(PokerPlayerInterface::new(player, starting_chips)); // TODO: Is this efficient?
        }
        Poker {
            players: poker_players,
            deck: Deck::new(),
            pot_size: 0,
            big_blind_player: 0,
            big_blind: 5,
        }
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn pot_size(&self) -> u32 {
        self.pot_size
    }

    pub fn play(&mut self) {
        // While there are multiple players left
        while self.num_alive_players() > 1 {
            // Reset necessary internal variables to start a new round
            self.new_round();
            // Deal cards
            self.deal();
            // Make bets
            self.make_bets(self.big_blind);
        }
    }

    /// Get the number of players that still has chips
    pub fn num_alive_players(&self) -> usize {
        let mut count = 0;
        for player in &self.players {
            if player.is_in_round() {
                count += 1;
            }
        }
        count
    }

    /// Reset fields so a new round may start
    fn new_round(&mut self) {
        self.pot_size = 0;
        for player in &mut self.players {
            player.new_round();
        }
    }

    /// Finds the next player in play. Return value will be the same as passed in index if no other
    /// players are left in the round.
    fn next_living_player(&self, index: usize) -> usize {
        let mut moving_index = index;
        for _ in 0..self.num_players() {
            moving_index = (moving_index + 1) % self.players.len();
            if self.players[moving_index].is_in_round() {
               break; 
            }
        }
        moving_index
    }

    /// Deal two cards to each player
    fn deal(&mut self) {
        // Re-shuffle cards
        self.deck = Deck::new();
        self.deck.shuffle();

        // Deal cards to all players
        for player in &mut self.players {
            match self.deck.draw_cards(2) {
                Ok(mut cards) => player.player.get_cards(&mut cards),
                // Shuffle in a new deck. Is this normal poker, no, but I wanted to get used to
                // match statements... TODO: Make more like poker
                Err(_e) => {
                    println!("There are too many players, re-shuffling deck");
                    self.deck = Deck::new();
                    self.deck.shuffle();
                }
            }
        }
    }

    /// True if all players are finished betting for this round
    fn done_betting(&self) -> bool {
        for player in &self.players {
            if let &Some(PlayerAction::Raise(_)) = player.get_action() { 
                return false;
            }
        }
        return true;
    }

    /// Allow all players still in this round check, raise, call, or fold
    ///
    /// If there is only one player remaining after bets are made, the index of the winning player
    /// is returned
    fn make_bets(&mut self, bet: mut u32, last_betting_player: mut usize) -> Option<usize> {
        let mut next_player = self.big_blind_player;
        loop {
            // Go to next player
            next_player = self.next_living_player();

            // Only one player left
            if self.num_alive_players() <= 1 {
                return Some(self.next_living_player(0)); // Get only active player
            }
            // Everyone has finished betting
            if next_player == last_betting_player {
                return None;
            }

            // Done betting for this round
            if self.done_betting() == true {
                return None;
            }
            // Get player's action
            let player_needs_to_bet_amount = bet - 
            self.players[next_player].make_bets(self.pot_size, bet - self.players[], )




        }
    }


}

#[cfg(test)]
mod poker_tests {
    use super::super::*;
    use super::*;
    static NUM_PLAYERS: usize = 5;
    static NUM_STARTING_CHIPS: u32 = 10000;

    #[test]
    fn test_number_alive_players() {
        let mut game = poker_game_setup();
        game.new_round();
        assert_eq!(game.num_players(), NUM_PLAYERS);
        assert_eq!(game.num_alive_players(), NUM_PLAYERS);
    }

    // Make sure we deal cards from the deck for each player
    #[test]
    fn test_deal_cards() {
        let mut game = poker_game_setup();
        // Start out with all of the cards
        assert_eq!(game.deck.count(), 52);
        // Deal out cards
        game.deal();
        // Correct number of cards left
        assert_eq!(game.deck.count(), 52 - (2 * NUM_PLAYERS));
    }

    #[test]
    fn test_done_betting() {
        let game = poker_game_setup();

        assert_eq!(game.done_betting(), true);
    }

    fn poker_game_setup() -> Poker {
        let mut poker_players: Vec<Box<dyn PokerPlayer>> = Vec::new();

        for _ in 0..NUM_PLAYERS {
            poker_players.push(Box::new(BasicPlayer::new()));
        }
        Poker::new(poker_players, NUM_STARTING_CHIPS)
    }
}
