use std::collections::BTreeSet;

use crate::hand::Hand;
use crate::player::Player;

pub struct Trick<'a> {
    pub hand: Hand,
    pub current_player_id: usize,
    pub passed_player_ids: BTreeSet<usize>,
    players: &'a [Player; 4],
}

pub enum InvalidPlayedHand {
    WrongType,
    NotHighEnough,
}

impl<'a> Trick<'a> {
    pub fn try_play_hand(&mut self, hand: Hand) -> Result<(), InvalidPlayedHand> {
        assert!(!self.passed_player_ids.contains(&self.current_player_id));
        if let Hand::Pass = hand {
            self.passed_player_ids.insert(self.current_player_id);
        } else {
            // assert!(Trick::check_player_has_cards(&self.players[self.current_player_id], hand));
            Trick::check_hand_playable(&self.hand, &hand)?;
            self.hand = hand;
        }
        Ok(())
    }

    fn is_same_type(previous: &Hand, attempted: &Hand) -> bool {
        match (previous, attempted) {
            (Hand::Lone(_), Hand::Lone(_)) => true,
            (Hand::Pair(_), Hand::Pair(_)) => true,
            (Hand::Trips(_), Hand::Trips(_)) => true,
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

    fn next_player(&self) -> Option<usize> {
        for i in 1..4 {
            let next_id = (self.current_player_id + i) % 4;
            if !self.passed_player_ids.contains(&next_id) {
                return Some(next_id);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
}
