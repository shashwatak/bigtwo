mod card;
mod deck;
mod hand;
mod player;
mod test_util;
mod trick;

use trick::GameContinueStatus;

use crate::card::{Card, THREE_OF_CLUBS};
use crate::hand::Hand;
use crate::deck::Deck;
use crate::player::{Player, get_user_input, cards_to_string};
use crate::trick::Trick;

fn main() {
    let mut players = <[Player; 4]>::default();

    let user_submit_hand: fn(&Hand, &Vec<Card>) -> Hand = |hand,cards| {
        println!("Your Turn: {}", cards_to_string(&cards)); 
        println!("Must Beat: {}", hand);
        get_user_input(&mut std::io::stdin().lock())
    };
    
    let user_start_game_or_trick: fn(&Vec<Card>) -> Hand = |cards| {
        println!("Your Turn: {}", cards_to_string(&cards)); 
        get_user_input(&mut std::io::stdin().lock())
    };
    players[0].submit_hand = user_submit_hand;
    players[0].start_game = user_start_game_or_trick;
    players[0].start_trick = user_start_game_or_trick;

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
            GameContinueStatus::NewTrick(new_starting_player_idx) => {
                starting_player_idx = new_starting_player_idx
            }
        }
    };

    println!("Game Over, Player {winner} wins!!");
}

fn deal_cards(players: &mut [Player; 4], mut deck: Deck) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    deck.cards[..].shuffle(&mut rng);

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
            println!("Player {index} has the Three of Clubs and may begin");
            return index;
        }
    }
    unreachable!();
}
