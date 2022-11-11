//! Represents any one of the allowed combinations of cards (known as a "Hand").
//! Cannot be used to represent an unrecognized / nonsensical combination.

use core::fmt;
use std::collections::BTreeSet;
use std::ops::Index;
use std::str::FromStr;

use crate::card::Card;
use crate::card::ParseCardError;

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

/// Represents the possible ways that a string can fail to parse into a reasonable Hand.
#[derive(Debug)]
pub enum ParseHandError {
    /// Not able to parse one of the Cards in this string.
    BadCard(ParseCardError),
    /// Playing with a single deck, only one of each card allowed.
    DuplicateCard,
    /// For programmer convenience, must provide Cards in descending order>
    NotSortedDescending,
    /// All cards were parsed, in the correct order, but the result is not a valid Hand.
    InvalidHand(InvalidHandError),
}

impl From<ParseCardError> for ParseHandError {
    fn from(e: ParseCardError) -> Self {
        Self::BadCard(e)
    }
}

/// Represents the ways valid Cards can fail to combine into a valid Hand.
/// TODO: represent invalid 5 card hands
/// TODO 2: this might actually be kinda redundant, could just remove it?
#[derive(Debug)]
pub enum InvalidHandError {
    /// Two cards of different Rank
    UnmatchedPair,
    /// Three cards, at least one is a different Rank
    UnmatchedTrips,
    /// Incorrect number of cards (0, 4, 6+)
    WrongQuantity,
}

impl From<InvalidHandError> for ParseHandError {
    fn from(e: InvalidHandError) -> Self {
        Self::InvalidHand(e)
    }
}

impl Hand {
    /// Given a slice of Cards, either return a Hand, or an Error
    pub fn try_from_cards(cards: &[Card]) -> Result<Hand, ParseHandError> {
        match cards {
            [] => Ok(Hand::Pass),
            [a] => Ok(Hand::Lone(*a)),
            [a, b] => Ok(Hand::try_pair(*a, *b)?),
            [a, b, c] => Ok(Hand::try_trips(*a, *b, *c)?),
            _ => Err(ParseHandError::InvalidHand(InvalidHandError::WrongQuantity)),
        }
    }

    /// Given two cards, return a Pair or an Error
    pub fn try_pair(first: Card, second: Card) -> Result<Hand, InvalidHandError> {
        assert!(second < first);
        if first.rank == second.rank {
            Ok(Hand::Pair(first, second))
        } else {
            Err(InvalidHandError::UnmatchedPair)
        }
    }

    /// Given three cards, return a Trip or an Error
    pub fn try_trips(first: Card, second: Card, third: Card) -> Result<Hand, InvalidHandError> {
        assert!(third < second);
        assert!(second < first);
        if first.rank == second.rank && second.rank == third.rank {
            Ok(Hand::Trips(first, second, third))
        } else {
            Err(InvalidHandError::UnmatchedTrips)
        }
    }

    /// Return an Error if this slice of Cards is incoherent.
    pub fn sanitize_cards(cards: &[Card]) -> Result<(), ParseHandError> {
        let mut unique_cards: BTreeSet<&Card> = BTreeSet::new();
        for card in cards {
            unique_cards.insert(card);
        }
        if unique_cards.len() < cards.len() {
            return Err(ParseHandError::DuplicateCard);
        }

        for (i, card) in cards.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if cards[i - 1] < *card {
                return Err(ParseHandError::NotSortedDescending);
            }
        }

        Ok(())
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(hand_str: &str) -> Result<Hand, Self::Err> {
        let hand_str = hand_str.trim();
        if hand_str.is_empty() {
            return Ok(Hand::Pass);
        }
        let maybe_cards = hand_str.split(' ').collect::<Vec<&str>>();
        let mut cards: Vec<Card> = vec![];
        for maybe_card in maybe_cards {
            cards.push(maybe_card.parse()?);
        }

        Self::sanitize_cards(&cards[..])?;

        Self::try_from_cards(&cards)
    }
}

