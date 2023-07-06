#![doc = include_str!("../../README.md")]
// use crate::*;
// mod card;
// mod deck;
// mod game;
// mod hand;
// mod player;
// mod trick;

use bigtwo::game::perform_game;

fn main() {
    println!("-------------------");
    println!("Welcome to Big Two!");
    println!("Submit hands by typing the cards in e.g. \"3C 3D 3S\"");
    println!("-------------------");
    perform_game();
}
