mod check_player_can_play_hand;
use check_player_can_play_hand::check_player_can_play_hand;

mod next_player_id;
use next_player_id::next_player_id;

use std::collections::BTreeSet;

use crate::card::THREE_OF_CLUBS;
use crate::hand::Hand;
use crate::player::Player;

#[derive(Debug)]
pub struct Trick {
    hand: Vec<Hand>,
    current_player_id: usize,
    passed_player_ids: BTreeSet<usize>,
}

#[derive(Debug)]
pub enum GameContinueStatus {
    NewTrick(usize),
    GameOver(usize),
}

#[derive(Debug)]
enum StepStatus {
    Continue,
    TrickOver(usize),
    GameOver(usize),
}

impl Trick {
    pub fn start(starting_player_id: usize, players: &mut [Player; 4], is_first: bool) -> Self {
        let player = &mut players[starting_player_id];

        let starting_hand = if is_first {
            loop {
                assert_eq!(player.cards[0], THREE_OF_CLUBS);
                let attempt = (player.start_game)(&player.cards);
                match attempt {
                    Hand::Trips(_, _, a) if a == THREE_OF_CLUBS => break attempt,
                    Hand::Pair(_, a) if a == THREE_OF_CLUBS => break attempt,
                    Hand::Lone(a) if a == THREE_OF_CLUBS => break attempt,
                    _ => println!("Must play a hand that includes the Three of Clubs."),
                }
            }
        } else {
            loop {
                let attempt = (player.start_trick)(&player.cards);
                if let Hand::Pass = attempt {
                    println!("Starting Hand cannot be Pass.");
                    continue;
                }
                break attempt;
            }
        };

        println!("Player {starting_player_id} begins with {starting_hand}");
        player.remove_hand_from_cards(&starting_hand);

        let next_player_id = next_player_id(starting_player_id, &BTreeSet::new());

        Self {
            hand: vec![starting_hand],
            current_player_id: next_player_id,
            passed_player_ids: BTreeSet::new(),
        }
    }

    pub fn do_trick(&mut self, players: &mut [Player; 4]) -> GameContinueStatus {
        // its possible the trick is started and the game is over instantly because
        // the player that started the trick finished their cards
        if let StepStatus::GameOver(winner) = self.is_trick_over(players) {
            return GameContinueStatus::GameOver(winner);
        }

        loop {
            self.do_player_turn(players);
            let trick_status = self.is_trick_over(players);
            match trick_status {
                StepStatus::Continue => continue,
                StepStatus::TrickOver(last_player) => {
                    break GameContinueStatus::NewTrick(last_player)
                }
                StepStatus::GameOver(winner) => break GameContinueStatus::GameOver(winner),
            }
        }
    }

    fn do_player_turn(&mut self, players: &mut [Player; 4]) {
        assert!(
            self.passed_player_ids.len() < 4 - 1,
            "there must be at least 2 players who have not yet passed"
        );

        assert!(
            players.iter().all(|p| !p.cards.is_empty()),
            "all players must have some cards in order to step (game should have \
            ended when any player went to 0 cards)"
        );

        let player = &mut players[self.current_player_id];
        let submitted_hand = Trick::get_submitted_hand(player, self.hand.last().unwrap());

        if let Hand::Pass = submitted_hand {
            println!("Player {} passed", self.current_player_id);
            self.passed_player_ids.insert(self.current_player_id);
        } else {
            println!(
                "Player {} played {}",
                self.current_player_id, submitted_hand
            );
            player.remove_hand_from_cards(&submitted_hand);
            self.hand.push(submitted_hand);
        }
        self.current_player_id = next_player_id(self.current_player_id, &self.passed_player_ids);
    }

    fn is_trick_over(&self, players: &[Player; 4]) -> StepStatus {
        for (player_id, player) in players.iter().enumerate() {
            if player.cards.is_empty() {
                return StepStatus::GameOver(player_id);
            }
        }

        if self.passed_player_ids.len() == (4 - 1) {
            for (player_id, _) in players.iter().enumerate() {
                if !self.passed_player_ids.contains(&player_id) {
                    return StepStatus::TrickOver(player_id);
                }
            }
            unreachable!();
        } else {
            StepStatus::Continue
        }
    }

