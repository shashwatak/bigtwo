use std::collections::BTreeSet;
use std::fmt::Display;

use crate::{
    card::{Card, THREE_OF_CLUBS},
    hand::Hand,
};

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
            if let Ok(trips) = Hand::try_trips(c, b, a) {
                return trips;
            } else if let Ok(pair) = Hand::try_pair(b, a) {
                return pair;
            } else {
                return Hand::Lone(a);
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
    use crate::card::{number::Number, suit::Suit};
    use crate::test_util::tests::vec_card_from_str;

    #[test]
    fn test_play_smallest_single_or_pass() {
        let hand_to_beat: Hand = "4H".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(
            hand,
            Hand::Lone(Card {
                number: Number::Four,
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
