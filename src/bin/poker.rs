use cards_framework::game_logic::poker;
use cards_framework::game_logic::poker::PokerPlayer;

fn main() {
    let starting_chips = 10000;
    let num_players = 5;
    let mut players = Vec::<poker::BasicPlayer>::new();
    for _ in 0..num_players {
        players.push(poker::BasicPlayer::new(starting_chips));
    }
}
