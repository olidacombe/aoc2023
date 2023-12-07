pub fn total_winnings(mut it: impl Iterator<Item = String>) -> u64 {
    u64::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        assert_eq!(total_winnings(example.lines().map(String::from)), 6440);
    }
}
