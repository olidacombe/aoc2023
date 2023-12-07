use std::sync::OnceLock;

use regex::Regex;

fn strip_whitespace(line: &str) -> String {
    // can regex is not all whitespace is simple space
    line.replace(" ", "")
}

fn get_number(line: &str) -> u64 {
    let stripped = strip_whitespace(line);
    static NUMBER: OnceLock<Regex> = OnceLock::new();
    NUMBER
        .get_or_init(|| Regex::new(r"\d+").unwrap())
        .find(stripped.as_str())
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
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

pub fn num_record_breaking_strategies(mut it: impl Iterator<Item = String>) -> u64 {
    let time = get_number(it.next().unwrap().as_str());
    let record = get_number(it.next().unwrap().as_str());
    num_ways_to_beat_record(&time, &record)
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
            num_record_breaking_strategies(example.lines().map(String::from)),
            71503
        )
    }
}
