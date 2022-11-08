//! Run the entire Game Loop.

use crate::card::THREE_OF_CLUBS;
use crate::constants::NUM_PLAYERS;
use crate::deck::Deck;
use crate::player::Player;
use crate::trick::{perform_trick, TrickResult};

/// Run the entire Game Loop.
/// 1. Generate 4 Players (3 NPC and 1 PC)
/// 2. Generate a Deck of 52-Standard-Playing-Cards
/// 3. Shuffle the Deck and deal 13 cards to each player
/// 4. Perform Tricks in a loop until a Trick returns GameOver
/// 5. TODO: return Scores.
pub fn perform_game() {
    let mut players = <[Player; NUM_PLAYERS]>::default();
    players[0].convert_to_stdio_user();

    deal_cards(&mut players, Deck::new());

    let mut starting_player_idx = find_player_with_three_of_clubs(&players);
    println!("Player {starting_player_idx} has the Three of Clubs and may begin");
    let mut is_first_trick_of_game = true;

    let winner: usize = loop {
        let trick_result = perform_trick(starting_player_idx, &mut players, is_first_trick_of_game);
        is_first_trick_of_game = false;
        match trick_result {
            TrickResult::GameOver(winner) => break winner,
            TrickResult::NewTrick(new_starting_player_idx) => {
                starting_player_idx = new_starting_player_idx;
                println!("Player {starting_player_idx} wins the trick (everybody else passed) and starts the next trick");
            }
        }
    };

    println!("Game Over, Player {winner} wins!!");
}

fn deal_cards(players: &mut [Player; NUM_PLAYERS], mut deck: Deck) {
    println!("Dealing Cards...");
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    deck.cards[..].shuffle(&mut rng);

    let mut player_index: usize = 0;
    while let Some(card) = deck.cards.pop() {
        let index = player_index % NUM_PLAYERS;
        players[index].cards.push(card);
        player_index += 1;
    }
    for player in players {
        player.cards.sort();
    }
    assert_eq!(deck.cards.len(), 0);
}

fn find_player_with_three_of_clubs(players: &[Player; NUM_PLAYERS]) -> usize {
    for (index, player) in players.iter().enumerate() {
        if player.cards.contains(&THREE_OF_CLUBS) {
            return index;
        }
    }
    unreachable!();
}
