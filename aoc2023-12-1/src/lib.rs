use itertools::Itertools;
use std::ops::Add;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{space1, u64},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

struct ConditionRecord {
    known: String,
    damage_sizes: Vec<u64>,
}

impl From<String> for ConditionRecord {
    fn from(line: String) -> Self {
        let (known, damage_sizes) = parse_condition_record(line.as_str()).unwrap().1;
        ConditionRecord {
            known: known.to_string(),
            damage_sizes,
        }
    }
}

fn vec_decrement(vec: &Vec<u64>) -> Vec<u64> {
    if vec.len() == 0 {
        return Vec::new();
    }
    let (first, rest) = vec.split_first().unwrap();
    if *first == 1 {
        rest.to_vec()
    } else {
        let mut ret = vec.clone();
        let first = ret.first_mut().unwrap();
        *first -= 1;
        ret
    }
}

impl ConditionRecord {
    fn possible_arrangements(&self) -> PossibleArrangements {
        if self.known.len() < 1 {
            return PossibleArrangements::default();
        }
        // println!("{} {}", &self.known, self.damage_sizes.iter().join(","));
        if self.known == "." && self.damage_sizes.is_empty() {
            return vec!["."].into();
        }
        if self.known == "#" && self.damage_sizes == [1] {
            return vec!["#"].into();
        }
        let (first, rest) = self.known.split_at(1);
        match first {
            "." => {
                return Self::new(rest.to_owned(), self.damage_sizes.clone())
                    .possible_arrangements()
                    .prepend(".");
            }
            "?" => {
                return Self::new(".".to_owned() + rest, self.damage_sizes.clone())
                    .possible_arrangements()
                    + Self::new("#".to_owned() + rest, self.damage_sizes.clone())
                        .possible_arrangements()
            }
            _ => {}
        }
        if self.known.len() < 2 {
            return PossibleArrangements::default();
        }
        let (firs2, rest) = self.known.split_at(2);
        match firs2 {
            "#." => {
                if self.damage_sizes.first() == Some(&1) {
                    return Self::new(".".to_owned() + rest, vec_decrement(&self.damage_sizes))
                        .possible_arrangements()
                        .prepend("#");
                }
            }
            "##" => {
                if let Some(n) = self.damage_sizes.first() {
                    if n > &1 {
                        return Self::new("#".to_owned() + rest, vec_decrement(&self.damage_sizes))
                            .possible_arrangements()
                            .prepend("#");
                    }
                }
            }
            "#?" => {
                return Self::new("##".to_owned() + rest, self.damage_sizes.clone())
                    .possible_arrangements()
                    + Self::new("#.".to_owned() + rest, self.damage_sizes.clone())
                        .possible_arrangements();
            }
            _ => {}
        }
        // println!("---");
        PossibleArrangements::default()
    }

    fn new(known: String, damage_sizes: Vec<u64>) -> Self {
        Self {
            known,
            damage_sizes,
        }
    }
}

fn parse_condition_record(input: &str) -> IResult<&str, (&str, Vec<u64>)> {
    separated_pair(
        take_while1(|c| c == '.' || c == '#' || c == '?'),
        space1,
        separated_list0(tag(","), u64),
    )(input)
}

#[derive(PartialEq, Eq, Debug, Default)]
struct PossibleArrangements(Vec<String>);

impl From<Vec<String>> for PossibleArrangements {
    fn from(arrangements: Vec<String>) -> Self {
        Self(arrangements)
    }
}

impl From<Vec<&str>> for PossibleArrangements {
    fn from(arrangements: Vec<&str>) -> Self {
        Self(arrangements.into_iter().map(String::from).collect())
    }
}

impl Add for PossibleArrangements {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0, rhs.0].concat())
    }
}

impl PossibleArrangements {
    fn prepend(&self, prefix: &str) -> Self {
        Self(self.0.iter().map(|s| prefix.to_owned() + s).collect())
    }

    fn len(&self) -> u64 {
        self.0.len() as u64
    }
}

pub fn sum_possible_arrangements(it: impl Iterator<Item = String>) -> u64 {
    let records: Vec<ConditionRecord> = it.map(ConditionRecord::from).collect();
    records
        .iter()
        .map(|r| r.possible_arrangements().len())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parser() {
        let parsed = parse_condition_record("???.### 1,1,3");
        assert_eq!(parsed, Ok(("", ("???.###", vec![1, 1, 3]))));
    }

    #[test]
    fn example_1() {
        let record = ConditionRecord::from("???.### 1,1,3".to_owned());
        assert_eq!(record.possible_arrangements(), vec!["#.#.###"].into());
    }

    #[test]
    fn example_2() {
        let record = ConditionRecord::from(".??..??...?##. 1,1,3".to_owned());
        assert_eq!(
            record.possible_arrangements(),
            vec![
                "..#...#...###.",
                "..#..#....###.",
                ".#....#...###.",
                ".#...#....###."
            ]
            .into()
        );
    }

    #[test]
    fn example_3() {
        let record = ConditionRecord::from("?#?#?#?#?#?#?#? 1,3,1,6".to_owned());
        assert_eq!(
            record.possible_arrangements(),
            vec![".#.###.#.######"].into()
        );
    }
    #[test]
    fn example_4() {
        let record = ConditionRecord::from("????.#...#... 4,1,1".to_owned());
        assert_eq!(record.possible_arrangements(), vec!["####.#...#..."].into());
    }
    #[test]
    fn example_5() {
        let record = ConditionRecord::from("????.######..#####. 1,6,5".to_owned());
        assert_eq!(
            record.possible_arrangements(),
            vec![
                "...#.######..#####.",
                "..#..######..#####.",
                ".#...######..#####.",
                "#....######..#####."
            ]
            .into()
        );
    }
    #[test]
    fn example_6() {
        let record = ConditionRecord::from("?###???????? 3,2,1".to_owned());
        assert_eq!(
            record.possible_arrangements(),
            vec![
                ".###....##.#",
                ".###...##..#",
                ".###...##.#.",
                ".###..##...#",
                ".###..##..#.",
                ".###..##.#..",
                ".###.##....#",
                ".###.##...#.",
                ".###.##..#..",
                ".###.##.#..."
            ]
            .into()
        );
    }

    #[test]
    fn full_example() {
        let example = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            21
        );
    }
}
