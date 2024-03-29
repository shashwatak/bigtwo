//! Represents the suit on a Standard-52 card, ordered.
use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Represents the suit on a Standard-52 card, ordered.
/// In Big Two, the convention is to order the Suits alphabetically.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// A convenience for iterating through Suit's variants, without Index or Iterator trait.
/// TODO: replace with Index trait and/or Iterator trait
const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

impl Suit {
    /// A convenience for iterating through Suit's variants, without Index or Iterator trait.
    /// TODO: replace with Index trait and/or Iterator trait
    pub fn all() -> [Suit; 4] {
        SUITS
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "C"),
            Suit::Diamonds => write!(f, "D"),
            Suit::Hearts => write!(f, "H"),
            Suit::Spades => write!(f, "S"),
        }
    }
}

/// Represents the possible errors from attempting to parse a Suit from a string.
#[derive(Debug)]
pub enum ParseSuitError {
    /// Empty string.
    Empty,
    /// Wrong number of chars.
    BadLength,
    /// parsed a character that is not part of any Suit
    BadChar(char),
}

impl FromStr for Suit {
    type Err = ParseSuitError;
    fn from_str(suit_str: &str) -> Result<Self, Self::Err> {
        match suit_str {
            "" => Err(Self::Err::Empty),
            c if c.len() >= 2 => Err(Self::Err::BadLength),
            "C" => Ok(Suit::Clubs),
            "D" => Ok(Suit::Diamonds),
            "H" => Ok(Suit::Hearts),
            "S" => Ok(Suit::Spades),
            c => Err(Self::Err::BadChar(c.chars().next().unwrap())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_suit_to_from_string() {
        {
            let suit = "".parse::<Suit>();
            assert!(matches!(suit, Err(ParseSuitError::Empty)));
        }
        {
            let suit = "DD".parse::<Suit>();
            assert!(matches!(suit, Err(ParseSuitError::BadLength)));
        }
        {
            let suit = "T".parse::<Suit>();
            assert!(matches!(suit, Err(ParseSuitError::BadChar(_))));
        }
    }

    #[test]
    fn test_good_suit_to_from_string() {
        let good_suits = ["C", "D", "H", "S"];
        for expected_suit in good_suits {
            let suit = expected_suit.parse::<Suit>();
            assert!(matches!(suit, Ok(_)));
            let result_suit = suit.unwrap().to_string();
            assert_eq!(expected_suit, result_suit);
        }
    }
}
