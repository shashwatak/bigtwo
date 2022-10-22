use core::fmt;
use std::str::FromStr;

use crate::card::{Card, ParseCardError};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hand {
    Lone(Card),
    Pair(Card, Card),
    Trips(Card, Card, Card),
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::Lone(a) => write!(f, "{}", a),
            Hand::Pair(a, b) => write!(f, "{} {}", a, b),
            Hand::Trips(a, b, c) => write!(f, "{} {} {}", a, b, c),
        }
    }
}

#[derive(Debug)]
pub enum ParseHandError {
    Empty,
    BadLen,
    BadCard(ParseCardError),
}

impl From<ParseCardError> for ParseHandError {
    fn from(error: ParseCardError) -> Self {
        Self::BadCard(error)
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(hand_str: &str) -> Result<Self, Self::Err> {
        match &hand_str.split(" ").collect::<Vec<&str>>()[..] {
            [] => Err(Self::Err::Empty),
            chars if chars.len() != 2 => Err(Self::Err::BadLen),
            [a] => 
            [number_char, suit_char] => {
                let maybe_number = number_char.to_string().parse::<Number>();
                let maybe_suit = suit_char.to_string().parse::<Suit>();
                match (maybe_number, maybe_suit) {
                    (Ok(number), Ok(suit)) => Ok(Hand { number, suit }),
                    (Err(e), _) => Err(Self::Err::BadNumber(e)),
                    (_, Err(e)) => Err(Self::Err::BadSuit(e)),
                }
            }
            _ => unreachable!(),
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
            assert!(matches!(hand, Err(ParseHandError::Empty)));
        }
        {
            let hand = "3CC".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::BadLen)));
        }
        {
            let hand = "SD".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::BadNumber(_))));
        }
        {
            let hand = "3K".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::BadSuit(_))));
        }
    }

    #[test]
    fn test_good_hand_to_from_string() {
        let good_hands = ["2S", "3C", "KD", "AH", "TS", "QC", "JD"];
        for expected_hand in good_hands {
            let hand = expected_hand.to_string().parse::<Hand>();
            assert!(matches!(hand, Ok(_)));
            let result_hand = hand.unwrap().to_string();
            assert_eq!(expected_hand, result_hand);
        }
    }

    #[test]
    fn test_hand_order() {
        assert!("2S".parse::<Hand>().unwrap() > "2D".parse::<Hand>().unwrap());
        assert!("2S".parse::<Hand>().unwrap() > "AS".parse::<Hand>().unwrap());
        assert!("TD".parse::<Hand>().unwrap() == "TD".parse::<Hand>().unwrap());
    }
}
