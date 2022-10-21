mod number;
mod suit;

use number::Number;
use suit::Suit;

use core::fmt;
use std::str::FromStr;

use self::{number::ParseNumberError, suit::ParseSuitError};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    number: Number,
    suit: Suit,
}


impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.suit)
    }
}

#[derive(Debug)]
pub enum ParseCardError {
    Empty,
    BadLen,
    BadNumber(ParseNumberError),
    BadSuit(ParseSuitError),
}

impl FromStr for Card {
    type Err = ParseCardError;
    fn from_str(cell_str: &str) -> Result<Self, Self::Err> {
        match &cell_str.chars().collect::<Vec<char>>()[..] {
            [] => Err(Self::Err::Empty),
            chars if chars.len() != 2 => Err(Self::Err::BadLen),
            [number_char, suit_char] => {
                let maybe_number = number_char.to_string().parse::<Number>();
                let maybe_suit = suit_char.to_string().parse::<Suit>();
                match (maybe_number, maybe_suit) {
                    (Ok(number), Ok(suit)) => Ok(Card {number, suit}),
                    (Err(e), _) => Err(Self::Err::BadNumber(e)),
                    (_, Err(e)) => Err(Self::Err::BadSuit(e)),
                }
            },
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
            assert!(matches!(cell, Err(ParseCardError::BadNumber(_))));
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
}
