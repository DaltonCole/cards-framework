use super::PokerPlayer;
use crate::cards::Card;

pub struct BasicPlayer {
    hand: Vec<Card>,
    chips: u32,
}

impl PokerPlayer for BasicPlayer {
    /// Creates a new player with an empty hand and starting_chips chips
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// use cards_framework::game_logic::poker::PokerPlayer;
    /// let total_chips = 10000;
    /// let player = poker::BasicPlayer::new(total_chips);
    /// assert_eq!(player.num_chips(), total_chips);
    /// ```
    fn new(starting_chips: u32) -> BasicPlayer {
        BasicPlayer {
            hand: Vec::<Card>::new(),
            chips: starting_chips,
        }
    }

    /// Gives N cards to the player
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// # use cards_framework::game_logic::poker::PokerPlayer;
    /// use cards_framework::cards::Deck;
    /// use cards_framework::cards::Card;
    ///
    /// let mut player = poker::BasicPlayer::new(0);
    /// let mut deck = Deck::new();
    /// player.get_cards(&mut deck.draw_cards(2).unwrap());
    /// assert_eq!(player.num_cards(), 2);
    /// player.get_cards(&mut deck.draw_cards(1).unwrap());
    /// assert_eq!(player.num_cards(), 3);
    /// ```
    fn get_cards(&mut self, new_cards: &mut Vec<Card>) {
        self.hand.append(new_cards);
    }

    /// Number of cards the player is currently holding
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// # use cards_framework::game_logic::poker::PokerPlayer;
    /// # use cards_framework::cards::Deck;
    /// # use cards_framework::cards::Card;
    ///
    /// let mut player = poker::BasicPlayer::new(0);
    /// let mut deck = Deck::new();
    /// player.get_cards(&mut deck.draw_cards(2).unwrap());
    /// # assert_eq!(player.num_cards(), 2);
    /// # player.get_cards(&mut deck.draw_cards(1).unwrap());
    /// # assert_eq!(player.num_cards(), 3);
    /// ```
    fn num_cards(&self) -> usize {
        self.hand.len()
    }

    /// Number of chips this player currently has
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// # use cards_framework::game_logic::poker::PokerPlayer;
    /// let num_chips = 9001;
    /// let mut player = poker::BasicPlayer::new(num_chips);
    /// assert_eq!(player.num_chips(), num_chips);
    /// ```
    fn num_chips(&self) -> u32 {
        self.chips
    }

    /// Makes a random bet
    ///
    /// #Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// # use cards_framework::game_logic::poker::PokerPlayer;
    /// let num_chips = 9001;
    /// let mut player = poker::BasicPlayer::new(num_chips);
    /// player.make_bet();
    /// ```
    fn make_bet(&self) -> u32 {
        rand::random::<u32>() % self.chips
    }
}
