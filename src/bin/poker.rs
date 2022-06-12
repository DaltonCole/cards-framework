use cards_framework::game_logic::poker;

fn main() {
    let STARTING_CHIPS = 10000;
    let NUM_PLAYERS = 5;
    let mut players = Vec::<poker::Player>::new();
    for _ in 0..NUM_PLAYERS {
        players.push(poker::Player::new(STARTING_CHIPS));
    }
}
