use std::fmt::Display;
use std::collections::BTreeSet;

use crate::{card::{Card, THREE_OF_CLUBS}, hand::Hand};

pub struct Player {
    pub cards: Vec<Card>,
    pub submit_hand: fn(&Hand, &Vec<Card>) -> Hand,
    pub start_game: fn(&Vec<Card>) -> Hand,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            cards: vec![],
            submit_hand: PLAY_SMALLEST_SINGLE_OR_PASS,
            start_game: USE_THREE_OF_CLUBS,
        }
    }
}

fn cards_to_string(cards: &Vec<Card>) -> String {
    cards.iter().map(|card| card.to_string()).collect()
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
            if let Ok(trips) = Hand::try_trips(a,b,c) {
                return trips;
            } else if let Ok(pair) = Hand::try_pair(a, b) {
                return pair;
            } else {
                return Hand::Lone(a)
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


impl Player {
    pub fn remove_hand_from_cards(&mut self, hand: &Hand) {
    }
    
    pub fn has_cards(&self, hand: &Hand) -> bool {
        let cards: BTreeSet<&Card> = BTreeSet::from_iter(self.cards.iter());

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
    use crate::test_util::tests::vec_card_from_str;
    use crate::card::{number::Number, suit::Suit};

    #[test]
    fn test_play_smallest_single_or_pass() {

        let hand_to_beat : Hand = "4H".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Lone(Card { number: Number::Four, suit: Suit::Spades })));

        let hand_to_beat : Hand = "4H 4C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));

        let hand_to_beat : Hand = "6C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));
    }

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
}
