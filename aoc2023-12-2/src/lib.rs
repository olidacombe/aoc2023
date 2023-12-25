use itertools::Itertools;
use num_integer::binomial;
use std::{
    iter::{repeat, zip},
    ops::{Add, AddAssign, Mul},
};
use tracing::{debug, info, trace};

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{space1, u64},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn validate_old(candidate: &str, filter: &str) -> bool {
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

fn validate_damage_sizes(damage_sizes: &[usize], candidates: &[usize]) -> bool {
    if candidates.iter().sum::<usize>() > damage_sizes.iter().sum() {
        return false;
    }

    for (candi, damage) in zip(
        candidates.iter().sorted().rev(),
        damage_sizes.iter().sorted().rev(),
    ) {
        if candi > damage {
            return false;
        }
    }

    true
}

fn hash_sizes(s: &str) -> Vec<usize> {
    let mut hash_size = 0;
    let mut hash_sizes = Vec::new();

    for c in s.chars() {
        match c {
            '#' => {
                hash_size += 1;
            }
            _ => {
                hash_sizes.push(hash_size);
                hash_size = 0;
            }
        }
    }
    if hash_size > 0 {
        hash_sizes.push(hash_size);
    }

    hash_sizes
}

fn num_hashes(s: &str) -> (usize, usize, usize, usize) {
    let mut num = 0;
    let mut max_counter = 0;
    let mut max = 0;
    let mut end = 0;
    let mut start_tracker = 0;
    let mut start = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '#' => {
                if max_counter == 0 {
                    start_tracker = i;
                }
                num += 1;
                max_counter += 1;
                if max_counter > max {
                    start = start_tracker;
                    max = max_counter;
                }
            }
            _ => {
                if max_counter > 0 {
                    end = i;
                    max_counter = 0;
                }
            }
        }
    }
    if end < start {
        end = s.len();
    }
    (num, max, start, end)
}

fn num_qs(s: &str) -> usize {
    s.chars().filter(|c| *c == '?').count()
}

fn split_at_middle_dot(s: &str) -> Option<(&str, &str)> {
    let n = s.len() / 2;
    let (l, r) = s.split_at(n);
    if r.starts_with(".") {
        return Some((l, r.split_at(1).1));
    }
    for i in 1..n + 1 {
        let (l, r) = s.split_at(n + i);
        if r.starts_with(".") {
            return Some((l, r.split_at(1).1));
        }
        let (l, r) = s.split_at(n - i);
        if r.starts_with(".") {
            return Some((l, r.split_at(1).1));
        }
    }
    None
}

