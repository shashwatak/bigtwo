use core::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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

#[derive(Debug)]
pub enum ParseSuitError {
    Empty,
    BadLen,
    BadChar(char),
}

impl FromStr for Suit {
    type Err = ParseSuitError;
    fn from_str(suit_str: &str) -> Result<Self, Self::Err> {
        match suit_str {
            "" => Err(Self::Err::Empty),
            c if c.len() >= 2 => Err(Self::Err::BadLen),
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
            let suit = "".to_string().parse::<Suit>();
            assert!(matches!(suit, Err(ParseSuitError::Empty)));
        }
        {
            let suit = "DD".to_string().parse::<Suit>();
            assert!(matches!(suit, Err(ParseSuitError::BadLen)));
        }
        {
            let suit = "T".to_string().parse::<Suit>();
            assert!(matches!(suit, Err(ParseSuitError::BadChar(_))));
        }
    }

    #[test]
    fn test_good_suit_to_from_string() {
        let good_suits = ["C", "D", "H", "S"];
        for expected_suit in good_suits {
            let suit = expected_suit.to_string().parse::<Suit>();
            assert!(matches!(suit, Ok(_)));
            let result_suit = suit.unwrap().to_string();
            assert_eq!(expected_suit, result_suit);
        }
    }
}
