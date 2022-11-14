//! defines Hand precendece, i.e. which Hand may be played atop which other Hand

use crate::hand::Hand;

/// We want to keep the derived PartialOrd and Ord for Hand, but we cannot
/// use that for the actual game logic as there are many exceptions.
pub fn order(current: &Hand, attempt: &Hand) -> Option<std::cmp::Ordering> {
    // TODO first catch the FullHouse and FourOfAKind because they are weird exception.

    // Hands that are the same variation can be compared using derived Ord
    // Fivers of different variations can be compared using derived Ord
    if std::mem::discriminant(current) == std::mem::discriminant(attempt)
        || matches!(
            (current, attempt),
            (
                Hand::Straight(..) | Hand::Flush(..),
                Hand::Straight(..) | Hand::Flush(..)
            )
        )
    {
        Some(current.cmp(attempt))
    } else {
        // Lones, Pairs, and Trips can only be compared to like
        // Fivers can only be compared to other Fivers
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_check_hand_order() {
        assert!(matches!(
            order(&"".parse().unwrap(), &"".parse().unwrap()),
            Some(Ordering::Equal)
        ));

        assert!(matches!(
            order(&"3C".parse().unwrap(), &"3D".parse().unwrap()),
            Some(Ordering::Less)
        ));

        assert!(matches!(
            order(&"3D".parse().unwrap(), &"3C".parse().unwrap()),
            Some(Ordering::Greater)
        ));

        assert!(matches!(
            order(&"4S 4D".parse().unwrap(), &"3D".parse().unwrap()),
            None
        ));

        assert!(matches!(
            order(&"TC 8C 7C 5C 4C".parse().unwrap(), &"2S AS KC QC JS".parse().unwrap()), 
            Some(Ordering::Greater)
        ));

    }
}
