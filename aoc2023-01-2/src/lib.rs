use std::{collections::HashMap, sync::OnceLock};

use color_eyre::{eyre::eyre, Result};

trait EndScanner {
    type Return;

    fn first_n(&self, n: usize) -> Self::Return;
    fn last_n(&self, n: usize) -> Self::Return;
}

impl<'a> EndScanner for &'a String {
    type Return = &'a str;
    fn first_n(&self, n: usize) -> Self::Return {
        &self.as_str()[0..n]
    }
    fn last_n(&self, n: usize) -> Self::Return {
        // TODO
        &self.as_str()[0..n]
    }
}

fn stats_with_digit_repr(s: &str) -> Option<u32> {
    static DIGITS: OnceLock<HashMap<&str, u32>> = OnceLock::new();
    DIGITS
        .get_or_init(|| {
            HashMap::from([
                ("0", 0),
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9),
                ("zero", 0),
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ])
        })
        .iter()
        .find(|(match_me, _)| s.starts_with(*match_me))
        .map(|(_, v)| v)
        .copied()
}

fn first_digit(v: &str) -> Result<u32> {
    for i in 0..v.len() {
        if let Some(digit) = stats_with_digit_repr(&v[i..]) {
            return Ok(digit);
        }
    }
    Err(eyre!("No digit found in {v}"))
}

fn last_digit(v: &str) -> Result<u32> {
    for i in (0..v.len()).rev() {
        if let Some(digit) = stats_with_digit_repr(&v[i..]) {
            return Ok(digit);
        }
    }
    Err(eyre!("No digit found in {v}"))
}

fn calibration(v: &str) -> Result<u32> {
    Ok(10 * first_digit(v)? + last_digit(v)?)
}

pub fn sum_calibration(it: impl Iterator<Item = String>) -> Result<u32> {
    it.filter_map(|line| calibration(&line).ok())
        .reduce(|acc, v| acc + v)
        .ok_or(eyre!("Some calibration summing error 🤷"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn examples() -> Result<()> {
        for (line, expected) in [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ] {
            assert_eq!(
                calibration(line)?,
                expected,
                "Expect \"{line}\" => {expected}"
            );
        }
        Ok(())
    }

    #[test]
    fn full_calculation() -> Result<()> {
        let example = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        assert_eq!(sum_calibration(example.lines().map(String::from))?, 281);
        Ok(())
    }
}
