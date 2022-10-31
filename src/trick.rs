use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter}, iter::FromIterator,
};

use crate::hand::Hand;
use crate::card::Card;
use crate::player::PassingPlayer;

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
pub enum PlayHandError {
    NotMatch,
    NotHighEnough,
    NotPlayerCards,
}

impl Trick {
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

    fn check_player_can_play_hand(current: &Hand, player: &PassingPlayer, attempt: &Hand) -> Result<(), PlayHandError> {

        if !Hand::is_same_type(current, attempt) {
            return Err(PlayHandError::NotMatch);
        }
        
        if current > attempt {
            return Err(PlayHandError::NotHighEnough);
        }

        if !Trick::check_player_has_cards(&player.cards, attempt) {
            return Err(PlayHandError::NotPlayerCards); 
        }

        Ok(())
    }

    fn check_player_has_cards(cards: &Vec<Card>, hand: &Hand) -> bool {
        let cards : BTreeSet<&Card> = BTreeSet::from_iter(cards.iter());

        match hand {
            Hand::Lone(a) => cards.contains(a),
            Hand::Pair(a, b) => cards.contains(a) && cards.contains(b),
            Hand::Trips(a, b, c) => cards.contains(a) && cards.contains(b) && cards.contains(c),
            Hand::Pass => true,
        }

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
        
        // player has a few cards
        let mut player = PassingPlayer { cards: vec!["3S".parse().unwrap(), ] };

        // plays a Three of Spades
        let hand : Hand = "3S".parse().unwrap();
        let res = Trick::check_player_can_play_hand(&trick.hand, &player, &hand);
        assert!(matches!(res, Ok(())));

        // update hand
        trick.hand = hand;

        // incorrectly plays a Three of Diamonds, reject
        let hand : Hand = "3D".parse().unwrap();
        let res = Trick::check_player_can_play_hand(&trick.hand, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::NotHighEnough)));

        // incorrectly plays a pair of Three's, reject
        let hand : Hand = "4H 4D".parse().unwrap();
        let res = Trick::check_player_can_play_hand(&trick.hand, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::NotMatch)));

        // passes
        let hand : Hand = "".parse().unwrap();
        let res = Trick::check_player_can_play_hand(&trick.hand, &player, &hand);
        assert!(matches!(res, Ok(_)));
    }
}
