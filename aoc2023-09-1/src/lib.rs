use nom::{character::complete::space1, multi::separated_list1, IResult};

fn parse_seq(line: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, nom::character::complete::i64)(line)
}

#[derive(Debug)]
struct Seq(Vec<i64>);

impl From<&str> for Seq {
    fn from(value: &str) -> Self {
        Self(parse_seq(value).unwrap().1)
    }
}

impl Seq {
    /// returns a Seq from this Seq's differences
    fn diffs(&self) -> Self {
        Self(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect(),
        )
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|v| *v == 0)
    }

    /// calculates the next value
    pub fn extrapolate(&self) -> i64 {
        if self.is_zero() {
            0
        } else {
            self.0.last().unwrap() + self.diffs().extrapolate()
        }
    }
}

pub fn extrapolated_sum(it: impl Iterator<Item = String>) -> i64 {
    it.map(|line| Seq::from(line.as_str()).extrapolate()).sum()
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
        assert_eq!(extrapolated_sum(example.lines().map(String::from)), 114);
    }
}
