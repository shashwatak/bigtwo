//! Represents any one of the allowed combinations of cards (known as a "Hand").
//! Cannot be used to represent an unrecognized / nonsensical combination.


pub mod iter;
pub mod try_from;
pub mod order;

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
    /// A Trip and a Pair
    FullHouse(Card, Card, Card, Card, Card),
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

