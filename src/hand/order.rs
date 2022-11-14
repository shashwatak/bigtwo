//! defines Hand precendece, i.e. which Hand may be played atop which other Hand

use crate::hand::Hand;

/// We want to keep the derived PartialOrd and Ord for Hand, but we cannot
/// use that for the actual game logic as there are many exceptions.
pub fn order(current: &Hand, attempt: &Hand) -> Option<std::cmp::Ordering> {
    // The derived PartialOrd works in many cases, but there are some exceptions:
    //  - Some Hand variants cannot be compared to different variants
    //      - Lones only with Lones, Pairs only with Pairs, and Trips only with Trips
    //  - Some Hand variants cannot be compared by Cards in descending order
    //      - A FullHouse always looks either like:
    //          - AAABB
    //          - AABBB
    //        Though A may have higher Rank, we use the Trip to decide order, which might be B.
    //      - The same is true of FourPlusKick, it is either:
    //          - AAAAB
    //          - ABBBB
    //        Though A may have higher Rank, we use the Quad to decide order, which might be B.

    // if both current and attempt are the same variant of Hand
    if std::mem::discriminant(current) == std::mem::discriminant(attempt) {
        // FullHouses and FourPlusKick cannot be matched using derived PartialOrd
        if matches!(current, Hand::FullHouse(..)) {
            return Some(order_full_house(current, attempt));
        }
    }
    // if both current and attempt are different variants,
    // cannot match Lone, Pair, or Trip with anything but themselves
    else if matches!(current, Hand::Lone(..) | Hand::Pair(..) | Hand::Trips(..))
        || matches!(attempt, Hand::Lone(..) | Hand::Pair(..) | Hand::Trips(..))
    {
        return None;
    }

    // use derived Ord for the rest
    Some(current.cmp(attempt))
}

// Return an Ordering between 2 FullHouses
fn order_full_house(current: &Hand, attempt: &Hand) -> std::cmp::Ordering {
    assert!(matches!(current, Hand::FullHouse(..)));
    assert!(matches!(attempt, Hand::FullHouse(..)));

    let a = get_full_house_trip(current);
    let b = get_full_house_trip(attempt);
    if a != b {
        return a.cmp(&b);
    }
    let a = get_full_house_pair(current);
    let b = get_full_house_pair(attempt);
    a.cmp(&b)
}

/// Copy the three Cards making up the Trip component of the FullHouse, and
/// put them into a Hand::Trip
fn get_full_house_trip(full_house: &Hand) -> Hand {
    assert!(matches!(full_house, Hand::FullHouse(..)));

    // if the first and third cards match, first three cards is the Trip,
    // otherwise its the last three cards
    if full_house[0].rank == full_house[2].rank {
        Hand::try_trips(full_house[0], full_house[1], full_house[2]).unwrap()
    } else {
        Hand::try_trips(full_house[2], full_house[3], full_house[4]).unwrap()
    }
}

/// Copy the two Cards making up the Pair component of the FullHouse, and
/// put them into a Hand::Pair
fn get_full_house_pair(full_house: &Hand) -> Hand {
    assert!(matches!(full_house, Hand::FullHouse(..)));

    // if the first and third cards don't match, then the first two cards are the Pair
    if full_house[0].rank != full_house[2].rank {
        Hand::try_pair(full_house[0], full_house[1]).unwrap()
    } else {
        Hand::try_pair(full_house[3], full_house[4]).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_check_hand_order() {
        // trivial match, Pass equals Pass
        assert!(matches!(
            order(&"".parse().unwrap(), &"".parse().unwrap()),
            Some(Ordering::Equal)
        ));

        // Single Comparisons
        assert!(matches!(
            order(&"3C".parse().unwrap(), &"3D".parse().unwrap()),
            Some(Ordering::Less)
        ));
        assert!(matches!(
            order(&"3D".parse().unwrap(), &"3C".parse().unwrap()),
            Some(Ordering::Greater)
        ));

        // Pair doesn't match Single
        assert!(matches!(
            order(&"4S 4D".parse().unwrap(), &"3D".parse().unwrap()),
            None
        ));

        // Flush beats Straight
        assert!(matches!(
            order(
                &"TC 8C 7C 5C 4C".parse().unwrap(),
                &"2S AS KC QC JS".parse().unwrap()
            ),
            Some(Ordering::Greater)
        ));

        // Flush does not match Trip
        assert!(matches!(
            order(
                &"TC 8C 7C 5C 4C".parse().unwrap(),
                &"2S 2D 2C".parse().unwrap()
            ),
            None
        ));

        // FullHouse Beats Flush
        assert!(matches!(
            order(
                &"TC 8C 7C 5C 4C".parse().unwrap(),
                &"7S 7D 7C 4H 4D".parse().unwrap()
            ),
            Some(Ordering::Less)
        ));

        // FullHouse uses Trips to compare
        assert!(matches!(
            order(
                &"2S 2D 7S 7D 7C".parse().unwrap(),
                &"8S 8D 8C 4H 4D".parse().unwrap()
            ),
            Some(Ordering::Less)
        ));
    }

    #[test]
    fn test_full_house_major() {
        let a: Hand = "8S 8D 8C 4H 4D".parse().unwrap();
        let b: Hand = "2S 2D 7S 7D 7C".parse().unwrap();
        assert!(a > b);
    }
}