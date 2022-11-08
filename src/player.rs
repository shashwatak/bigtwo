//! Represents a player in the game, could be AI or User.

mod get_ai_input;
mod get_user_input;

use std::collections::BTreeSet;
use std::fmt::Display;

use get_ai_input::{
    PLAY_SMALLEST_SINGLE_OR_PASS, START_TRICK_WITH_SMALLEST_SINGLE, USE_THREE_OF_CLUBS,
};
use get_user_input::get_user_input;

use crate::{card::Card, hand::Hand};

/// Represents a player in the game, could be AI or User.
/// We use settable Function-Pointers / Closures to change from AI to User.
/// NOTE: Using settable Function-Pointers instead of Traits/Generics because
/// it's just a bit easier for me right now.
pub struct Player {
    pub cards: Vec<Card>,
    pub submit_hand: fn(&Hand, &Vec<Card>) -> Hand,
    pub start_game: fn(&Vec<Card>) -> Hand,
    pub start_trick: fn(&Vec<Card>) -> Hand,
}

impl Default for Player {
    /// Returns an AI Player with the defult movesets.
    fn default() -> Self {
        Self {
            cards: vec![],
            submit_hand: PLAY_SMALLEST_SINGLE_OR_PASS,
            start_game: USE_THREE_OF_CLUBS,
            start_trick: START_TRICK_WITH_SMALLEST_SINGLE,
        }
    }
}

/// useful for printing
fn cards_to_string(cards: &[Card]) -> String {
    cards.iter().map(|card| format!("|{}|", card)).collect()
}

impl Player {
    /// Use this to transform any player from default AI into a User that
    /// accepts inputs from stdin.
    pub fn convert_to_stdio_user(&mut self) {
        self.submit_hand = |_, cards| {
            println!("=== Your Turn.");
            println!("=== {}", cards_to_string(cards));
            get_user_input(&mut std::io::stdin().lock())
        };
        self.start_game = |cards| {
            println!("=== Please start the game using the |3C| .");
            println!("=== {}", cards_to_string(cards));
            get_user_input(&mut std::io::stdin().lock())
        };
        self.start_trick = |cards| {
            println!("=== You may play any valid hand.");
            println!("=== {}", cards_to_string(cards));
            get_user_input(&mut std::io::stdin().lock())
        };
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", cards_to_string(&self.cards))
    }
}

impl Player {
    /// Used by the caller / game logic to take a Player's cards (ostensibly after the Player has
    /// played them legally).
    pub fn remove_hand_from_cards(&mut self, hand: &Hand) {
        assert!(self.has_cards(hand));
        match hand {
            Hand::Lone(a) => self.remove_cards_from_cards(&[*a]),
            Hand::Pair(a, b) => self.remove_cards_from_cards(&[*a, *b]),
            Hand::Trips(a, b, c) => self.remove_cards_from_cards(&[*a, *b, *c]),
            _ => unreachable!(),
        }
    }

    /// Used internally to remove any slice of cards from a Player's cards.
    /// # Panics
    /// - Will panic if any of cards_to_remove are not in Player's cards.
    fn remove_cards_from_cards(&mut self, cards_to_remove: &[Card]) {
        for to_remove in cards_to_remove {
            let index = self
                .cards
                .iter()
                .position(|card| *card == *to_remove)
                .unwrap();
            self.cards.remove(index);
        }
    }

    /// Used to make sure the Player actually has the cards they tried to play.
    pub fn has_cards(&self, hand: &Hand) -> bool {
        Player::hand_in_cards(hand, &self.cards)
    }

    /// Used internally, converts players cards into BTreeSet to check if cards from hand are
    /// present.
    fn hand_in_cards(hand: &Hand, cards: &[Card]) -> bool {
        let cards: BTreeSet<&Card> = BTreeSet::from_iter(cards);
        match hand {
            Hand::Lone(a) => cards.contains(a),
            Hand::Pair(a, b) => cards.contains(a) && cards.contains(b),
            Hand::Trips(a, b, c) => cards.contains(a) && cards.contains(b) && cards.contains(c),
            Hand::Pass => true,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::test_util::vec_card_from_str;

    #[test]
    fn test_has_cards() {
        let cards = vec_card_from_str("3C 3S 4H 4D 4S");
        let mut player = Player::default();
        player.cards = cards;

        let hand: Hand = "3C".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "3S 3C".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "4S 4H 4D".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "3D".parse().unwrap();
        assert!(!player.has_cards(&hand));

        let hand: Hand = "4S 4H 4C".parse().unwrap();
        assert!(!player.has_cards(&hand));
    }

    #[test]
    fn test_remove_cards_from_hand() {
        let mut player = Player::default();
        player.cards = vec_card_from_str("3D 3S 5S 6S");
        player.remove_hand_from_cards(&"3S 3D".parse().unwrap());
        assert!(!player.cards.contains(&"3S".parse().unwrap()));
        assert!(!player.cards.contains(&"3D".parse().unwrap()));
        assert!(player.cards.contains(&"5S".parse().unwrap()));
        assert!(player.cards.contains(&"6S".parse().unwrap()));
    }
}
