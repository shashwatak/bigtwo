#[doc = include_str!("../README.md")]

mod card;
mod constants;
mod deck;
mod game;
mod hand;
mod player;
mod trick;

use game::perform_game;

fn main() {
    println!("-------------------");
    println!("Welcome to Big Two!");
    println!("Submit hands by typing the cards in e.g. \"3C 3D 3S\"");
    println!("-------------------");
    perform_game();
}

#[cfg(test)]
mod tests {
    pub mod test_util;
}
