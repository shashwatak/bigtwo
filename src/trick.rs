use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::hand::{Hand, InvalidPlayedHand};

#[derive(Debug)]
pub struct Trick {
    pub hand: Hand,
    pub current_player_id: usize,
    pub passed_player_ids: BTreeSet<usize>,
}

impl Trick {
    pub fn new(starting_hand: Hand, next_player_id: usize) -> Self {
        Self {
            hand: starting_hand,
            current_player_id: next_player_id,
            passed_player_ids: BTreeSet::new(),
        }
    }
}

impl Display for Trick {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Current Player: {}", self.current_player_id)
    }
}

#[derive(Debug)]
pub enum PlayHandStatus {
    TrickOver,
    FailedPlay(InvalidPlayedHand),
}

impl From<InvalidPlayedHand> for PlayHandStatus {
    fn from(e: InvalidPlayedHand) -> Self {
        Self::FailedPlay(e)
    }
}

impl Trick {
    fn check_current_player_can_play_hand(&self, hand: Hand) -> Result<Hand, PlayHandStatus> {

        assert!(self.passed_player_ids.len() < 4);
        assert!(!self.passed_player_ids.contains(&self.current_player_id));

        if self.passed_player_ids.len() >= 4 - 1 {
            return Err(PlayHandStatus::TrickOver); 
        }

        Hand::check_hand_playable(&self.hand, &hand)?;

        Ok(hand)
    }

    fn next_player_id(
        current_player_id: usize,
        passed_player_ids: &BTreeSet<usize>,
    ) -> Option<usize> {
        assert!(current_player_id < 4);
        for i in 1..4 {
            let next_id = (current_player_id + i) % 4;
            if !passed_player_ids.contains(&next_id) {
                return Some(next_id);
            }
        }
        None
    }

    fn trick_winner_player_id(&self) -> Option<usize> {
        if self.passed_player_ids.len() == 3 {
            Some(self.current_player_id)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_next_player_id() {
        let current: usize = 0;
        let has_passed: BTreeSet<usize> = BTreeSet::new();
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next.unwrap(), 1);

        let current: usize = 3;
        let has_passed: BTreeSet<usize> = BTreeSet::new();
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next.unwrap(), 0);

        let current: usize = 0;
        let has_passed: BTreeSet<usize> = BTreeSet::from([1, 2]);
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next.unwrap(), 3);

        let current: usize = 2;
        let has_passed: BTreeSet<usize> = BTreeSet::from([0, 3]);
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next.unwrap(), 1);

        let current: usize = 3;
        let has_passed: BTreeSet<usize> = BTreeSet::from([0, 1, 2]);
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next, None);
    }

    #[test]
    fn test_try_play_hand() {

        // new trick begins with a Three of Clubs (ostensibly by player 0), 
        let mut trick = Trick::new("3C".parse().unwrap(), 1);

        // plays a Three of Spades
        let hand = trick.check_current_player_can_play_hand("3S".parse().unwrap());
        assert!(matches!(hand, Ok(Hand::Lone(_))));

        // update hand
        trick.hand = hand.unwrap();

        // incorrectly plays a Three of Diamonds, reject
        let hand = trick.check_current_player_can_play_hand("3D".parse().unwrap());
        assert!(matches!(hand, Err(PlayHandStatus::FailedPlay(InvalidPlayedHand::NotHighEnough))));

        // incorrectly plays a pair of Three's, reject
        let hand = trick.check_current_player_can_play_hand("4H 4D".parse().unwrap());
        assert!(matches!(hand, Err(PlayHandStatus::FailedPlay(InvalidPlayedHand::WrongType))));

        // passes
        let hand = trick.check_current_player_can_play_hand("".parse().unwrap());
        assert!(matches!(hand, Ok(Hand::Pass)));

        trick.passed_player_ids.insert(1);
        trick.passed_player_ids.insert(2);
        trick.passed_player_ids.insert(3);
        trick.current_player_id = 0;
        let hand = trick.check_current_player_can_play_hand("4C".parse().unwrap());
        assert!(matches!(hand, Err(PlayHandStatus::TrickOver)));

    }
}
