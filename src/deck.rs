use crate::card::rank::Rank;
use crate::card::suit::Suit;
use crate::card::Card;

#[derive(Debug)]
pub struct Deck {
    // Originally wanted to use a fixed-size array, to keep things on the stack,
    // but this proved difficult because it is not possible to move Cards out of
    // the array, and we want to avoid Copy
    // pub cards: [Card; NUM_CARDS_IN_DECK],
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        let ranks = &Rank::all();
        let suits = &Suit::all();
        for rank in ranks {
            for suit in suits {
                cards.push(Card {
                    rank: *rank,
                    suit: *suit,
                });
            }
        }

        cards.reverse();
        Deck { cards }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeSet;

    use super::*;

    const NUM_CARDS_IN_DECK: usize = 52;

    #[test]
    fn test_new_deck() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), NUM_CARDS_IN_DECK);
        let mut unique_cards: BTreeSet<Card> = BTreeSet::new();
        for card in deck.cards {
            unique_cards.insert(card);
        }
        assert_eq!(unique_cards.len(), NUM_CARDS_IN_DECK);
    }
}
