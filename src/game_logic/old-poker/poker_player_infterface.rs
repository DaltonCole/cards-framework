use std::cmp::min;
use super::{Poker, PokerPlayer, PlayerAction, PlayerError};
use crate::cards::Card;

pub struct PokerPlayerInterface {
    pub player: Box<dyn PokerPlayer>,
    hand: Vec<Card>,
    chips: u32,
    pub gambled_chips: u32,
    action: Option<PlayerAction>,
}

impl PokerPlayerInterface {

    pub fn new(player: Box<dyn PokerPlayer>, starting_chips: u32) -> PokerPlayerInterface {
        PokerPlayerInterface {
            player,
            hand: Vec::new(),
            chips: starting_chips,
            gambled_chips: 0,
            action: None,
        }
    }

    /// True if this player is in the round. Player will no longer be in game when number of chips
    /// hit 0.
    pub fn is_in_round(&self) -> bool {
        (self.action == None || self.action != Some(PlayerAction::Fold)) && self.chips != 0
    }

    /// Gets the player's action
    pub fn get_action(&self) -> &Option<PlayerAction> {
        &self.action
    }

    pub fn new_round(&mut self) {
        self.action = None;
        self.gambled_chips = 0;
    }

    pub fn num_chips(&self) -> u32 {
        self.chips
    }

    pub fn take_chips_for_blind(&mut self, num_chips: u32) -> Result<(), PlayerError> {
        match self.chips >= num_chips {
            true => {
                self.chips -= num_chips;
                Ok(())
            }
            false => Err(PlayerError::NotEnoughForBlind),
        }
    }

    pub fn give_pot(&mut self, pot_size: u32) {
        self.chips += pot_size;
    }

    /// Player makes a bet. Updates the player interface member variables.
    ///
    /// PlayerError::CheckActionOnBet error is returned if the player checked when a bet was
    /// given
    pub fn make_bet(&mut self, mut call_amount: u32) -> Result<PlayerAction, PlayerError> {
        // All in
        if self.gambled_chips > 0 && self.chips == 0 {
            Some(PlayerAction::Check);
        }

        // Get amount needed to call
        call_amount -= self.gambled_chips;

        // Get Action
        match self.player.make_bet(self.chips, call_amount) {
            PlayerAction::Call => {
                let bet_chips = min(call_amount, self.chips);
                self.chips -= bet_chips;
                self.gambled_chips += bet_chips;
                self.action = Some(PlayerAction::Call);
                Ok(PlayerAction::Call)
            }, PlayerAction::Raise(mut bet_chips) => {
                bet_chips = min(bet_chips, self.chips);
                self.chips -= bet_chips;
                self.gambled_chips += self.gambled_chips;
                self.action = Some(PlayerAction::Raise(bet_chips));
                Ok(PlayerAction::Raise(bet_chips))
            }, PlayerAction::Check => {
                if call_amount != 0 {
                    return Err(PlayerError::CheckActionOnBet);
                }
                self.action = Some(PlayerAction::Check);
                Ok(PlayerAction::Check)
            }, PlayerAction::Fold => {
                self.action = Some(PlayerAction::Fold);
                Ok(PlayerAction::Fold)
            }
        }
    }
}

#[cfg(test)]
mod poker_player_interface_tests {
    use super::*;
    use super::super::*;
    static NUM_STARTING_CHIPS: u32 = 10000;

    #[test]
    fn test_new() {
        let player = player_setup();
        assert_eq!(player.num_chips(), NUM_STARTING_CHIPS);
    }

    #[test]
    fn test_take_chips_for_blind() {
        let mut player = player_setup();
        match player.take_chips_for_blind(NUM_STARTING_CHIPS / 2) {
            Ok(()) => assert_eq!(player.num_chips(), NUM_STARTING_CHIPS / 2),
            Err(_) => panic!("Test failed"),
        }
    }

    #[test]
    fn test_get_action() {
        let mut player = player_setup();
        player.action = None;
        assert_eq!(player.get_action(), &None);
        player.action = Some(PlayerAction::Call);
        assert_eq!(player.get_action(), &Some(PlayerAction::Call));
        player.action = Some(PlayerAction::Raise(50));
        assert_eq!(player.get_action(), &Some(PlayerAction::Raise(50)));
        player.action = Some(PlayerAction::Check);
        assert_eq!(player.get_action(), &Some(PlayerAction::Check));
        player.action = Some(PlayerAction::Fold);
        assert_eq!(player.get_action(), &Some(PlayerAction::Fold));
    }

    fn player_setup() -> PokerPlayerInterface {
        let basic_poker_player = Box::new(BasicPlayer::new());
        PokerPlayerInterface::new(basic_poker_player, NUM_STARTING_CHIPS)
    }
}
