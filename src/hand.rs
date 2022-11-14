//! Represents any one of the allowed combinations of cards (known as a "Hand").
//! Cannot be used to represent an unrecognized / nonsensical combination.


pub mod iter;
pub mod try_from;

use core::fmt;

use crate::card::Card;

/// Represents any one of the allowed combinations of cards (known as a "Hand").
/// Cannot be used to represent an unrecognized / nonsensical combination.
/// TODO: Fivers: Straight, Flush, FullHouse, FourPlusKicker, StraightFlush
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hand {
    /// aka Singles, Highs, Loners, Solos
    Lone(Card),
    /// aka Dubs, Dual, Two-of-a-Kind
    Pair(Card, Card),
    /// aka Three-of-a-Kind
    Trips(Card, Card, Card),
    /// 5 Cards with consecutive Rank
    Straight(Card, Card, Card, Card, Card),
    /// 5 Cards of the same Suit
    Flush(Card, Card, Card, Card, Card),
    /// No Hand, No Cards
    Pass,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: Vec<String> = vec![];
        for card in self.cards() {
            out.push(card.to_string());
        }
        let out = out.join(" ");
        let out = out.trim();
        write!(f, "{}", out)
    }
}


impl Hand {
    /// Returns true when both provided Hands are the same variant.
    pub fn is_same_type(previous: &Hand, attempted: &Hand) -> bool {
        matches!(attempted, Hand::Pass) || previous.cards().len() == attempted.cards().len()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hand_order() {
        assert!("2S".parse::<Hand>().unwrap() > "2D".parse::<Hand>().unwrap());
        assert!("2S 2D".parse::<Hand>().unwrap() > "AS AD".parse::<Hand>().unwrap());
        assert!("TS TD TC".parse::<Hand>().unwrap() < "TS TH TC".parse::<Hand>().unwrap());
        assert!("".parse::<Hand>().unwrap() > "2S 2H 2D".parse::<Hand>().unwrap());
    }

}
