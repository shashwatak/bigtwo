use crate::{card::Card, hand::Hand};

#[derive(Debug, Default)]
pub struct PassingPlayer {
    pub cards: Vec<Card>,
}

pub trait PlaysHands {
    fn play_hand() -> Hand;
}

impl PlaysHands for PassingPlayer {
    fn play_hand() -> Hand {
        Hand::Pass
    }
}
