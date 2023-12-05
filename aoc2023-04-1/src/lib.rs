use std::{collections::HashSet, sync::OnceLock};

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Card {
    winning: Vec<u32>,
    in_hand: Vec<u32>,
}

impl Card {
    pub fn score(&self) -> u32 {
        static BASE: u32 = 2;
        let winning_set = HashSet::<&u32>::from_iter(self.winning.iter());
        let in_hand_set = HashSet::<&u32>::from_iter(self.in_hand.iter());
        let num_winning_numbers = winning_set.intersection(&in_hand_set).count();
        match num_winning_numbers {
            0 => 0,
            n => BASE.pow((n - 1).try_into().unwrap()),
        }
    }
}

fn numbers(input: &str) -> Vec<u32> {
    static NUMBER: OnceLock<Regex> = OnceLock::new();
    NUMBER
        .get_or_init(|| Regex::new(r"\d+").unwrap())
        .find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        static SPLIT: OnceLock<Regex> = OnceLock::new();
        let captures = SPLIT
            .get_or_init(|| Regex::new(r":(?<winning>.*)\|(?<hand>.*)$").unwrap())
            .captures(value)
            .unwrap();
        let winning = captures.name("winning").unwrap().as_str();
        let hand = captures.name("hand").unwrap().as_str();
        Self {
            winning: numbers(winning),
            in_hand: numbers(hand),
        }
    }
}

pub fn sum_points(it: impl Iterator<Item = String>) -> u32 {
    it.map(|line| Card::from(line.as_str()).score()).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn game_deserialize() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            card,
            Card {
                winning: vec![41, 48, 83, 86, 17],
                in_hand: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn full_example() {
        let example = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        assert_eq!(sum_points(example.lines().map(String::from)), 13);
    }
}
