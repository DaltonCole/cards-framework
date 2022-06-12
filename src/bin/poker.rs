use cards_framework::game_logic::poker;

fn main() {
    let starting_chips = 10000;
    let num_players = 5;
    let mut players = Vec::<poker::Player>::new();
    for _ in 0..num_players {
        players.push(poker::Player::new(starting_chips));
    }
}
