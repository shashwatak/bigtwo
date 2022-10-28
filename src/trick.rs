use std::collections::BTreeSet;

use crate::hand::Hand;
use crate::player::Player;

pub struct Trick {
    pub hand: Hand,
    pub current_player_id: usize,
    pub passed_player_ids: BTreeSet<usize>,
}

pub enum PlayHandStatus {
    GameOver,
    TrickOver,
    Pass,
    SuccessPlay,
    FailedPlay(InvalidPlayedHand),
}

pub enum InvalidPlayedHand {
    WrongType,
    NotHighEnough,
}

impl From<InvalidPlayedHand> for PlayHandStatus {
    fn from(e: InvalidPlayedHand) -> Self {
       Self::FailedPlay(e) 
    }
}

impl Trick {

    fn try_play_hand(&mut self, hand: Hand) -> PlayHandStatus {
        // assert!(!self.passed_player_ids.contains(&self.current_player_id));
        if let Hand::Pass = hand {
            return PlayHandStatus::Pass;
            // self.passed_player_ids.insert(self.current_player_id);
        } 
            Trick::check_hand_playable(&self.hand, &hand);
            self.hand = hand;
            PlayHandStatus::SuccessPlay
    }

    fn is_same_type(previous: &Hand, attempted: &Hand) -> bool {
        match (previous, attempted) {
            (Hand::Lone(_), Hand::Lone(_)) => true,
            (Hand::Pair(_, _), Hand::Pair(_, _)) => true,
            (Hand::Trips(_, _, _), Hand::Trips(_, _, _)) => true,
            _ => false,
        }
    }

    fn check_hand_playable(previous: &Hand, attempted: &Hand) -> Result<(), InvalidPlayedHand> {
        if Trick::is_same_type(previous, attempted) {
            if attempted > previous {
                Ok(())
            } else {
                Err(InvalidPlayedHand::NotHighEnough)
            }
        } else {
            Err(InvalidPlayedHand::WrongType)
        }
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
    use crate::card::Card;

    #[test]
    fn test_check_hand_is_not_playable() {
        let previous: Hand = "3S 3C".to_string().parse().unwrap();
        let attempted: Hand = "3H 3D".to_string().parse().unwrap();
        assert!(matches!(
            Trick::check_hand_playable(&previous, &attempted),
            Err(InvalidPlayedHand::NotHighEnough)
        ));

        let previous: Hand = "3S 3C".to_string().parse().unwrap();
        let attempted: Hand = "4S 4H 4D".to_string().parse().unwrap();
        assert!(matches!(
            Trick::check_hand_playable(&previous, &attempted),
            Err(InvalidPlayedHand::WrongType)
        ));

        let previous: Hand = "3S 3C".to_string().parse().unwrap();
        let attempted: Hand = "2S".to_string().parse().unwrap();
        assert!(matches!(
            Trick::check_hand_playable(&previous, &attempted),
            Err(InvalidPlayedHand::WrongType)
        ));
    }

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
}
