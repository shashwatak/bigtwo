use std::collections::BTreeSet;

use crate::hand::Hand;
use crate::player::Player;

pub struct Trick {
    pub hand: Hand,
    pub player: Player,
    pub passed_players: BTreeSet<Player>,
}

pub enum InvalidPlayedHand {
    WrongType,
    NotHighEnough,
}

impl Trick {
    pub fn try_play_hand(&mut self, hand: Hand) -> Result<(), InvalidPlayedHand> { 
        Trick::check_hand_playable(&self.hand, &hand)?;
        self.hand = hand;
        if let Some(p) = self.next_player() {
            self.player = p;
        } else {
            unreachable!();
        }
        Ok(())
    }

    fn check_hand_playable(previous: &Hand, attempted: &Hand) -> Result<(), InvalidPlayedHand> {
        Err(InvalidPlayedHand::WrongType)
    }

    fn next_player(&self) -> Option<Player> {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_hand_update() {}
}
