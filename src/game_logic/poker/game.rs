use super::{Player, PlayerInterface, Action, History, Table};
use crate::cards::{Deck, DeckError};

pub struct Game {
    table: Table,
    deck: Deck,
    small_blind_amount: u32,
    history: History,
}

impl Game {

    pub fn new(small_blind_amount: u32) -> Game {
        Game {
            table: Table::new(),
            deck: Deck::new(),
            small_blind_amount,
            history: History::new(),
        }
    }

    pub fn add_player(&mut self, player: PlayerInterface) {
        self.table.add_player(player);
    }

    pub fn play(&mut self) {
        let mut starting_player = 0;

        while self.table.players.len() > 1 {
            // Start a new turn history
            self.history.new_turn();

            // Make a new shuffled deck
            self.shuffle_entire_deck();

            // --- Take Blinds --- //
            // Take big blind from previous starting player
            self.table.players[starting_player].take_blind(self.small_blind_amount * 2);
            // Go to next player
            starting_player = self.next_player(starting_player);
            // Take small blind from first betting player
            self.table.players[starting_player].take_blind(self.small_blind_amount);

            // Deal out cards
            if let Err(_) = self.deal() {
                panic!("Error occurred while dealing cards");
            }

            // --- Bets --- //

        }
    }

    fn next_player(&self, player_index: usize) -> usize {
        (player_index + 1) % self.table.players.len()
    }

    /// Shuffles the entire deck
    fn shuffle_entire_deck(&mut self) {
        self.deck = Deck::new();
        self.deck.shuffle();
    }
    /// Deals out cards to each player
    fn deal(&mut self) -> Result<(), DeckError> {
        for player in &mut self.table.players {
            let drawn_cards = self.deck.draw_cards(2)?;
            player.new_hand(drawn_cards);
        }
        Ok(())
    }

    /// Goes through pre-flop betting
    ///
    /// Returns the player who won or None to continue game
    fn pre_flop(&mut self, first_player: usize) -> Option<usize> {
        let mut last_raise_player: Option<usize> = None;
        let mut round_betting_amount = 0;

        // New RoundHistory
        self.history.new_round();

        while self.is_round_ended() == false {
            for player_index in 0..self.table.players.len() {
                // Get the index of player who should perform an action next
                let index = (player_index + first_player) % self.table.players.len();

                // See if this player was the last to raise, if so, end round
                if last_raise_player == Some(index) {
                    return self.is_there_one_person_remaining();
                }

                // Player does action
                if let Action::Raise(raise_amount) = self.table.players[player_index].player_action(round_betting_amount, &self.history) {
                    round_betting_amount += raise_amount;
                    last_raise_player = Some(player_index);
                }

                // Add player action to history
                self.history.add_player_history(&self.table.players[player_index], player_index, false);
            }
        }
        return self.is_there_one_person_remaining();
    }

    // If only one player hasn't folded, return that player, else None
    fn is_there_one_person_remaining(&self) -> Option<usize> {
        let mut no_fold: Option<usize> = None;
        for (index, player) in self.table.players.iter().enumerate() {
            if *player.get_action() != Some(Action::Fold) {
                if no_fold != None {
                    return None;
                }
                no_fold = Some(index);
            }
        }
        return no_fold;
    }

    fn is_round_ended(&self) -> bool {
        for player in &self.table.players {
            match player.get_action() {
                Some(Action::Raise(_)) | None => { return false; },
                _ => {},
            }
        }
        return true;
    }
}
