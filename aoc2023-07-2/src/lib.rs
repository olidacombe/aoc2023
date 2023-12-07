use std::{collections::HashMap, sync::OnceLock};

use regex::Regex;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
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
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            'J' => Ok(Card::Joker),
            _ => Err(CardError::Unk),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    OAK5,
    OAK4,
    FH,
    OAK3,
    P2,
    P1,
    HC,
}

impl From<&[Card; 5]> for Type {
    fn from(cards: &[Card; 5]) -> Self {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        let mut high_card: Option<Card> = None;
        let mut joker_count = 0;
        for card in cards {
            match high_card {
                None => high_card = Some(card.clone()),
                Some(ref leader) => {
                    if card > leader {
                        high_card = Some(card.clone());
                    }
                }
            }
            if card == &Card::Joker {
                joker_count += 1;
                continue;
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
        let (mut top_count, _) = ordered_counts.pop().unwrap();
        top_count += joker_count;
        match top_count {
            5 => Self::OAK5,
            4 => Self::OAK4,
            3 => {
                let (second_count, _) = ordered_counts.pop().unwrap();
                match second_count {
                    2 => Self::FH,
                    _ => Self::OAK3,
                }
            }
            2 => {
                let (n, _) = ordered_counts.pop().unwrap();
                match n {
                    2 => Self::P2,
                    _ => Self::P1,
                }
            }
            _ => Self::HC,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    t: Type,
    cards: [Card; 5],
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Prospect {
    hand: Hand,
    bid: u64,
}

impl From<&str> for Prospect {
    fn from(value: &str) -> Self {
        static HAND_BID: OnceLock<Regex> = OnceLock::new();
        let (_, [hand, bid]) = HAND_BID
            .get_or_init(|| Regex::new(r"(\w+)\s+(\d+)").unwrap())
            .captures(value)
            .unwrap()
            .extract();
        let hand = Hand::from(hand);
        let bid = bid.parse().unwrap();
        Self { hand, bid }
    }
}

pub fn total_winnings(it: impl Iterator<Item = String>) -> u64 {
    let mut prospects: Vec<Prospect> = it.map(|line| Prospect::from(line.as_str())).collect();
    prospects.sort();
    prospects
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, prospect)| (idx as u64 + 1) * prospect.bid)
        .sum()
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
        assert_eq!(total_winnings(example.lines().map(String::from)), 5905);
    }
}
