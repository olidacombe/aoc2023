use std::iter::zip;

#[derive(Debug)]
enum Count {
    Rollable(usize),
    Finished(usize),
}

impl Default for Count {
    fn default() -> Self {
        Self::Rollable(0)
    }
}

impl Count {
    pub fn take(self) -> usize {
        match self {
            Count::Rollable(count) => count,
            Count::Finished(count) => count,
        }
    }
}

#[derive(Debug)]
struct LoadCalculator {
    num_rows: usize,
    counts: Vec<Count>,
}

impl LoadCalculator {
    pub fn new(num_columns: usize) -> Self {
        let mut counts = Vec::new();
        counts.resize_with(num_columns, Default::default);
        Self {
            num_rows: 0,
            counts,
        }
    }

    pub fn push_row(&mut self, line: &str) {
        self.num_rows += 1;
        for (ch, cnt) in zip(line.chars(), self.counts.iter_mut()) {
            if let Count::Rollable(current) = cnt {
                match ch {
                    '#' => {
                        *cnt = Count::Finished(*current);
                    }
                    'O' => {
                        *current += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn total(self) -> usize {
        let n = self.num_rows;
        // n(n+1)/2 - (n-c)(n-c+1)/2
        self.counts
            .into_iter()
            .map(|c| c.take())
            .map(|c| {
                let ret = n * (n + 1) / 2 - (n - c) * (n - c + 1) / 2;
                dbg!(&ret);
                ret
            })
            // .map(|c| n * (n + 1) / 2 - (n - c) * (n - c + 1) / 2)
            .sum()
    }
}

pub fn total_load(it: impl Iterator<Item = String>) -> usize {
    let mut it = it.peekable();
    let mut load_calculator = LoadCalculator::new(it.peek().unwrap().len());
    for line in it {
        load_calculator.push_row(line.as_str());
    }
    dbg!(&load_calculator);
    load_calculator.total()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};
        assert_eq!(total_load(example.lines().map(String::from)), 136);
    }
}
