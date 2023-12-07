use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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
    P2(Card, Card),
    P1(Card),
    HC(Card),
}

impl From<&[Card; 5]> for Type {
    fn from(cards: &[Card; 5]) -> Self {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        let mut high_card: Option<Card> = None;
        for card in cards {
            match high_card {
                None => high_card = Some(card.clone()),
                Some(ref leader) => {
                    if card > leader {
                        high_card = Some(card.clone());
                    }
                }
            }
            if let Some(current) = counts.get_mut(&card) {
                *current += 1;
            } else {
                counts.insert(card.clone(), 1);
            }
        }
        let mut ordered_counts: Vec<(usize, Card)> = counts
            .into_iter()
            .map(|(card, count)| (count, card))
            .collect();
        ordered_counts.sort();
        let (top_count, most_frequent_card) = ordered_counts.pop().unwrap();
        match top_count {
            5 => Self::OAK5(most_frequent_card),
            4 => Self::OAK4(most_frequent_card),
            3 => {
                let (second_count, smfc) = ordered_counts.pop().unwrap();
                match second_count {
                    2 => Self::FH(most_frequent_card, smfc),
                    _ => Self::OAK3(most_frequent_card),
                }
            }
            2 => {
                let (n, card) = ordered_counts.pop().unwrap();
                match n {
                    2 => Self::P2(most_frequent_card, card),
                    _ => Self::P1(most_frequent_card),
                }
            }
            _ => Self::HC(high_card.unwrap()),
        }
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
