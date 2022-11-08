mod card;
mod constants;
mod deck;
mod hand;
mod player;
mod test_util;
mod trick;

use card::THREE_OF_CLUBS;
use constants::NUM_PLAYERS;
use deck::Deck;
use player::Player;
use trick::{GameContinueStatus, Trick};

fn main() {
    println!("-------------------");
    println!("Welcome to Big Two!");
    println!("Submit hands by typing the cards in e.g. \"3C 3D 3S\"");
    println!("-------------------");
    let mut players = <[Player; NUM_PLAYERS]>::default();
    players[0].convert_to_stdio_user();

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
