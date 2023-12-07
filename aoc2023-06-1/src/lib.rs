use std::{iter::zip, sync::OnceLock};

use regex::Regex;

fn get_numbers(line: &str) -> Vec<u64> {
    static NUMBERS: OnceLock<Regex> = OnceLock::new();
    NUMBERS
        .get_or_init(|| Regex::new(r"\d+").unwrap())
        .find_iter(line)
        .filter_map(|s| s.as_str().parse().ok())
        .collect()
}

pub fn product_of_record_breaking_strategies(mut it: impl Iterator<Item = String>) -> u64 {
    let times = get_numbers(it.next().unwrap().as_str());
    let records = get_numbers(it.next().unwrap().as_str());
    zip(times.iter(), records.iter())
        // TODO - real number of ways to beat record!
        .map(|(time, record)| time + record)
        .reduce(|acc, v| acc * v)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        assert_eq!(
            product_of_record_breaking_strategies(example.lines().map(String::from)),
            288
        )
    }
}
