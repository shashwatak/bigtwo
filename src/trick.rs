mod check_player_can_play_hand;
use check_player_can_play_hand::check_player_can_play_hand;

use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::card::THREE_OF_CLUBS;
use crate::hand::Hand;
use crate::player::Player;

#[derive(Debug)]
pub struct Trick {
    pub hand: Hand,
    pub current_player_id: usize,
    pub passed_player_ids: BTreeSet<usize>,
}

#[derive(Debug)]
pub enum StartStatus {
    GameOver(usize),
}


#[derive(Debug)]
pub enum StepStatus {
    Played,
    Passed,
    TrickOver(usize),
    GameOver(usize),
}

impl Trick {
    pub fn new(starting_hand: Hand, next_player_id: usize) -> Self {
        if let Hand::Pass = starting_hand {
            panic!("Starting Hand cannot be Pass");
        }

        Self {
            hand: starting_hand,
            current_player_id: next_player_id,
            passed_player_ids: BTreeSet::new(),
        }
    }

    pub fn start(
        starting_player_id: usize,
        players: &mut [Player; 4],
        is_first: bool,
    ) -> Result<Self, StartStatus> {
        let player = &mut players[starting_player_id];
        let starting_hand: Hand;
        let trick: Trick;
        if is_first {
            starting_hand = loop {
                assert_eq!(player.cards[0], THREE_OF_CLUBS);
                let attempt = (player.start_game)(&player.cards);
                match attempt {
                    Hand::Trips(_, _, a) if a == THREE_OF_CLUBS => break attempt,
                    Hand::Pair(_, a) if a == THREE_OF_CLUBS => break attempt,
                    Hand::Lone(a) if a == THREE_OF_CLUBS => break attempt,
                    _ => println!("must play a hand that includes the Three of Clubs"),
                }
            };
        } else {
            starting_hand = (player.start_trick)(&player.cards);
        }

        player.remove_hand_from_cards(&starting_hand);
        if player.cards.len() == 0 {
            Err(StartStatus::GameOver(starting_player_id))
        } else {
            let next_player_id = Trick::next_player_id(starting_player_id, &BTreeSet::new());
            trick = Trick::new(starting_hand, next_player_id);
            Ok(trick)
        }
    }

    pub fn step(&mut self, players: &mut [Player; 4]) -> StepStatus {
        assert!(
            self.passed_player_ids.len() < 4 - 1,
            "there must be at least 2 players playing"
        );

        let player = &mut players[self.current_player_id];

        let submitted_hand = Trick::get_submitted_hand(player, &self.hand);

        let next_player_id = Trick::next_player_id(self.current_player_id, &self.passed_player_ids);

        if let Hand::Pass = submitted_hand {
            self.passed_player_ids.insert(self.current_player_id);

            if self.passed_player_ids.len() == (4 - 1) {
                StepStatus::TrickOver(next_player_id)
            } else {
                self.current_player_id = next_player_id;
                StepStatus::Passed
            }
        } else {
            player.remove_hand_from_cards(&submitted_hand);
            self.hand = submitted_hand;

            if player.cards.len() == 0 {
                StepStatus::GameOver(self.current_player_id)
            } else {
                self.current_player_id = next_player_id;
                StepStatus::Played
            }
        }
    }

    fn get_submitted_hand(player: &Player, hand_to_beat: &Hand) -> Hand {
        loop {
            let attempt = (player.submit_hand)(hand_to_beat, &player.cards);

            let is_attempt_allowed = check_player_can_play_hand(hand_to_beat, &player, &attempt);

            match is_attempt_allowed {
                Ok(()) => break attempt,
                Err(e) => println!("{}: {}", attempt, e),
            }
        }
    }
}

impl Display for Trick {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Current Player: {}\nHand To Beat: {}",
            self.current_player_id, self.hand
        )
    }
}

impl Trick {
    pub fn next_player_id(current_player_id: usize, passed_player_ids: &BTreeSet<usize>) -> usize {
        assert!(current_player_id < 4);
        assert!(
            passed_player_ids.len() < 4 - 1,
            "there must be at least 2 players playing"
        );
        for i in 1..4 {
            let next_id = (current_player_id + i) % 4;
            if !passed_player_ids.contains(&next_id) {
                assert_ne!(current_player_id, next_id);
                return next_id;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::tests::vec_card_from_str;

    #[test]
    fn test_next_player_id() {
        let current: usize = 0;
        let has_passed: BTreeSet<usize> = BTreeSet::new();
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next, 1);

        let current: usize = 3;
        let has_passed: BTreeSet<usize> = BTreeSet::new();
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next, 0);

        let current: usize = 0;
        let has_passed: BTreeSet<usize> = BTreeSet::from([1, 2]);
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next, 3);

        let current: usize = 2;
        let has_passed: BTreeSet<usize> = BTreeSet::from([0, 3]);
        let next = Trick::next_player_id(current, &has_passed);
        assert_eq!(next, 1);
    }

