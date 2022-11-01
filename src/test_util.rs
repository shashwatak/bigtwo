#[cfg(test)]
pub mod tests {

    use crate::card::Card;

    pub fn vec_card_from_str(input: &str) -> Vec<Card> {
        input
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<Card>>()
    }
}
