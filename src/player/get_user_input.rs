
use std::io::{self, Write};
use std::io::BufRead;

use crate::hand::Hand;
use crate::card::Card;

pub fn get_user_input<Input: BufRead>(f: &mut Input) -> Hand {
    loop {
        let mut line = String::new();
        print!("> ");

        io::stdout().flush().unwrap();
        f.read_line(&mut line).unwrap();

        let mut cards = vec![];
        let mut card_errs = vec![];

        let card_strs: Vec<&str> = line.split_whitespace().collect();
        for card_str in card_strs {
            let maybe_card = card_str.to_uppercase().parse::<Card>();
            match maybe_card {
                Err(e) => {
                    println!("error: could not understand {card_str}, {:?}", e);
                    card_errs.push(e);
                }
                Ok(c) => cards.push(c),
            }
        }

        if card_errs.is_empty() {
            cards.sort();
            cards.reverse();
            let maybe_hand = Hand::try_from_cards(&cards);
            if let Ok(hand) = maybe_hand {
                break hand;
            } else {
                println!("error: invalid hand {:?}", maybe_hand.err());
            }
        }
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::card::{rank::Rank, suit::Suit};
    use crate::card::THREE_OF_CLUBS;

    #[test]
    fn test_get_user_input() {
        let mut input = "3C".as_bytes();
        let hand = get_user_input(&mut input);
        assert!(matches!(hand, Hand::Lone(c) if c == THREE_OF_CLUBS));

        const THREE_OF_DIAMONDS: Card = Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        };
        const THREE_OF_SPADES: Card = Card {
            rank: Rank::Three,
            suit: Suit::Spades,
        };

        let mut input = "3C 3S 3D".as_bytes();
        let hand = get_user_input(&mut input);
        assert!(
            matches!(hand, Hand::Trips(a, b, c) if a == THREE_OF_SPADES && b == THREE_OF_DIAMONDS && c == THREE_OF_CLUBS,)
        );

        let mut input = "3G\n3S 4D\n3C 3S 3D".as_bytes();
        let hand = get_user_input(&mut input);
        assert!(
            matches!(hand, Hand::Trips(a, b, c) if a == THREE_OF_SPADES && b == THREE_OF_DIAMONDS && c == THREE_OF_CLUBS,)
        );
    }
}
