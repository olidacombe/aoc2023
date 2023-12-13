use nom::{
    branch::alt,
    bytes::complete::tag,
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

struct ArrangementFinder<'a> {
    known: &'a str,
    damage_sizes: Vec<u64>,
}

impl<'a> From<&'a ConditionRecord> for ArrangementFinder<'a> {
    fn from(value: &'a ConditionRecord) -> Self {
        ArrangementFinder {
            known: value.known.as_str(),
            damage_sizes: value.damage_sizes.clone(),
        }
    }
}

fn vec_decrement(vec: &Vec<u64>) -> Vec<u64> {
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

impl<'a> ArrangementFinder<'a> {
    fn possible_arrangements(&self) -> Vec<&str> {
        if !self.known.chars().all(|c| c == '.') && self.damage_sizes.iter().sum::<u64>() == 0 {
            return Vec::new();
        }
        let (first, rest) = self.known.split_at(1);
        match first {
            "." => Self::new(rest, self.damage_sizes.clone()).possible_arrangements(),
            "?" => [
                Self::new(&format!(".{rest}"), self.damage_sizes.clone()).possible_arrangements(),
                Self::new(&format!("#{rest}"), vec_decrement(&self.damage_sizes))
                    .possible_arrangements(),
            ]
            .concat(),
            _ => unreachable!(),
        }
    }

    fn new(known: &'a str, damage_sizes: Vec<u64>) -> Self {
        Self {
            known,
            damage_sizes,
        }
    }
}

impl ConditionRecord {
    fn possible_arrangements(&self) -> Vec<&str> {
        Vec::new()
    }
}

fn parse_condition_record(input: &str) -> IResult<&str, (&str, Vec<u64>)> {
    separated_pair(
        alt((tag("."), tag("#"), tag("?"))),
        space1,
        separated_list0(tag(","), u64),
    )(input)
}

pub fn sum_possible_arrangements(it: impl Iterator<Item = String>) -> u64 {
    let records: Vec<ConditionRecord> = it.map(ConditionRecord::from).collect();
    u64::default()
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
            21
        );
    }
}