    #[test]
    fn test_trick_start() {
        // setup a trick where 4 players are dealt cards, P1 initializes the Trick with 3C, P2 is
        // next
        let mut players = <[Player; 4]>::default();
        players[0].cards = vec_card_from_str("AS");
        players[1].cards = vec_card_from_str("3C 4D 7S 8D");
        players[2].cards = vec_card_from_str("3H");
        players[3].cards = vec_card_from_str("7D");
        let starting_player_id: usize = 1;
        let trick: Trick = Trick::start(starting_player_id, &mut players, true).unwrap();

        match trick.hand {
            Hand::Lone(a) => assert_eq!(a, "3C".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 0);
        assert_eq!(trick.current_player_id, 2);
        assert_eq!(players[starting_player_id].cards.len(), 3);


        // setup a trick where 4 players are dealt cards, P1 initializes the Trick with 4D, P2 is
        // next
        let mut players = <[Player; 4]>::default();
        players[0].cards = vec_card_from_str("AS");
        players[1].cards = vec_card_from_str("4D 7S");
        players[2].cards = vec_card_from_str("3H");
        players[3].cards = vec_card_from_str("7D");
        let starting_player_id: usize = 1;
        let trick: Trick = Trick::start(starting_player_id, &mut players, false).unwrap();

        match trick.hand {
            Hand::Lone(a) => assert_eq!(a, "4D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 0);
        assert_eq!(trick.current_player_id, 2);
        assert_eq!(players[starting_player_id].cards.len(), 1);

        // setup a trick where the starting player will win as soon as they initialize the trick
        let mut players = <[Player; 4]>::default();
        players[0].cards = vec_card_from_str("2S");
        players[1].cards = vec_card_from_str("3D");
        players[2].cards = vec_card_from_str("3H");
        players[3].cards = vec_card_from_str("7D");
        let starting_player_id: usize = 2;
        let trick_start = Trick::start(starting_player_id, &mut players, false);
        assert!(matches!(trick_start, Err(StartStatus::GameOver(p)) if p == starting_player_id));

    }

    #[test]
    fn test_trick_step() {
        // setup a trick where 4 players are dealt cards, P0 initializes the Trick with 6D, P1 is
        // next
        let mut players = <[Player; 4]>::default();
        players[0].cards = vec_card_from_str("6D AS 2S");
        players[1].cards = vec_card_from_str("3D 4H");
        players[2].cards = vec_card_from_str("3H 4H");
        players[3].cards = vec_card_from_str("7D 4S");
        let starting_player_id: usize = 0;
        let mut trick: Trick = Trick::start(starting_player_id, &mut players, false).unwrap();

        // P1 plays 7D, then P2
        let step_status = trick.step(&mut players);
        assert!(matches!(step_status, StepStatus::Passed));
        match trick.hand {
            Hand::Lone(a) => assert_eq!(a, "6D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 1);
        assert!(trick.passed_player_ids.contains(&1));
        assert_eq!(trick.current_player_id, 2);

        // P2 must pass, then P3
        let step_status = trick.step(&mut players);
        assert!(matches!(step_status, StepStatus::Passed));
        match trick.hand {
            Hand::Lone(a) => assert_eq!(a, "6D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 3);

        // P3 plays 7D, then to P0
        let step_status = trick.step(&mut players);
        assert!(matches!(step_status, StepStatus::Played));
        match trick.hand {
            Hand::Lone(a) => assert_eq!(a, "7D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 0);

        // P0 plays Ace of Spades, then to P3 (skipping P1 and P2 who passed)
        let step_status = trick.step(&mut players);
        assert!(matches!(step_status, StepStatus::Played));
        match trick.hand {
            Hand::Lone(a) => assert_eq!(a, "AS".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 3);

        // P3 passes, Trick is Over and P0 won the Trick
        let step_status = trick.step(&mut players);
        match step_status {
            StepStatus::TrickOver(winner) => assert_eq!(winner, 0),
            a => panic!("{:?}", a),
        }
        match trick.hand {
            Hand::Lone(a) => assert_eq!(a, "AS".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 3);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert!(trick.passed_player_ids.contains(&3));
        assert_eq!(trick.current_player_id, 3);
    }

    #[test]
    fn test_trick_step_game_over() {
    }
}
