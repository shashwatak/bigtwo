mod card;
mod deck;
mod hand;
mod player;
mod test_util;
mod trick;

use crate::deck::Deck;
use crate::hand::Hand;
use crate::player::PassingPlayer;
use crate::trick::Trick;

fn main() {
    let deck = Deck::new();
    let players = [PassingPlayer::default; 4];
    let starting_player: usize = 0;
    let starting_hand: Hand = "3C".parse().unwrap();
    let trick = Trick::new(starting_hand, starting_player);
    println!("{}", trick);
}
