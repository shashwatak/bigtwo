//! Represents any one of the allowed combinations of cards (known as a "Hand").
//! Cannot be used to represent an unrecognized / nonsensical combination.

pub mod iter;
pub mod order;
pub mod try_from;

use core::fmt;

use crate::card::Card;

/// Represents any one of the allowed combinations of cards (known as a "Hand").
/// Cannot be used to represent an unrecognized / nonsensical combination.
/// TODO: Fivers: Straight, Flush, FullHouse, FourPlusKicker, StraightFlush
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hand {
    /// No Hand, No Cards
    Pass,
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
    /// All Four of one Rank, plus any other card
    FourPlusKick(Card, Card, Card, Card, Card),
    /// 5 Cards of consecutive Rank AND of the same Suit
    StraightFlush(Card, Card, Card, Card, Card),
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
