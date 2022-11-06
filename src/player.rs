use std::fmt::Display;
use std::{collections::BTreeSet, io::BufRead};

use crate::{
    card::{Card, THREE_OF_CLUBS},
    hand::Hand,
};

pub struct Player {
    pub cards: Vec<Card>,
    pub submit_hand: fn(&Hand, &Vec<Card>) -> Hand,
    pub start_game: fn(&Vec<Card>) -> Hand,
    pub start_trick: fn(&Vec<Card>) -> Hand,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            cards: vec![],
            submit_hand: PLAY_SMALLEST_SINGLE_OR_PASS,
            start_game: USE_THREE_OF_CLUBS,
            start_trick: START_TRICK_WITH_SMALLEST_SINGLE,
        }
    }
}

pub fn cards_to_string(cards: &[Card]) -> String {
    cards.iter().map(|card| format!(" [{}] ", card)).collect()
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
            if let Ok(trips) = Hand::try_trips(c, b, a) {
                trips
            } else if let Ok(pair) = Hand::try_pair(b, a) {
                pair
            } else {
                Hand::Lone(a)
            }
        }
        _ => panic!("oop"),
    }
};

pub const PLAY_SMALLEST_SINGLE_OR_PASS: fn(&Hand, &Vec<Card>) -> Hand = |hand, cards| {
    if let Hand::Lone(c) = hand {
        for card in cards {
            if card > c {
                return Hand::Lone(*card);
            }
        }
    }
    Hand::Pass
};

pub const START_TRICK_WITH_SMALLEST_SINGLE: fn(&Vec<Card>) -> Hand = |cards| Hand::Lone(cards[0]);

pub fn get_user_input<Input: BufRead>(f: &mut Input) -> Hand {
    loop {
        println!("Please enter the cards you would like to play, seperated by spaces");
        print!("> ");
        let mut line = String::new();
        f.read_line(&mut line).unwrap();

        let mut cards = vec![];
        let mut card_errs = vec![];

        let card_strs: Vec<&str> = line.split_whitespace().collect();
        for card_str in card_strs {
            let maybe_card = card_str.parse::<Card>();
            match maybe_card {
                Err(e) => {
                    println!("error: could not understand {card_str}, {:?}", e);
                    card_errs.push(e);
                }
                Ok(c) => cards.push(c),
            }
        }
        if card_errs.is_empty() {
            cards.sort();
            cards.reverse();
            let maybe_hand = Hand::try_from_cards(&cards);
            if let Ok(hand) = maybe_hand {
                break hand;
            } else {
                println!("error: invalid hand {:?}", maybe_hand.err());
            }
        }
    }
}

impl Player {
    pub fn remove_hand_from_cards(&mut self, hand: &Hand) {
        assert!(self.has_cards(hand));
        match hand {
            Hand::Lone(a) => self.remove_cards_from_cards(&[*a]),
            Hand::Pair(a, b) => self.remove_cards_from_cards(&[*a, *b]),
            Hand::Trips(a, b, c) => self.remove_cards_from_cards(&[*a, *b, *c]),
            _ => unreachable!(),
        }
    }

    fn remove_cards_from_cards(&mut self, cards_to_remove: &[Card]) {
        for to_remove in cards_to_remove {
            let index = self
                .cards
                .iter()
                .position(|card| *card == *to_remove)
                .unwrap();
            self.cards.remove(index);
        }
    }

    pub fn has_cards(&self, hand: &Hand) -> bool {
        let cards: BTreeSet<&Card> = BTreeSet::from_iter(self.cards.iter());

        match hand {
            Hand::Lone(a) => cards.contains(a),
            Hand::Pair(a, b) => cards.contains(a) && cards.contains(b),
            Hand::Trips(a, b, c) => cards.contains(a) && cards.contains(b) && cards.contains(c),
            Hand::Pass => true,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::card::{number::Number, suit::Suit};
    use crate::test_util::tests::vec_card_from_str;

    #[test]
    fn test_play_smallest_single_or_pass() {
        let hand_to_beat: Hand = "4H".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(
            hand,
            Hand::Lone(Card {
                number: Number::Four,
                suit: Suit::Spades
            })
        ));

        let hand_to_beat: Hand = "4H 4C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));

        let hand_to_beat: Hand = "6C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));
    }

    #[test]
    fn test_use_three_of_clubs() {
        let cards = vec_card_from_str("3C 4C 5D 2S");
        let hand = USE_THREE_OF_CLUBS(&cards);
        assert!(matches!(hand, Hand::Lone(a) if a == THREE_OF_CLUBS));

        let cards = vec_card_from_str("3C 3D 5D 2S");
        let hand = USE_THREE_OF_CLUBS(&cards);
        assert!(matches!(hand, Hand::Pair(_, a) if a == THREE_OF_CLUBS));

        let cards = vec_card_from_str("3C 3D 3S 2S");
        let hand = USE_THREE_OF_CLUBS(&cards);
        assert!(matches!(hand, Hand::Trips(_, _, a) if a == THREE_OF_CLUBS));
    }

    #[test]
    fn test_has_cards() {
        let cards = vec_card_from_str("3C 3S 4H 4D 4S");
        let mut player = Player::default();
        player.cards = cards;

        let hand: Hand = "3C".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "3S 3C".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "4S 4H 4D".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "3D".parse().unwrap();
        assert!(!player.has_cards(&hand));

        let hand: Hand = "4S 4H 4C".parse().unwrap();
        assert!(!player.has_cards(&hand));
    }

    #[test]
    fn test_remove_cards_from_hand() {
        let mut player = Player::default();
        player.cards = vec_card_from_str("3D 3S 5S 6S");
        player.remove_hand_from_cards(&"3S 3D".parse().unwrap());
        assert!(!player.cards.contains(&"3S".parse().unwrap()));
        assert!(!player.cards.contains(&"3D".parse().unwrap()));
        assert!(player.cards.contains(&"5S".parse().unwrap()));
        assert!(player.cards.contains(&"6S".parse().unwrap()));
    }

    #[test]
    fn test_get_user_input() {
        let mut input = "3C".as_bytes();
        let hand = get_user_input(&mut input);
        assert!(matches!(hand, Hand::Lone(c) if c == THREE_OF_CLUBS));

        const THREE_OF_DIAMONDS: Card = Card {
            number: Number::Three,
            suit: Suit::Diamonds,
        };
        const THREE_OF_SPADES: Card = Card {
            number: Number::Three,
            suit: Suit::Spades,
        };

        let mut input = "3C 3S 3D".as_bytes();
        let hand = get_user_input(&mut input);
        assert!(
            matches!(hand, Hand::Trips(a, b, c) if a == THREE_OF_SPADES && b == THREE_OF_DIAMONDS && c == THREE_OF_CLUBS,)
        );

        let mut input = "3G\n3S 4D\n3C 3S 3D".as_bytes();
        let hand = get_user_input(&mut input);
        assert!(
            matches!(hand, Hand::Trips(a, b, c) if a == THREE_OF_SPADES && b == THREE_OF_DIAMONDS && c == THREE_OF_CLUBS,)
        );
    }
}
