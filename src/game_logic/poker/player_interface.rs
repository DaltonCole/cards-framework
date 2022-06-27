use super::{Action, Player, Table};
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

    pub fn player_action(&mut self, bet_amount: u32, table: &Table) -> Action {
        let err = None;
        let call_amount = bet_amount - self.gambled_chips;
        loop {
            self.action = Some(self.player.player_action(call_amount, &self, &table, &err));
            // TODO: make sure this is a valid action
            return self.action.unwrap();
        }
    }
}
