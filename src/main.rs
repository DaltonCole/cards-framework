use cards_framework::cards::Card;
use cards_framework::cards::Deck;
use cards_framework::cards::Suit;
use cards_framework::cards::Value;

fn main() {
    let s4 = Card {
        suit: Suit::Spade,
        value: Value::Four,
    };
    println!("{:?}", Value::Three as u8);
    println!("{}", Value::Three.value());
    println!("{}", s4.value());
    println!("{:?}", s4);

    let mut deck = Deck::new();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);
}
