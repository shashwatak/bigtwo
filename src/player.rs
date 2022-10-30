use crate::{card::Card, hand::Hand};

#[derive(Debug, Default)]
pub struct PassingPlayer {
    pub cards: Vec<Card>,
}

pub trait PlaysHands {
    fn playHand() -> Hand;
}

impl PlaysHands for PassingPlayer {
    fn playHand() -> Hand {
        Hand::Pass
    }
}
