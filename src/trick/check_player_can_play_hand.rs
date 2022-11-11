//! Checks if a specified Player can actually play the Hand they are attempting to play.
use crate::hand::Hand;
use crate::player::Player;

use std::fmt::{Display, Formatter};

/// Represents the different ways a Player's attempted Hand is not playable
#[derive(Debug)]
pub enum PlayHandError {
    /// Attempted Hand must be the same number of cards as previous played Hand.
    NotMatching,

    /// Highest card of attempted Hand must be higher than the highest card of the previously
    /// played Hand.
    TooLow,

    /// Attempted Hand has cards not found in the player's cards.
    StolenCards,
}

impl Display for PlayHandError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::NotMatching => write!(f, "wrong number of cards"),
            Self::TooLow => write!(f, "highest is not high enough"),
            Self::StolenCards => write!(f, "these cards are not in the players hand"),
        }
    }
}

/// Checks if a specified Player can actually play the Hand they are attempting to play.
/// Returns () if the Hand is playable, otherwise returns a specific PlayHandError.
pub fn check_player_can_play_hand(
    current: &Hand,
    player: &Player,
    attempt: &Hand,
) -> Result<(), PlayHandError> {
    if !Hand::is_same_type(current, attempt) {
        return Err(PlayHandError::NotMatching);
    }

    if current > attempt {
        return Err(PlayHandError::TooLow);
    }

    if !player.has_cards(attempt) {
        return Err(PlayHandError::StolenCards);
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::test_util::vec_card_from_str;

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
        assert!(matches!(res, Err(PlayHandError::TooLow)));

        // incorrectly plays a Pair of Fours, reject
        let hand: Hand = "4H 4D".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::NotMatching)));

        // incorrectly plays cards they don't have
        let hand: Hand = "2S".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::StolenCards)));

        // passes
        let hand: Hand = "".parse().unwrap();
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Ok(_)));
    }

    #[test]
    fn test_five_card_hands(){
        let mut player = Player::default();
        let hand_to_beat: Hand = "7D 6H 5C 4H 3D".parse().unwrap();

        // loses
        let cards = vec_card_from_str("7C 6D 5H 4D 3S");
        let hand = Hand::try_from_cards(&cards[..]).unwrap();
        player.cards = cards;
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Err(PlayHandError::TooLow)));

        // wins
        let cards = vec_card_from_str("7S 6D 5H 4D 3S");
        let hand = Hand::try_from_cards(&cards[..]).unwrap();
        player.cards = cards;
        let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
        assert!(matches!(res, Ok(_)));

        // ensure that lones, pairs, and trips cannot play on fivers
        let not_matching_hands = ["2S", "2S 2H", "2S 2H 2D"];
            let cards = vec_card_from_str("2S 2H 2D");
        player.cards = cards;
        for hand in not_matching_hands {
            let hand : Hand = hand.parse().unwrap();
            let res = check_player_can_play_hand(&hand_to_beat, &player, &hand);
            assert!(matches!(res, Err(PlayHandError::NotMatching)));
        }

    }
}
