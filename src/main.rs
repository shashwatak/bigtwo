mod card;
mod deck;
mod hand;
mod player;
mod test_util;
mod trick;

use std::collections::BTreeSet;

use crate::card::THREE_OF_CLUBS;
use crate::deck::Deck;
use crate::player::Player;
use crate::trick::Trick;

fn main() {
    let mut players = <[Player; 4]>::default();
    deal_cards(&mut players, Deck::new());
    for (index, player) in players.iter().enumerate() {
        println!("Player {}: {}", index, player);
    }
    let starting_player_idx = find_player_with_three_of_clubs(&players);
    let starting_player = &players[starting_player_idx];
    let starting_hand = (starting_player.start_game)(&starting_player.cards);
    let next_player_idx = Trick::next_player_id(starting_player_idx, &BTreeSet::new());
    let trick = Trick::new(starting_hand, next_player_idx);
    println!("{}", trick);
}

fn deal_cards(players: &mut [Player; 4], mut deck: Deck) {
    // TODO: shuffle the deck :3
    let mut player_index: usize = 0;
    while let Some(card) = deck.cards.pop() {
        let index = player_index % 4;
        players[index].cards.push(card);
        player_index += 1;
    }
    for player in players {
        player.cards.sort();
    }
    assert_eq!(deck.cards.len(), 0);
}

fn find_player_with_three_of_clubs(players: &[Player; 4]) -> usize {
    for (index, player) in players.iter().enumerate() {
        if player.cards.contains(&THREE_OF_CLUBS) {
            return index;
        }
    }
    unreachable!();
}
