use itertools::Itertools;
use num_integer::binomial;
use std::{
    iter::{repeat, zip},
    ops::{Add, AddAssign, Mul},
};
use tracing::{info, trace};

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{space1, u64},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn validate_damage_sizes(damage_sizes: &[usize], candidates: &[usize]) -> bool {
    if candidates.iter().sum::<usize>() > damage_sizes.iter().sum() {
        trace!("{candidates:?} > {damage_sizes:?} => fail");
        return false;
    }

    for (candi, damage) in zip(
        candidates.iter().sorted().rev(),
        damage_sizes.iter().sorted().rev(),
    ) {
        if candi > damage {
            trace!("{candidates:?} > {damage_sizes:?} => fail");
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
                if hash_size > 0 {
                    hash_sizes.push(hash_size);
                }
                hash_size = 0;
            }
        }
    }
    if hash_size > 0 {
        hash_sizes.push(hash_size);
    }

    hash_sizes
}

fn num_qs(s: &str) -> usize {
    s.chars().filter(|c| *c == '?').count()
}

fn split_at_middle_match<const C: char>(s: &str) -> Option<(&str, &str)> {
    let n = s.len() / 2;
    let (l, r) = s.split_at(n);
    if r.starts_with(C) {
        return Some((l, r.split_at(1).1));
    }
    for i in 1..n + 1 {
        let (l, r) = s.split_at(n + i);
        if r.starts_with(C) {
            return Some((l, r.split_at(1).1));
        }
        let (l, r) = s.split_at(n - i);
        if r.starts_with(C) {
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

    // Get damage sizes as found in the filter
    let filter_damage_sizes = hash_sizes(filter);
    if filter_damage_sizes == damage_sizes {
        trace!("{filter} : {filter_damage_sizes:?} = {damage_sizes:?} => 1");
        // we have found an exact match, if we make all ?s into .s
        return 1;
    }

    if !validate_damage_sizes(damage_sizes, &filter_damage_sizes) {
        trace!("{filter} : {filter_damage_sizes:?} X {damage_sizes:?} => 0");
        return 0;
    }

    let k = damage_sizes.len();
    let num_hashes = filter_damage_sizes.iter().sum::<usize>();
    let total_damage = damage_sizes.iter().sum::<usize>();
    let length = filter.len();
    let mandatory_size = total_damage + k - 1; // all the # groups plus a . between
    if mandatory_size > length {
        trace!("{filter} {damage_sizes:?} mandatory {mandatory_size} > {length} => 0");
        return 0;
    }

    // Divide on "middlemost" '.'
    if let Some((filter_left, filter_right)) = split_at_middle_match::<'.'>(filter) {
        let mut arrangements = 0;
        for i in 0..damage_sizes.len() + 1 {
            let (damage_left, damage_right) = damage_sizes.split_at(i);
            arrangements += possible_arrangements(damage_left, filter_left)
                * possible_arrangements(damage_right, filter_right);
        }
        return arrangements;
    }

    // No '.' found
    // So if we're not working with the right number of ?s we are done
    let num_qs = num_qs(filter);
    if num_qs < k - 1 {
        trace!("{filter} insufficient qs for {damage_sizes:?}");
        return 0;
    }

    let free_dots = length - mandatory_size;

    // We are long enough and all '?'
    if num_hashes == 0 {
        let combinatorial_arrangements = binomial(k + free_dots, free_dots);
        trace!("{filter} * {damage_sizes:?} => {combinatorial_arrangements}");
        return combinatorial_arrangements;
    }

    // Divide on "middlemost" '?'
    if let Some((filter_left, filter_right)) = split_at_middle_match::<'?'>(filter) {
        return possible_arrangements(
            damage_sizes,
            (filter_left.to_string() + "." + filter_right).as_str(),
        ) + possible_arrangements(
            damage_sizes,
            (filter_left.to_string() + "#" + filter_right).as_str(),
        );
    }

    unreachable!("How did we get here?");
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
    fn example_line_12() {
        let example = indoc! {"
            ?.###??????#???.?.?? 11,1
        "};
        assert_eq!(
            sum_possible_arrangements(example.lines().map(String::from)),
            // 4 // 1 round
            5184
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
    fn middle_dot_odd_even_lengths() {
        assert_eq!(split_at_middle_match::<'.'>(".abc"), Some(("", "abc")));
        assert_eq!(split_at_middle_match::<'.'>("abc."), Some(("abc", "")));
        assert_eq!(split_at_middle_match::<'.'>(".abcd"), Some(("", "abcd")));
        assert_eq!(split_at_middle_match::<'.'>("abcd."), Some(("abcd", "")));
        assert_eq!(split_at_middle_match::<'.'>("ab.cd"), Some(("ab", "cd")));
        assert_eq!(split_at_middle_match::<'.'>("abc.de"), Some(("abc", "de")));
        assert_eq!(split_at_middle_match::<'.'>("ab.cde"), Some(("ab", "cde")));
        assert_eq!(split_at_middle_match::<'.'>("abc"), None);
        assert_eq!(split_at_middle_match::<'.'>("abcd"), None);
    }
}
