use core::fmt;
use std::collections::HashSet;
use std::str::FromStr;

use crate::card::Card;
use crate::card::ParseCardError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Hand {
    Lone([Card; 1]),
    Pair([Card; 2]),
    Trips([Card; 3]),
}

fn join_cards(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string()
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::Lone(cards) => write!(f, "{}", join_cards(cards)),
            Hand::Pair(cards) => write!(f, "{}", join_cards(cards)),
            Hand::Trips(cards) => write!(f, "{}", join_cards(cards)),
        }
    }
}

#[derive(Debug)]
pub enum ParseHandError {
    Empty,
    BadLen,
    BadCard(ParseCardError),
    DuplicateCard,
    NotSortedDescending,
    InvalidHand(InvalidHandError),
}

impl From<ParseCardError> for ParseHandError {
    fn from(e: ParseCardError) -> Self {
        Self::BadCard(e)
    }
}

#[derive(Debug)]
pub enum InvalidHandError {
    UnmatchedPair,
    UnmatchedTrips,
}

impl From<InvalidHandError> for ParseHandError {
    fn from(e: InvalidHandError) -> Self {
        Self::InvalidHand(e)
    }
}

fn try_cards(maybe_cards: &[&str]) -> Result<Vec<Card>, ParseHandError> { 
    let mut cards : Vec<Card> = vec![];
    for maybe_card in maybe_cards {
        cards.push(maybe_card.parse()?);
    }

    let unique_cards: HashSet<Card> = HashSet::from_iter(cards.iter().cloned());
    if unique_cards.len() < cards.len() {
        return Err(ParseHandError::DuplicateCard);
    }
    for (i, card) in cards.iter().enumerate() {
        if i == 0 { continue; }
        if cards[i-1] < *card {
            return Err(ParseHandError::NotSortedDescending);
        }
    }
    Ok(cards)
}
    // let cards : [Card; 2] = [maybe_cards[0].parse()?, maybe_cards[1].parse()?];
fn try_pair(cards: [Card; 2]) -> Result<Hand, InvalidHandError> {
    if cards[0].number == cards[1].number {
        Ok(Hand::Pair(cards))
    } else {
        Err(InvalidHandError::UnmatchedPair)
    }
}

fn try_trips(cards: [Card; 3]) -> Result<Hand, InvalidHandError> {
    if cards[0].number == cards[1].number && cards[0].number == cards[2].number {
        Ok(Hand::Trips(cards))
    } else {
        Err(InvalidHandError::UnmatchedTrips)
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(hand_str: &str) -> Result<Self, Self::Err> {
        let hand_str = hand_str.trim();
        if let "" = hand_str {
            return Err(Self::Err::Empty);
        }
        let splits = hand_str.split(' ').collect::<Vec<&str>>();
        let cards = try_cards(&splits[..])?;
        match cards[..] {
            [a] => Ok(Hand::Lone([a])),
            [a, b] => Ok(try_pair([a, b])?),
            [a, b, c] => Ok(try_trips([a, b, c])?),
            _ => Err(Self::Err::BadLen),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_hand_to_from_string() {
        {
            let hand = "".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::Empty)));
        }
        {
            let hand = "AJ".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::BadCard(_))));
        }
        {
            let hand = "7D 5C 4C 3C".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::BadLen)));
        }
        {
            let hand = "3C 4D".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::NotSortedDescending)));
        }
        {
            let hand = "7D 3C 4C 5C 3C".to_string().parse::<Hand>();
            assert!(matches!(hand, Err(ParseHandError::DuplicateCard)));
        }
        {
            let hand = "2D 3S".to_string().parse::<Hand>();
            assert!(matches!(
                hand,
                Err(ParseHandError::InvalidHand(InvalidHandError::UnmatchedPair))
            ));
        }
        {
            let hand = "2S 2H 3S".to_string().parse::<Hand>();
            assert!(matches!(
                hand,
                Err(ParseHandError::InvalidHand(
                    InvalidHandError::UnmatchedTrips
                ))
            ));
        }
    }

    #[test]
    fn test_good_hand_to_from_string() {
        let good_hands = ["2S", "3D 3C", "KS KH KC"];
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
    }
}