    fn get_submitted_hand(player: &Player, hand_to_beat: &Hand) -> Hand {
        loop {
            let attempt = (player.submit_hand)(hand_to_beat, &player.cards);

            let is_attempt_allowed = check_player_can_play_hand(hand_to_beat, player, &attempt);

            match is_attempt_allowed {
                Ok(()) => break attempt,
                Err(e) => println!("{}: {}", attempt, e),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::tests::vec_card_from_str;

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
        let trick: Trick = Trick::start(starting_player_id, &mut players, true);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));

        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"3C".parse().unwrap()),
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
        let trick: Trick = Trick::start(starting_player_id, &mut players, false);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));

        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"4D".parse().unwrap()),
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
        let trick = Trick::start(starting_player_id, &mut players, false);
        assert!(
            matches!(trick.is_trick_over(&players), StepStatus::GameOver(p) if p == starting_player_id)
        );
        assert_eq!(trick.passed_player_ids.len(), 0);
        assert_eq!(trick.current_player_id, 3);
        assert_eq!(players[starting_player_id].cards.len(), 0);
    }

    #[test]
    fn test_trick_start_step_trick_over() {
        // setup a trick where 4 players are dealt cards, P0 initializes the Trick with 6D, P1 is
        // next
        let mut players = <[Player; 4]>::default();
        players[0].cards = vec_card_from_str("6D AS 2S");
        players[1].cards = vec_card_from_str("3D 4H");
        players[2].cards = vec_card_from_str("3H 4D");
        players[3].cards = vec_card_from_str("7D 4S");
        let starting_player_id: usize = 0;
        let mut trick: Trick = Trick::start(starting_player_id, &mut players, false);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));

        // P1 plays 7D, then P2
        trick.do_player_turn(&mut players);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"6D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 1);
        assert!(trick.passed_player_ids.contains(&1));
        assert_eq!(trick.current_player_id, 2);

        // P2 must pass, then P3
        trick.do_player_turn(&mut players);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"6D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 3);

        // P3 plays 7D, then to P0
        trick.do_player_turn(&mut players);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"7D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 0);

        // P0 plays Ace of Spades, then to P3 (skipping P1 and P2 who passed)
        trick.do_player_turn(&mut players);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"AS".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 3);

        // P3 passes, Trick is Over and P0 won the Trick
        trick.do_player_turn(&mut players);
        match trick.is_trick_over(&players) {
            StepStatus::TrickOver(winner) => assert_eq!(winner, 0),
            a => panic!("{:?}", a),
        }
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"AS".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 3);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert!(trick.passed_player_ids.contains(&3));
        assert_eq!(trick.current_player_id, 0);
    }

    #[test]
    fn test_trick_start_step_game_over() {
        // setup a trick where 4 players are dealt cards, P0 initializes the Trick with 6D, P1 is
        // next
        let mut players = <[Player; 4]>::default();
        players[0].cards = vec_card_from_str("6D AS");
        players[1].cards = vec_card_from_str("3D 4H");
        players[2].cards = vec_card_from_str("3H 4D");
        players[3].cards = vec_card_from_str("7D 4S");
        let starting_player_id: usize = 0;
        let mut trick: Trick = Trick::start(starting_player_id, &mut players, false);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));

        // P1 plays 7D, then P2
        trick.do_player_turn(&mut players);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"6D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 1);
        assert!(trick.passed_player_ids.contains(&1));
        assert_eq!(trick.current_player_id, 2);

        // P2 must pass, then P3
        trick.do_player_turn(&mut players);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"6D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 3);

        // P3 plays 7D, then to P0
        trick.do_player_turn(&mut players);
        assert!(matches!(
            trick.is_trick_over(&players),
            StepStatus::Continue
        ));
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"7D".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 0);

        // P0 plays Ace of Spades, Game is now over!
        trick.do_player_turn(&mut players);
        assert!(
            matches!(trick.is_trick_over(&players), StepStatus::GameOver(p) if p == starting_player_id)
        );
        match trick.hand.last().unwrap() {
            Hand::Lone(a) => assert_eq!(a, &"AS".parse().unwrap()),
            a => panic!("{}", a),
        }
        assert_eq!(trick.passed_player_ids.len(), 2);
        assert!(trick.passed_player_ids.contains(&1));
        assert!(trick.passed_player_ids.contains(&2));
        assert_eq!(trick.current_player_id, 3);
    }
}
