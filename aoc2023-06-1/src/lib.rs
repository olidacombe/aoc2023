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

fn distance(race_time: &u64, hold_time: &u64) -> u64 {
    hold_time * (race_time - hold_time)
}

fn num_ways_to_beat_record(race_time: &u64, record: &u64) -> u64 {
    let t: f64 = *race_time as f64;
    let r: f64 = *record as f64;

    let midpoint = t / 2.0;
    let delta = (t * t / 4.0 - r).sqrt();

    let mut min_win = (midpoint - delta).ceil() as u64;
    if distance(race_time, &min_win) == *record {
        min_win += 1;
    }
    let mut max_win = (midpoint + delta).floor() as u64;
    if distance(race_time, &max_win) == *record {
        max_win -= 1;
    }
    max_win - min_win + 1
}

pub fn product_of_record_breaking_strategies(mut it: impl Iterator<Item = String>) -> u64 {
    let times = get_numbers(it.next().unwrap().as_str());
    let records = get_numbers(it.next().unwrap().as_str());
    zip(times.iter(), records.iter())
        .map(|(time, record)| num_ways_to_beat_record(time, record))
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
