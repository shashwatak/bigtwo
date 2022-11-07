//! Represents a player in the game, could be AI or User.

mod get_user_input;

use std::collections::BTreeSet;
use std::fmt::Display;

use get_user_input::get_user_input;

use crate::{
    card::{Card, THREE_OF_CLUBS},
    hand::Hand,
};

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
    cards.iter().map(|card| format!(" |{}| ", card)).collect()
}

impl Player {
    /// Use this to transform any player from default AI into a User that
    /// accepts inputs from stdin.
    pub fn convert_to_stdio_user(&mut self) {
        self.submit_hand = |_, cards| {
            println!("=== Your Turn.");
            println!("-> {{ {} }}", cards_to_string(cards));
            get_user_input(&mut std::io::stdin().lock())
        };
        self.start_game = |cards| {
            println!("=== Please start the game using the |3C| .");
            println!("-> {{ {} }}", cards_to_string(cards));
            get_user_input(&mut std::io::stdin().lock())
        };
        self.start_trick = |cards| {
            println!("=== You may play any valid hand.");
            println!("-> {{ {} }}", cards_to_string(cards));
            get_user_input(&mut std::io::stdin().lock())
        };
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", cards_to_string(&self.cards))
    }
}

pub const USE_THREE_OF_CLUBS: fn(&Vec<Card>) -> Hand = |cards| {
    assert_eq!(cards[0], THREE_OF_CLUBS);

    match cards[..] {
        [a, b, c, ..] => {
            if let Ok(trips) = Hand::try_trips(c, b, a) {
                trips
            } else if let Ok(pair) = Hand::try_pair(b, a) {
                pair
            } else {
                Hand::Lone(a)
            }
        }
        _ => panic!("oop"),
    }
};

pub const PLAY_SMALLEST_SINGLE_OR_PASS: fn(&Hand, &Vec<Card>) -> Hand = |hand, cards| {
    if let Hand::Lone(c) = hand {
        for card in cards {
            if card > c {
                return Hand::Lone(*card);
            }
        }
    }
    Hand::Pass
};

pub const START_TRICK_WITH_SMALLEST_SINGLE: fn(&Vec<Card>) -> Hand = |cards| Hand::Lone(cards[0]);

impl Player {
    pub fn remove_hand_from_cards(&mut self, hand: &Hand) {
        assert!(self.has_cards(hand));
        match hand {
            Hand::Lone(a) => self.remove_cards_from_cards(&[*a]),
            Hand::Pair(a, b) => self.remove_cards_from_cards(&[*a, *b]),
            Hand::Trips(a, b, c) => self.remove_cards_from_cards(&[*a, *b, *c]),
            _ => unreachable!(),
        }
    }

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

    pub fn has_cards(&self, hand: &Hand) -> bool {
        Player::hand_in_cards(hand, &self.cards)
    }

    pub fn hand_in_cards(hand: &Hand, cards: &[Card]) -> bool {
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
    use crate::card::{rank::Rank, suit::Suit};
    use crate::test_util::tests::vec_card_from_str;

    #[test]
    fn test_play_smallest_single_or_pass() {
        let hand_to_beat: Hand = "4H".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(
            hand,
            Hand::Lone(Card {
                rank: Rank::Four,
                suit: Suit::Spades
            })
        ));

        let hand_to_beat: Hand = "4H 4C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));

        let hand_to_beat: Hand = "6C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));
    }

    #[test]
    fn test_use_three_of_clubs() {
        let cards = vec_card_from_str("3C 4C 5D 2S");
        let hand = USE_THREE_OF_CLUBS(&cards);
        assert!(matches!(hand, Hand::Lone(a) if a == THREE_OF_CLUBS));

        let cards = vec_card_from_str("3C 3D 5D 2S");
        let hand = USE_THREE_OF_CLUBS(&cards);
        assert!(matches!(hand, Hand::Pair(_, a) if a == THREE_OF_CLUBS));

        let cards = vec_card_from_str("3C 3D 3S 2S");
        let hand = USE_THREE_OF_CLUBS(&cards);
        assert!(matches!(hand, Hand::Trips(_, _, a) if a == THREE_OF_CLUBS));
    }

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
