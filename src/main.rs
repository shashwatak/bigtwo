mod card;
mod deck;
mod hand;
mod player;
mod test_util;
mod trick;

use std::collections::BTreeSet;

use trick::StepStatus;

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
    let mut starting_player_idx = find_player_with_three_of_clubs(&players);
    let mut is_first_trick = true;
    loop { 
        let mut trick = Trick::start(starting_player_idx, &mut players, is_first_trick).unwrap();
        is_first_trick = false;
        loop {
            println!("{}", trick);
            println!("{}", players[trick.current_player_id]);
            let trick_step = trick.step(&mut players);
            if let StepStatus::TrickOver(p) = trick_step {
                println!("Trick Over, Player {p} gets to start next trick");
                break;
            } else if let StepStatus::GameOver(p) = trick_step {
                println!("Game Over, Player {p} wins!!");
                return;
            }
        }
    }
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
