use std::fmt::Display;

use crate::{card::{Card, THREE_OF_CLUBS}, hand::Hand};

pub struct Player {
    pub cards: Vec<Card>,
    pub submit_hand: fn(&Hand, &Vec<Card>) -> Hand,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            cards: vec![],
            submit_hand: PLAY_SMALLEST_SINGLE_OR_PASS,
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
pub const PLAY_SMALLEST_SINGLE_OR_PASS: fn(&Hand, &Vec<Card>) -> Hand = |hand, cards| Hand::Pass;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::tests::vec_card_from_str;

    #[test]
    fn test_play_smallest_single_or_pass() {}

    fn test_use_three_of_clubs() {
        let cards = vec_card_from_str("3C 4C 5D");
        let hand = USE_THREE_OF_CLUBS(&cards);
        assert!(matches!(hand, Hand::Lone(a) if a == THREE_OF_CLUBS));
    }
}
