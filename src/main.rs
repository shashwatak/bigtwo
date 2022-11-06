mod card;
mod deck;
mod hand;
mod player;
mod test_util;
mod trick;

use trick::GameContinueStatus;

use crate::card::THREE_OF_CLUBS;
use crate::deck::Deck;
use crate::player::Player;
use crate::trick::{Trick, TrickContinueStatus};

fn main() {
    let mut players = <[Player; 4]>::default();
    deal_cards(&mut players, Deck::new());
    for (index, player) in players.iter().enumerate() {
        println!("Player {}: {}", index, player);
    }
    let mut starting_player_idx = find_player_with_three_of_clubs(&players);
    let mut is_first_trick = true;

    let winner: usize = loop {
        let mut trick = Trick::start(starting_player_idx, &mut players, is_first_trick);
        is_first_trick = false;
        let game_status = trick.do_trick(&mut players);
        match game_status {
            GameContinueStatus::GameOver(winner) => break winner,
            GameContinueStatus::NewTrick(last_player) => starting_player_idx = last_player,
        }
    };

    println!("Game Over, Player {winner} wins!!");
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
