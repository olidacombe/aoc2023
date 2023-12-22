use itertools::Itertools;
use rayon::prelude::*;
use std::{
    iter::{repeat, zip},
    ops::{Add, AddAssign, Mul},
};
use tracing::info;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{space1, u64},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn validate(candidate: &str, filter: &str) -> bool {
    for (c, v) in zip(candidate.chars(), filter.chars()) {
        if v == '?' {
            continue;
        }
        if c != v {
            return false;
        }
    }
    true
}

fn possible_arrangements(damage_sizes: &[usize], filter: &str) -> usize {
    let length = filter.len();
    let k = damage_sizes.len();
    if k == 0 {
        if validate(&str::repeat(".", length), filter) {
            return 1;
        }
        return 0;
    }
    let mandatory_size = damage_sizes.iter().sum::<usize>() + k - 1; // all the # groups plus a .
    let free_dots = length - mandatory_size;

    let (n, damage_sizes) = damage_sizes.split_last().unwrap();

    let mut arrangements = 0;

    let mut midfix = str::repeat("#", *n);
    if !damage_sizes.is_empty() {
        midfix = format!(".{}", midfix);
    }

    for suffix_dots in 0..free_dots + 1 {
        let suffix = format!("{}{}", midfix, str::repeat(".", suffix_dots));
        let suffix_len = suffix.len();
        let (prefix_filter, suffix_filter) = filter.split_at(length - suffix_len);

        if !validate(suffix.as_str(), suffix_filter) {
            continue;
        }

        arrangements += possible_arrangements(damage_sizes, prefix_filter);
    }

    arrangements.into()
}

#[derive(Debug)]
struct ConditionRecord {
    known: String,
    damage_sizes: Vec<usize>,
}

impl From<String> for ConditionRecord {
    fn from(line: String) -> Self {
        let (known, damage_sizes) = parse_condition_record(line.as_str()).unwrap().1;
        ConditionRecord {
            known: known.to_string(),
            damage_sizes: damage_sizes.iter().map(|s| *s as usize).collect(),
        }
    }
}

impl Mul<usize> for ConditionRecord {
    type Output = Self;

    fn mul(self, n: usize) -> Self::Output {
        let Self {
            known,
            damage_sizes,
        } = self;
        let known = repeat(known).take(n).join("?").to_string();
        let damage_sizes = damage_sizes
            .iter()
            .cloned()
            .cycle()
            .take(damage_sizes.len() * n)
            .collect();
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

// TODO replate .append with + :D
impl Add<&str> for PossibleArrangements {
    type Output = Self;

    fn add(self, prefix: &str) -> Self::Output {
        Self(self.0.into_iter().map(|s| s.to_owned() + prefix).collect())
    }
}

impl AddAssign for PossibleArrangements {
    fn add_assign(&mut self, mut rhs: Self) {
        self.0.append(&mut rhs.0);
    }
}

pub fn sum_possible_arrangements(it: impl Iterator<Item = String>) -> usize {
    let records: Vec<ConditionRecord> = it.map(|line| ConditionRecord::from(line) * 5).collect();
    records
        .par_iter()
        .map(|r| {
            info!("{:?}", &r);
            let ret = possible_arrangements(&r.damage_sizes, r.known.as_str());
            println!("{:?} = {}", &r, &ret);
            ret
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

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
            525152
        );
    }
}
