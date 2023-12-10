use aoc2023_09_1::Seq;

pub fn extrapolated_sum(it: impl Iterator<Item = String>) -> i64 {
    it.map(|line| Seq::from(line.as_str()).reversed().extrapolate())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        assert_eq!(extrapolated_sum(example.lines().map(String::from)), 2);
    }
}
