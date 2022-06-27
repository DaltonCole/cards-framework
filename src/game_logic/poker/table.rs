use super::PlayerInterface;
use crate::cards::Card;

pub struct Table {
    pub players: Vec<PlayerInterface>,
    pub flop: Option<Vec<Card>>,
    pub turn: Option<Vec<Card>>,
    pub river: Option<Vec<Card>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            players: Vec::new(),
            flop: None,
            turn: None,
            river: None,
        }
    }

    pub fn add_player(&mut self, player: PlayerInterface) {
        self.players.push(player);
    }

    pub fn get_pot_size(&self) -> u32 {
        let mut pot_size = 0;
        for player in &self.players {
            pot_size += player.num_gambled_chips();
        }
        pot_size
    }
}
