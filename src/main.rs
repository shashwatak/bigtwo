mod card;
mod deck;
mod hand;
mod player;
mod test_util;
mod trick;

use trick::GameContinueStatus;

use crate::card::THREE_OF_CLUBS;
use crate::deck::Deck;
use crate::player::{cards_to_string, get_user_input, Player};
use crate::trick::Trick;

fn main() {
    println!("-------------------");
    println!("Welcome to Big Two!");
    println!("Submit hands by typing the cards in e.g. \"3C 3D 3S\"");
    println!("-------------------");
    let mut players = <[Player; 4]>::default();

    players[0].submit_hand = |_, cards| {
        println!("=== Your Turn: {}", cards_to_string(&cards));
        get_user_input(&mut std::io::stdin().lock(), &cards)
    };
    players[0].start_game = |cards| {
        println!(
            "=== Please start the game using the |3C|: {}",
            cards_to_string(&cards)
        );
        get_user_input(&mut std::io::stdin().lock(), &cards)
    };
    players[0].start_trick = |cards| {
        println!("=== You may play any valid hand: {}", cards_to_string(&cards));
        get_user_input(&mut std::io::stdin().lock(), &cards)
    };

    deal_cards(&mut players, Deck::new());

    let mut starting_player_idx = find_player_with_three_of_clubs(&players);
    println!("Player {starting_player_idx} has the Three of Clubs and may begin");
    let mut is_first_trick = true;

    let winner: usize = loop {
        let mut trick = Trick::start(starting_player_idx, &mut players, is_first_trick);
        is_first_trick = false;
        let game_status = trick.do_trick(&mut players);
        match game_status {
            GameContinueStatus::GameOver(winner) => break winner,
            GameContinueStatus::NewTrick(new_starting_player_idx) => {
                starting_player_idx = new_starting_player_idx;
                println!("Player {starting_player_idx} wins the trick (everybody else passed) and starts the next trick");
            }
        }
    };

    println!("Game Over, Player {winner} wins!!");
}

fn deal_cards(players: &mut [Player; 4], mut deck: Deck) {
    println!("Dealing Cards...");
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
            return index;
        }
    }
    unreachable!();
}
