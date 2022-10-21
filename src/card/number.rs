use core::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
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

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Three => write!(f, "3"),
            Number::Four => write!(f, "4"),
            Number::Five => write!(f, "5"),
            Number::Six => write!(f, "6"),
            Number::Seven => write!(f, "7"),
            Number::Eight => write!(f, "8"),
            Number::Nine => write!(f, "9"),
            Number::Ten => write!(f, "T"),
            Number::Jack => write!(f, "J"),
            Number::Queen => write!(f, "Q"),
            Number::King => write!(f, "K"),
            Number::Ace => write!(f, "A"),
            Number::Two => write!(f, "2"),
        }
    }
}

#[derive(Debug)]
pub enum ParseNumberError {
    Empty,
    BadLen,
    BadChar(char),
}

impl FromStr for Number {
    type Err = ParseNumberError;
    fn from_str(number_str: &str) -> Result<Self, Self::Err> {
        match number_str {
            "" => Err(Self::Err::Empty),
            c if c.len() >= 2 => Err(Self::Err::BadLen),
            "3" => Ok(Number::Three),
            "4" => Ok(Number::Four),
            "5" => Ok(Number::Five),
            "6" => Ok(Number::Six),
            "7" => Ok(Number::Seven),
            "8" => Ok(Number::Eight),
            "9" => Ok(Number::Nine),
            "T" => Ok(Number::Ten),
            "J" => Ok(Number::Jack),
            "Q" => Ok(Number::Queen),
            "K" => Ok(Number::King),
            "A" => Ok(Number::Ace),
            "2" => Ok(Number::Two),
            c => Err(Self::Err::BadChar(c.chars().next().unwrap())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_number_to_from_string() {
        {
            let number = "".to_string().parse::<Number>();
            assert!(matches!(number, Err(ParseNumberError::Empty)));
        }
        {
            let number = "34".to_string().parse::<Number>();
            assert!(matches!(number, Err(ParseNumberError::BadLen)));
        }
        {
            let number = "R".to_string().parse::<Number>();
            assert!(matches!(number, Err(ParseNumberError::BadChar(_))));
        }
    }

    #[test]
    fn test_good_number_to_from_string() {
        let good_numbers = ["3", "7", "J", "A", "2"];
        for expected_number in good_numbers {
            let number = expected_number.to_string().parse::<Number>();
            assert!(matches!(number, Ok(_)));
            let result_number = number.unwrap().to_string();
            assert_eq!(expected_number, result_number);
        }
    }
}
