use crate::cards::Card;
use super::{Action, PlayerInterface};

pub struct PlayerHistory {
    // Position in turn order
    pub playerID: usize,
    // If the player has revealed their hand
    pub hand: Option<Vec<Card>>,
    pub chips: u32,
    pub gambled_chips: u32,
    pub action: Option<Action>,
}

pub struct RoundHistory {
    pub players: Vec<PlayerHistory>,
    pub pot_size: u32,
}

pub struct TurnHistory {
    pub rounds: Vec<RoundHistory>,
    pub pot_size: u32,
}

pub struct History {
    pub turns: Vec<TurnHistory>,
}

impl PlayerHistory {
    pub fn new(playerID: usize, hand: Option<Vec<Card>>, chips: u32, gambled_chips: u32, action: Option<Action>) -> PlayerHistory {
        PlayerHistory {
            playerID,
            hand,
            chips,
            gambled_chips,
            action,
        }
    }
}

impl RoundHistory {
    pub fn new() -> RoundHistory {
        RoundHistory {
            players: Vec::<PlayerHistory>::new(),
            pot_size: 0,
        }
    }

    pub fn add_player_history(&mut self, player: &PlayerInterface, playerID: usize, revealed_hand: bool) {
        let mut hand = None;
        if revealed_hand == true {
            hand = Some(player.get_hand());
        }
        self.players.push(PlayerHistory::new(playerID, hand, player.num_chips(), player.num_gambled_chips(), player.get_action().clone()));
    }
}

impl TurnHistory {
    pub fn new() -> TurnHistory {
        TurnHistory { 
            rounds: Vec::<RoundHistory>::new(),
            pot_size: 0,
        }
    }

    pub fn new_round(&mut self) {
        self.rounds.push(RoundHistory::new());
    }

    pub fn add_player_history(&mut self, player: &PlayerInterface, playerID: usize, revealed_hand: bool) {
        if let Some(current_round) = self.rounds.last_mut() {
            current_round.add_player_history(&player, playerID, revealed_hand);
        }
    }
}

impl History {
    pub fn new() -> History {
        History { turns: Vec::<TurnHistory>::new() }
    }

    pub fn new_turn(&mut self) {
        self.turns.push(TurnHistory::new());
    }

    pub fn new_round(&mut self) {
        match self.turns.last_mut() {
            Some(current_turn) => {
                current_turn.new_round();
            },
            None => {
                self.new_turn();
                self.new_round();
            }
        }
    }

    pub fn add_player_history(&mut self, player: &PlayerInterface, playerID: usize, revealed_hand: bool) {
        if let Some(current_turn) = self.turns.last_mut() {
            current_turn.add_player_history(&player, playerID, revealed_hand);
        }
    }
}
