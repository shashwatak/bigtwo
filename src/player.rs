use std::fmt::Display;

use crate::{card::Card, hand::Hand};

pub struct Player {
    pub cards: Vec<Card>,
    pub submit_hand: fn(&Hand, &Vec<Card>) -> Hand,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            cards : vec!(),
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

pub const PLAY_SMALLEST_SINGLE_OR_PASS: fn(&Hand, &Vec<Card>) -> Hand = |hand, cards| Hand::Pass;

#[cfg(test)]
mod tests {

    #[test]
    fn test_play_smallest_single_or_pass() {
        
    }
}
