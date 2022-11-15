//! Implements Index, Iterator, and ExactSizeIterator for Hand

use crate::card::Card;
use crate::hand::Hand;

use std::ops::Index;

impl Index<usize> for Hand {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            // This code is a bit tedious, but it is simple and it allows us to impl the Iterator
            // and ExactSizeIterator traits.
            // NOTE: If we tried to use the Hand Iterator here, it would cause an infinite recursion.
            Hand::Lone(a) if index == 0 => a,
            Hand::Pair(a, b) if index < 2 => [a, b][index],
            Hand::Trips(a, b, c) if index < 3 => [a, b, c][index],
            Hand::Straight(a, b, c, d, e)
            | Hand::Flush(a, b, c, d, e)
            | Hand::FullHouse(a, b, c, d, e)
            | Hand::FourPlusKick(a, b, c, d, e)
                if index < 5 =>
            {
                [a, b, c, d, e][index]
            }
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
            Hand::Straight(..) | Hand::Flush(..) | Hand::FullHouse(..) | Hand::FourPlusKick(..)
                if idx < 5 =>
            {
                Some(&self.hand[idx])
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.hand {
            Hand::Pass => (0, Some(0)),
            Hand::Lone(..) => (1 - self.index, Some(1 - self.index)),
            Hand::Pair(..) => (2 - self.index, Some(2 - self.index)),
            Hand::Trips(..) => (3 - self.index, Some(3 - self.index)),
            _ => (5 - self.index, Some(5 - self.index)),
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
    fn test_index_and_iterator() {
        let hand: Hand = "2S AS KS QS JS".parse().unwrap();
        assert_eq!(hand.cards().len(), 5);
        let cards: Vec<&Card> = Vec::from_iter(hand.cards());
        for i in 0..5 {
            assert_eq!(*cards[i], hand[i]);
        }
    }
}