fn possible_arrangements(damage_sizes: &[usize], filter: &str) -> usize {
    trace!(filter);
    // Plan:
    //
    // Given S, D = (n_1, ..., n_k)
    //
    // 1. Split at "middle-most" '.' char S -> (L, R)
    //  1. Sum f(L, p) * f(R, q) for all partitions p|q of D
    // 1. When no '.' found:
    //  1. If count(?) < k-1 return 0
    //  1. If count(#) > sum(D) return 0
    //  1. Else go brute as below
    //
    //  Maybe a cache too...?

    // Divide on "middlemost" '.'
    if let Some((filter_left, filter_right)) = split_at_middle_dot(filter) {
        let mut arrangements = 0;
        for i in 0..damage_sizes.len() + 1 {
            let (damage_left, damage_right) = damage_sizes.split_at(i);
            arrangements += possible_arrangements(damage_left, filter_left)
                * possible_arrangements(damage_right, filter_right);
        }
        return arrangements;
    }

    // Get damage sizes as found in the filter
    let filter_damage_sizes = hash_sizes(filter);
    if filter_damage_sizes == damage_sizes {
        // we have found an exact match, if we make all ?s into .s
        return 1;
    }

    if !validate_damage_sizes(damage_sizes, &filter_damage_sizes) {
        return 0;
    }

    // No '.' found
    let num_qs = num_qs(filter);
    if num_qs == 0 {
        return 0;
    }

    let k = damage_sizes.len();
    let (num_hashes, max_hashes, max_hash_start, max_hash_end) = num_hashes(filter);
    let total_damage = damage_sizes.iter().sum::<usize>();

    let max_damage = damage_sizes.iter().max().unwrap_or(&0);

    if max_hashes > *max_damage {
        return 0;
    }

    if total_damage == 0 && num_hashes == 0 {
        return 1;
    }
    if num_hashes == total_damage {
        return 1;
    }
    if num_hashes > total_damage {
        return 0;
    }
    if num_qs < k - 1 {
        return 0;
    }

    let length = filter.len();
    let mandatory_size = total_damage + k - 1; // all the # groups plus a . between
    if mandatory_size > length {
        return 0;
    }
    let free_dots = length - mandatory_size;

    // // We are long enough and all '?'
    if num_hashes == 0 {
        return binomial(k + free_dots, free_dots);
    }

    // if max_hash_end < length {
    //     let (left, right) = filter.split_at(max_hash_end);
    //     let end = right.split_at(1).1;
    //     debug!("Splitting {filter} -> {left}[?]{end}");
    //     return possible_arrangements(damage_sizes, (left.to_string() + "." + end).as_str())
    //         + possible_arrangements(damage_sizes, (left.to_string() + "#" + end).as_str());
    // } else {
    //     let (left, right) = filter.split_at(max_hash_start - 1);
    //     let end = right.split_at(1).1;
    //     debug!("Splitting {filter} -> {left}[?]{end}");
    //     return possible_arrangements(damage_sizes, (left.to_string() + "." + end).as_str())
    //         + possible_arrangements(damage_sizes, (left.to_string() + "#" + end).as_str());
    // }

    // if let Some(i) = filter.find("?#") {
    //     let (l, r) = filter.split_at(i);
    //     let (_, r) = r.split_at(2);
    //     let option_1 = l.to_string() + ".#" + r;
    //     let option_2 = l.to_string() + "##" + r;
    //     return possible_arrangements(damage_sizes, option_1.as_str())
    //         + possible_arrangements(damage_sizes, option_2.as_str());
    // } else if let Some(i) = filter.find("#?") {
    //     let (l, r) = filter.split_at(i);
    //     let (_, r) = r.split_at(2);
    //     let option_1 = l.to_string() + "#." + r;
    //     let option_2 = l.to_string() + "##" + r;
    //     return possible_arrangements(damage_sizes, option_1.as_str())
    //         + possible_arrangements(damage_sizes, option_2.as_str());
    // }

    // Brute force

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

        if !validate_old(suffix.as_str(), suffix_filter) {
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
        // .par_iter()
        .iter()
        .enumerate()
        .map(|(i, r)| {
            info!("{i}: {:?}", &r);
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
    fn example_1() {
        let example = indoc! {"
            ???.### 1,1,3
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            1
        );
    }

    #[test]
    fn example_2() {
        let example = indoc! {"
            .??..??...?##. 1,1,3
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            16384
        );
    }
    #[test]
    fn example_3() {
        let example = indoc! {"
            ?#?#?#?#?#?#?#? 1,3,1,6
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            1
        );
    }
    #[test]
    fn example_4() {
        let example = indoc! {"
            ????.#...#... 4,1,1
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            16
        );
    }
    #[test]
    fn example_5() {
        let example = indoc! {"
            ????.######..#####. 1,6,5
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            2500
        );
    }
    #[test]
    fn example_6() {
        let example = indoc! {"
            ?###???????? 3,2,1
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            506250
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
            525152
        );
    }

    #[test]
    fn num_hashes_basic() {
        assert_eq!(num_hashes("#??###??#"), (5, 3, 3, 6));
        assert_eq!(num_hashes("#?????##"), (3, 2, 6, 8));
    }

    #[test]
    fn middle_dot_odd_even_lengths() {
        assert_eq!(split_at_middle_dot(".abc"), Some(("", "abc")));
        assert_eq!(split_at_middle_dot("abc."), Some(("abc", "")));
        assert_eq!(split_at_middle_dot(".abcd"), Some(("", "abcd")));
        assert_eq!(split_at_middle_dot("abcd."), Some(("abcd", "")));
        assert_eq!(split_at_middle_dot("ab.cd"), Some(("ab", "cd")));
        assert_eq!(split_at_middle_dot("abc.de"), Some(("abc", "de")));
        assert_eq!(split_at_middle_dot("ab.cde"), Some(("ab", "cde")));
        assert_eq!(split_at_middle_dot("abc"), None);
        assert_eq!(split_at_middle_dot("abcd"), None);
    }
}
