use core::fmt;
use std::str::FromStr;

use crate::card::Card;
use crate::card::ParseCardError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hand {
    Lone([Card; 1]),
    Pair([Card; 2]),
    Trips([Card; 3]),
}

fn join_cards(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string()
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::Lone(cards) => write!(f, "{}", join_cards(cards)),
            Hand::Pair(cards) => write!(f, "{}", join_cards(cards)),
            Hand::Trips(cards) => write!(f, "{}", join_cards(cards)),
        }
    }
}

#[derive(Debug)]
pub enum ParseHandError {
    Empty,
    BadLen,
    BadCard(ParseCardError),
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

fn try_pair(cards: [Card; 2]) -> Result<Hand, InvalidHandError> {
    if cards[0].number == cards[1].number {
        Ok(Hand::Pair(cards))
    } else {
        Err(InvalidHandError::UnmatchedPair)
    }
}

fn try_trips(cards: [Card; 3]) -> Result<Hand, InvalidHandError> {
    if cards[0].number == cards[1].number && cards[0].number == cards[2].number {
        Ok(Hand::Trips(cards))
    } else {
        Err(InvalidHandError::UnmatchedTrips)
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(hand_str: &str) -> Result<Self, Self::Err> {
        match hand_str.split(' ').collect::<Vec<&str>>()[..] {
            [""] => Err(Self::Err::Empty),
            [a] => Ok(Hand::Lone([a.parse()?])),
            [a, b] => Ok(try_pair([a.parse()?, b.parse()?])?),
            [a, b, c] => Ok(try_trips([a.parse()?, b.parse()?, c.parse()?])?),
            _ => Err(Self::Err::BadLen),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_hand_to_from_string() {
        {
            let hand = "".to_string().parse::<Hand>();
            println!("{:?}", hand);
            assert!(matches!(hand, Err(ParseHandError::Empty)));
        }
        {
            let hand = "AJ".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::BadCard(_))));
        }
        {
            let hand = "3C 4C 5C 7D".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::BadLen)));
        }
        {
            let hand = "2D 3S".to_string().parse::<Hand>();
            assert!(matches!(
                hand,
                Err(ParseHandError::InvalidHand(InvalidHandError::UnmatchedPair))
            ));
        }
        {
            let hand = "2D 2H 3S".to_string().parse::<Hand>();
            assert!(matches!(
                hand,
                Err(ParseHandError::InvalidHand(
                    InvalidHandError::UnmatchedTrips
                ))
            ));
        }
    }

    #[test]
    fn test_good_hand_to_from_string() {
        let good_hands = ["2S", "3C 3D", "KD KH KC"];
        for expected_hand in good_hands {
            let hand = expected_hand.to_string().parse::<Hand>();
            assert!(matches!(hand, Ok(_)));
            let result_hand = hand.unwrap().to_string();
            assert_eq!(expected_hand, result_hand);
        }
    }

    #[test]
    fn test_hand_order() {
        // assert!("2S".parse::<Hand>().unwrap() > "2D".parse::<Hand>().unwrap());
        // assert!("2S".parse::<Hand>().unwrap() > "AS".parse::<Hand>().unwrap());
        // assert!("TD".parse::<Hand>().unwrap() == "TD".parse::<Hand>().unwrap());
    }
}
