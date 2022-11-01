use core::fmt;
use std::collections::BTreeSet;
use std::str::FromStr;

use crate::card::Card;
use crate::card::ParseCardError;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hand {
    Lone(Card),
    Pair(Card, Card),
    Trips(Card, Card, Card),
    Pass,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::Lone(a) => write!(f, "{}", a),
            Hand::Pair(a, b) => write!(f, "{} {}", a, b),
            Hand::Trips(a, b, c) => write!(f, "{} {} {}", a, b, c),
            Hand::Pass => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub enum ParseHandError {
    BadLen,
    BadCard(ParseCardError),
    DuplicateCard,
    NotSortedDescending,
    InvalidHand(InvalidHandError),
}

impl From<ParseCardError> for ParseHandError {
    fn from(e: ParseCardError) -> Self {
        Self::BadCard(e)
    }
}

#[derive(Debug)]
pub enum InvalidHandError {
    UnmatchedPair,
    UnmatchedTrips,
}

impl From<InvalidHandError> for ParseHandError {
    fn from(e: InvalidHandError) -> Self {
        Self::InvalidHand(e)
    }
}

impl Hand {
    pub fn try_from_cards(cards: &[Card]) -> Result<Hand, ParseHandError> {
        Self::sanitize_cards(cards)?;
        match cards {
            [] => Ok(Hand::Pass),
            [a] => Ok(Hand::Lone(*a)),
            [a, b] => Ok(Hand::try_pair(*a, *b)?),
            [a, b, c] => Ok(Hand::try_trips(*a, *b, *c)?),
            _ => Err(ParseHandError::BadLen),
        }
    }

    fn try_pair(first: Card, second: Card) -> Result<Hand, InvalidHandError> {
        if first.number == second.number {
            Ok(Hand::Pair(first, second))
        } else {
            Err(InvalidHandError::UnmatchedPair)
        }
    }

    fn try_trips(first: Card, second: Card, third: Card) -> Result<Hand, InvalidHandError> {
        if first.number == second.number && second.number == third.number {
            Ok(Hand::Trips(first, second, third))
        } else {
            Err(InvalidHandError::UnmatchedTrips)
        }
    }

    fn sanitize_cards(cards: &[Card]) -> Result<(), ParseHandError> {
        let mut unique_cards: BTreeSet<&Card> = BTreeSet::new();
        for card in cards {
            unique_cards.insert(&card);
        }
        if unique_cards.len() < cards.len() {
            return Err(ParseHandError::DuplicateCard);
        }

        for (i, card) in cards.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if cards[i - 1] < *card {
                return Err(ParseHandError::NotSortedDescending);
            }
        }

        Ok(())
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(hand_str: &str) -> Result<Hand, Self::Err> {
        let hand_str = hand_str.trim();
        if hand_str.len() == 0 {
            return Ok(Hand::Pass);
        }
        let maybe_cards = hand_str.split(' ').collect::<Vec<&str>>();
        let mut cards: Vec<Card> = vec![];
        for maybe_card in maybe_cards {
            cards.push(maybe_card.parse()?);
        }

        Hand::sanitize_cards(&cards[..])?;

        Hand::try_from_cards(&cards)
    }
}

impl Hand {
    pub fn is_same_type(previous: &Hand, attempted: &Hand) -> bool {
        match (previous, attempted) {
            (_, Hand::Pass) => true,
            (Hand::Lone(_), Hand::Lone(_)) => true,
            (Hand::Pair(_, _), Hand::Pair(_, _)) => true,
            (Hand::Trips(_, _, _), Hand::Trips(_, _, _)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_hand_to_from_string() {
        let hand = "AJ".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::BadCard(_))));

        let hand = "7D 5C 4C 3C".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::BadLen)));

        let hand = "3C 4D".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::NotSortedDescending)));

        let hand = "7D 3C 4C 5C 3C".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::DuplicateCard)));

        let hand = "2D 3S".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(InvalidHandError::UnmatchedPair))
        ));

        let hand = "2S 2H 3S".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(
                InvalidHandError::UnmatchedTrips
            ))
        ));
    }

    #[test]
    fn test_good_hand_to_from_string() {
        let good_hands = ["", "2S", "3D 3C", "KS KH KC"];
        for expected_hand in good_hands {
            let hand = expected_hand.to_string().parse::<Hand>();
            println!("{:?}", hand);
            assert!(matches!(hand, Ok(_)));
            let result_hand = hand.unwrap().to_string();
            assert_eq!(expected_hand, result_hand);
        }
    }

    #[test]
    fn test_hand_order() {
        assert!("2S".parse::<Hand>().unwrap() > "2D".parse::<Hand>().unwrap());
        assert!("2S 2D".parse::<Hand>().unwrap() > "AS AD".parse::<Hand>().unwrap());
        assert!("TS TD TC".parse::<Hand>().unwrap() < "TS TH TC".parse::<Hand>().unwrap());
        assert!("".parse::<Hand>().unwrap() > "2S 2H 2D".parse::<Hand>().unwrap());
    }
}
