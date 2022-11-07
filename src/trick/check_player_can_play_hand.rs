use crate::hand::Hand;
use crate::player::Player;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PlayHandError {
    Match,
    HighEnough,
    PlayerCards,
}

impl Display for PlayHandError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Match => write!(f, "wrong number of cards"),
            Self::HighEnough => write!(f, "highest is not high enough"),
            Self::PlayerCards => write!(f, "these cards are not in the players hand"),
        }
    }
}

pub fn check_player_can_play_hand(
    current: &Hand,
    player: &Player,
    attempt: &Hand,
) -> Result<(), PlayHandError> {

    if !Hand::is_same_type(current, attempt) {
        return Err(PlayHandError::Match);
    }

    if current > attempt {
        return Err(PlayHandError::HighEnough);
    }

    if !player.has_cards(attempt) {
        return Err(PlayHandError::PlayerCards);
    }

    Ok(())

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::tests::vec_card_from_str;

    #[test]
    fn test_check_player_can_play_hand() {
        // new trick begins with a Three of Clubs (ostensibly by player 0),
        let hand_to_beat = "3C".parse().unwrap();

        // player has a few cards
        let cards = vec_card_from_str("3C 3S 4H 4D 4S");
        let mut player = Player::default();
        player.cards = cards;

        // plays a Three of Spades
        let hand: Hand = "3S".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Ok(())));

        // update hand
        let hand_to_beat = hand;

        // incorrectly plays a Three of Diamonds, reject
        let hand: Hand = "3D".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::HighEnough)));

        // incorrectly plays a Pair of Fours, reject
        let hand: Hand = "4H 4D".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::Match)));

        // incorrectly plays cards they don't have
        let hand: Hand = "2S".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::PlayerCards)));

        // passes
        let hand: Hand = "".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Ok(_)));
    }
}
