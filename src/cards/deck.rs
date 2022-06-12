use super::Card;
use super::Suit;
use super::Value;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

use thiserror::Error;
#[derive(Error, Debug, PartialEq, Eq)]
pub enum DeckError {
    #[error("Deck does not have enough cards! Num cards {0}. Cards requested {1}")]
    NotEnoughCards(usize, usize),
}

impl Deck {
    /// Creates a new deck of 52 cards in [Suit](super::Suit) then [Value](super::Value) order
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::new();

        for suit in Suit::iter() {
            for value in Value::iter() {
                cards.push(Card {
                    suit: suit,
                    value: value,
                });
            }
        }

        Deck { cards }
    }

    /// Shuffles the **remainder** of the deck
    ///
    /// # Examples
    /// ```
    /// use cards_framework::cards::Deck;
    /// let mut deck1 = Deck::new();
    /// let mut deck2 = Deck::new();
    /// assert_eq!(format!("{}", deck1), format!("{}", deck2));
    /// deck1.shuffle();
    /// assert_ne!(format!("{}", deck1), format!("{}", deck2));
    /// deck2.shuffle();
    /// assert_ne!(format!("{}", deck1), format!("{}", deck2));
    /// ```
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Returns the number of cards left in the deck
    ///
    /// # Examples
    /// ```
    /// use cards_framework::cards::Deck;
    /// let mut deck = Deck::new();
    /// assert_eq!(52, deck.count());
    /// deck.draw_cards(12);
    /// assert_eq!(40, deck.count());
    ///
    /// ```
    pub fn count(&self) -> usize {
        (*self).cards.len()
    }

    /// Draws N card(s) from the top (back) of the deck.
    ///
    /// There must be at least N cards left on the deck, otherwise an error is returned.
    ///
    /// # Examples
    /// ```
    /// use cards_framework::cards::Deck;
    /// let mut deck = Deck::new();
    /// assert_eq!(52, deck.count());
    /// let num_cards_to_draw = 12;
    /// if deck.count() > num_cards_to_draw {
    ///     let drawn_cards = deck.draw_cards(num_cards_to_draw).unwrap();
    ///     assert_eq!(num_cards_to_draw, drawn_cards.len());
    /// }
    /// assert_eq!(40, deck.count());
    /// ```
    ///
    /// Error Example
    /// ```
    /// use cards_framework::cards::Deck;
    /// use cards_framework::cards::DeckError;
    /// let mut deck = Deck::new();
    /// assert_eq!(deck.draw_cards(100), Err(DeckError::NotEnoughCards(52, 100)));
    /// ```
    pub fn draw_cards(&mut self, num: usize) -> Result<Vec<Card>, DeckError> {
        if self.count() < num {
            return Err(DeckError::NotEnoughCards(self.count(), num));
        }
        Ok(self.cards.drain((self.count() - num)..).collect())
    }
}

use std::fmt;
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for card in &self.cards {
            s += &format!("{} ", card);
        }
        write!(f, "{}", s)
    }
}
