use core::fmt;
use std::str::FromStr;

/// Represents the "number" on a Standard-52 card, ordered.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Two,
}

/// A convenience for iterating through the enums without std::ops::Index trait.
/// TODO: replace with Index trait and/or Iterator trait
const RANKS: [Rank; 13] = [
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
    Rank::Two,
];

impl Rank {

    /// A convenience for iterating through Rank's variants, without Index or Iterator trait.
    /// TODO: replace with Index trait and/or Iterator trait
    pub fn all() -> [Rank; 13] {
        RANKS
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "T"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
        }
    }
}

/// Represents the possible errors from attempting to parse a Rank from a string.
#[derive(Debug)]
pub enum ParseRankError {
    /// Empty string.
    Empty,
    /// Wrong number of chars.
    BadLen,
    /// parsed a character that is not part of any Rank
    BadChar(char),
}

impl FromStr for Rank {
    type Err = ParseRankError;
    fn from_str(rank_str: &str) -> Result<Self, Self::Err> {
        match rank_str {
            "" => Err(Self::Err::Empty),
            c if c.len() >= 2 => Err(Self::Err::BadLen),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            "2" => Ok(Rank::Two),
            c => Err(Self::Err::BadChar(c.chars().next().unwrap())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_rank_to_from_string() {
        {
            let rank = "".to_string().parse::<Rank>();
            assert!(matches!(rank, Err(ParseRankError::Empty)));
        }
        {
            let rank = "34".parse::<Rank>();
            assert!(matches!(rank, Err(ParseRankError::BadLen)));
        }
        {
            let rank = "R".parse::<Rank>();
            assert!(matches!(rank, Err(ParseRankError::BadChar(_))));
        }
    }

    #[test]
    fn test_good_rank_to_from_string() {
        let good_ranks = ["3", "7", "J", "A", "2"];
        for expected_rank in good_ranks {
            let rank = expected_rank.parse::<Rank>();
            assert!(matches!(rank, Ok(_)));
            let result_rank = rank.unwrap().to_string();
            assert_eq!(expected_rank, result_rank);
        }
    }
}