impl Hand {
    /// Returns true when both provided Hands are the same variant.
    pub fn is_same_type(previous: &Hand, attempted: &Hand) -> bool {
        matches!(
            (previous, attempted),
            (_, Hand::Pass)
                | (Hand::Lone(_), Hand::Lone(_))
                | (Hand::Pair(_, _), Hand::Pair(_, _))
                | (Hand::Trips(_, _, _), Hand::Trips(_, _, _))
        )
        matches!(attempted, Hand::Pass) || previous.cards().len() == attempted.cards().len()
    }
}

impl Index<usize> for Hand {
    type Output = Card;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Hand::Lone(a) if index == 0 => a,
            Hand::Pair(a, b) if index < 2 => [a, b][index],
            Hand::Trips(a, b, c) if index < 3 => [a, b, c][index],
            _ => panic!("index {index} is out of bounds!"),
        }
    }
}

pub struct HandIterator<'a> {
    index: usize,
    hand: &'a Hand,
}

impl<'a> HandIterator<'a> {
    fn new(hand: &'a Hand) -> Self {
        HandIterator { index: 0, hand }
    }
}

impl<'a> Iterator for HandIterator<'a> {
    type Item = &'a Card;
    fn next(&mut self) -> Option<Self::Item> {
        // Uses the Index trait impl for Hand
        let idx = self.index;
        self.index += 1;
        match self.hand {
            Hand::Lone(..) if idx == 0 => Some(&self.hand[idx]),
            Hand::Pair(..) if idx < 2 => Some(&self.hand[idx]),
            Hand::Trips(..) if idx < 3 => Some(&self.hand[idx]),
            _ => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.hand {
            Hand::Pass => (0, Some(0)),
            Hand::Lone(..) => (1 - self.index, Some(1 - self.index)),
            Hand::Pair(..) => (2 - self.index, Some(2 - self.index)),
            Hand::Trips(..) => (3 - self.index, Some(3 - self.index)),
        }
    }
}

impl<'a> ExactSizeIterator for HandIterator<'a> {}

impl Hand {
    pub fn cards(&self) -> HandIterator {
        HandIterator::new(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_hand_to_from_string() {
        let hand = "AJ".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::BadCard(_))));

        let hand = "7D 5C 4C 3".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::BadCard(_))));

        let hand = "7D 5C 4C 3C".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(InvalidHandError::WrongQuantity))
        ));

        let hand = "3C 4D".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::NotSortedDescending)));

        let hand = "7D 3C 4C 5C 3C".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::DuplicateCard)));

        let hand = "2D 3S".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(InvalidHandError::UnmatchedPair))
        ));

        let hand = "2S 2H 3S".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(
                InvalidHandError::UnmatchedTrips
            ))
        ));
    }

    #[test]
    fn test_good_hand_to_from_string() {
        let good_hands = ["", "2S", "3D 3C", "KS KH KC"];
        for expected_hand in good_hands {
            let hand = expected_hand.to_string().parse::<Hand>();
            println!("{:?}", hand);
            assert!(matches!(hand, Ok(_)));
            let result_hand = hand.unwrap().to_string();
            assert_eq!(expected_hand, result_hand);
        }
    }

    #[test]
    fn test_hand_order() {
        assert!("2S".parse::<Hand>().unwrap() > "2D".parse::<Hand>().unwrap());
        assert!("2S 2D".parse::<Hand>().unwrap() > "AS AD".parse::<Hand>().unwrap());
        assert!("TS TD TC".parse::<Hand>().unwrap() < "TS TH TC".parse::<Hand>().unwrap());
        assert!("".parse::<Hand>().unwrap() > "2S 2H 2D".parse::<Hand>().unwrap());
    }

    use crate::card::rank::Rank;
    use crate::card::suit::Suit;

    #[test]
    fn test_hand_index_iterator() {
        let hand: Hand = "2S 2H 2D".parse().unwrap();
        assert_eq!(
            hand[0],
            Card {
                rank: Rank::Two,
                suit: Suit::Spades
            }
        );
        assert_eq!(
            hand[1],
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts
            }
        );
        assert_eq!(
            hand[2],
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds
            }
        );

        for (index, card) in hand.cards().enumerate() {
            assert_eq!(hand[index], *card);
        }
    }
}
