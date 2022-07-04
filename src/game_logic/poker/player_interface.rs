use super::{Action, Player, PlayerError, History, BasicPlayer};
use crate::cards::Card;
use std::cmp::min;

pub struct PlayerInterface {
    player: Box<dyn Player>,
    hand: Vec<Card>,
    chips: u32,
    gambled_chips: u32,
    action: Option<Action>,
}

impl PlayerInterface {
    pub fn new(player: Box<dyn Player>, starting_chips: u32) -> PlayerInterface {
        PlayerInterface {
            player,
            hand: Vec::new(),
            chips: starting_chips,
            gambled_chips: 0,
            action: None,
        }
    }

    pub fn new_hand(&mut self, cards: Vec<Card>) {
        self.hand = cards;
        self.gambled_chips = 0;
        self.action = None;
    }

    pub fn get_hand(&self) -> Vec<Card> {
        self.hand
    }

    pub fn take_blind(&mut self, blind_amount: u32) {
        self.gambled_chips = min(blind_amount, self.chips);
        self.chips -= self.gambled_chips;
    }

    pub fn give_pot(&mut self, pot_size: u32) {
        self.chips += pot_size;
    }

    pub fn num_chips(&self) -> u32 {
        self.chips
    }

    pub fn num_gambled_chips(&self) -> u32 {
        self.gambled_chips
    }

    pub fn get_action(&self) -> &Option<Action> {
        &self.action
    }

    pub fn player_action(&mut self, bet_amount: u32, history: &History) -> Action {
        let err: Option<PlayerError> = None;
        let call_amount = bet_amount - self.gambled_chips;
        loop {
            //match self.player.player_action(self.player, &self.hand, self.chips, self.gambled_chips, call_amount, &history, &err) {
            let action = Player::player_action(self.player, &self.hand, self.chips, self.gambled_chips, call_amount, &history, &err);
            // TODO, make sure actions are valid
            return action;
        }
    }
}
