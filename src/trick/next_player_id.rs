use std::collections::BTreeSet;

use crate::constants::NUM_PLAYERS;

pub fn next_player_id(current_player_id: usize, passed_player_ids: &BTreeSet<usize>) -> usize {
    assert!(current_player_id < NUM_PLAYERS);
    assert!(
        passed_player_ids.len() < NUM_PLAYERS,
        "all players cannot pass, one player must have not passed"
    );
    for i in 1..NUM_PLAYERS {
        let next_id = (current_player_id + i) % NUM_PLAYERS;
        if !passed_player_ids.contains(&next_id) {
            assert_ne!(current_player_id, next_id);
            return next_id;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_next_player_id() {
        let current: usize = 0;
        let has_passed: BTreeSet<usize> = BTreeSet::new();
        let next = next_player_id(current, &has_passed);
        assert_eq!(next, 1);

        let current: usize = 3;
        let has_passed: BTreeSet<usize> = BTreeSet::new();
        let next = next_player_id(current, &has_passed);
        assert_eq!(next, 0);

        let current: usize = 0;
        let has_passed: BTreeSet<usize> = BTreeSet::from([1, 2]);
        let next = next_player_id(current, &has_passed);
        assert_eq!(next, 3);

        let current: usize = 2;
        let has_passed: BTreeSet<usize> = BTreeSet::from([0, 3]);
        let next = next_player_id(current, &has_passed);
        assert_eq!(next, 1);
    }
}
