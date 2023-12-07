use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Error)]
enum CardError {
    #[error("unrecognized")]
    Unk,
}

impl TryFrom<char> for Card {
    type Error = CardError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(CardError::Unk),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    OAK5(Card),
    OAK4(Card),
    FH(Card, Card),
    OAK3(Card),
    P2(Card),
    P1(Card),
    HC(Card),
}

impl From<&[Card; 5]> for Type {
    fn from(value: &[Card; 5]) -> Self {
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: [Card; 5],
    t: Type,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = value
            .chars()
            .take(5)
            .filter_map(|c| c.try_into().ok())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let t = Type::from(&cards);
        Self { cards, t }
    }
}

pub fn total_winnings(mut it: impl Iterator<Item = String>) -> u64 {
    u64::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        assert_eq!(total_winnings(example.lines().map(String::from)), 6440);
    }
}
