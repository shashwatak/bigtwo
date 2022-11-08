use crate::card::{Card, THREE_OF_CLUBS};
use crate::hand::Hand;

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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::card::{rank::Rank, suit::Suit};
    use crate::tests::test_util::vec_card_from_str;

    #[test]
    fn test_play_smallest_single_or_pass() {
        let hand_to_beat: Hand = "4H".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = (PLAY_SMALLEST_SINGLE_OR_PASS)(&hand_to_beat, &player_cards);
        assert!(matches!(
            hand,
            Hand::Lone(Card {
                rank: Rank::Four,
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
}
