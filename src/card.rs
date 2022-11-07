pub mod rank;
pub mod suit;

use rank::Rank;
use suit::Suit;

use core::fmt;
use std::str::FromStr;

use self::{rank::ParseRankError, suit::ParseSuitError};

pub const THREE_OF_CLUBS: Card = Card {
    rank: Rank::Three,
    suit: Suit::Clubs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Debug)]
pub enum ParseCardError {
    Empty,
    BadLen,
    BadRank(ParseRankError),
    BadSuit(ParseSuitError),
}

impl From<ParseRankError> for ParseCardError {
    fn from(error: ParseRankError) -> Self {
        ParseCardError::BadRank(error)
    }
}

impl From<ParseSuitError> for ParseCardError {
    fn from(error: ParseSuitError) -> Self {
        ParseCardError::BadSuit(error)
    }
}

impl FromStr for Card {
    type Err = ParseCardError;
    fn from_str(cell_str: &str) -> Result<Self, Self::Err> {
        match &cell_str.chars().collect::<Vec<char>>()[..] {
            [] => Err(Self::Err::Empty),
            chars if chars.len() != 2 => Err(Self::Err::BadLen),
            [rank_char, suit_char] => {
                let rank = rank_char.to_string().parse::<Rank>()?;
                let suit = suit_char.to_string().parse::<Suit>()?;
                Ok(Card { rank, suit })
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_card_to_from_string() {
        {
            let card = "".to_string().parse::<Card>();
            assert!(matches!(card, Err(ParseCardError::Empty)));
        }
        {
            let cell = "3CC".to_string().parse::<Card>();
            assert!(matches!(cell, Err(ParseCardError::BadLen)));
        }
        {
            let cell = "SD".to_string().parse::<Card>();
            assert!(matches!(cell, Err(ParseCardError::BadRank(_))));
        }
        {
            let cell = "3K".to_string().parse::<Card>();
            assert!(matches!(cell, Err(ParseCardError::BadSuit(_))));
        }
    }

    #[test]
    fn test_good_card_to_from_string() {
        let good_cells = ["2S", "3C", "KD", "AH", "TS", "QC", "JD"];
        for expected_cell in good_cells {
            let cell = expected_cell.to_string().parse::<Card>();
            assert!(matches!(cell, Ok(_)));
            let result_cell = cell.unwrap().to_string();
            assert_eq!(expected_cell, result_cell);
        }
    }

    #[test]
    fn test_card_order() {
        assert!("2S".parse::<Card>().unwrap() > "2D".parse::<Card>().unwrap());
        assert!("2S".parse::<Card>().unwrap() > "AS".parse::<Card>().unwrap());
        assert!("TD".parse::<Card>().unwrap() == "TD".parse::<Card>().unwrap());
    }
}
