use crate::card::number::Number;
use crate::card::suit::Suit;
use crate::card::Card;

const NUM_CARDS_IN_DECK: usize = 52;

struct Deck {
    pub cards: [Card; NUM_CARDS_IN_DECK],
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: [Card; NUM_CARDS_IN_DECK] = ["3C".parse().unwrap(); NUM_CARDS_IN_DECK];
        let numbers = Number::all();
        let suits = Suit::all();
        for i in 0..NUM_CARDS_IN_DECK {
            cards[i].number = numbers[i / 4];
            cards[i].suit = suits[i % 4];
        }
        Deck { cards }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test_new_deck() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), NUM_CARDS_IN_DECK);
        let unique_cards : BTreeSet<Card> = BTreeSet::from(deck.cards);
        assert_eq!(unique_cards.len(), NUM_CARDS_IN_DECK);
    }
}
