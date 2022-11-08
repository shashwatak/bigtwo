//! Utilities that are only useful for making unittest fixtures or assertions.

use crate::card::Card;

/// Useful for making unittest fixtures, expects them to be correct (calls unwrap()).
pub fn vec_card_from_str(input: &str) -> Vec<Card> {
    input
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<Card>>()
}
