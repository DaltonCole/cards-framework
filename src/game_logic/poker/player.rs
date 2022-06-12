use crate::cards::Card;

pub struct Player {
    hand: Vec<Card>,
    chips: u32,
}

impl Player {
    /// Creates a new player with an empty hand and starting_chips chips
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// let total_chips = 10000;
    /// let player = poker::Player::new(total_chips);
    /// assert_eq!(player.num_chips(), total_chips);
    /// ```
    pub fn new(starting_chips: u32) -> Player {
        Player {
            hand: Vec::<Card>::new(),
            chips: starting_chips,
        }
    }

    /// Gives N cards to the player
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// use cards_framework::cards::Deck;
    /// use cards_framework::cards::Card;
    ///
    /// let mut player = poker::Player::new(0);
    /// let mut deck = Deck::new();
    /// player.get_cards(&mut deck.draw_cards(2).unwrap());
    /// assert_eq!(player.num_cards(), 2);
    /// player.get_cards(&mut deck.draw_cards(1).unwrap());
    /// assert_eq!(player.num_cards(), 3);
    /// ```
    pub fn get_cards(&mut self, new_cards: &mut Vec<Card>) {
        self.hand.append(new_cards);
    }

    /// Number of cards the player is currently holding
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// # use cards_framework::cards::Deck;
    /// # use cards_framework::cards::Card;
    ///
    /// let mut player = poker::Player::new(0);
    /// let mut deck = Deck::new();
    /// player.get_cards(&mut deck.draw_cards(2).unwrap());
    /// # assert_eq!(player.num_cards(), 2);
    /// # player.get_cards(&mut deck.draw_cards(1).unwrap());
    /// # assert_eq!(player.num_cards(), 3);
    /// ```
    pub fn num_cards(&self) -> usize {
        self.hand.len()
    }

    /// Number of chips this player currently has
    ///
    /// # Example
    /// ```
    /// use cards_framework::game_logic::poker;
    /// let num_chips = 9001;
    /// let mut player = poker::Player::new(num_chips);
    /// assert_eq!(player.num_chips(), num_chips);
    /// ```
    pub fn num_chips(&self) -> u32 {
        self.chips
    }
}
