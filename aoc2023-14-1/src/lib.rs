use std::{iter::zip, ops::AddAssign};

#[derive(Debug, Default)]
struct Count {
    val: usize,
    offset: usize,
}

impl AddAssign<usize> for Count {
    fn add_assign(&mut self, rhs: usize) {
        self.val += rhs;
    }
}

impl Count {
    pub fn load(&self, num_rows: usize) -> usize {
        let n = num_rows - self.offset;
        let c = self.val;
        n * (n + 1) / 2 - (n - c) * (n - c + 1) / 2
    }

    pub fn new(offset: usize) -> Self {
        Self { val: 0, offset }
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
        let mut finished_counts = Vec::new();
        for (ch, cnt) in zip(line.chars(), self.counts.iter_mut()) {
            match ch {
                '#' => {
                    finished_counts.push(std::mem::replace(cnt, Count::new(self.num_rows)));
                }
                'O' => {
                    *cnt += 1;
                }
                _ => {}
            }
        }
        self.counts.append(&mut finished_counts);
    }

    pub fn total(self) -> usize {
        let n = self.num_rows;
        // n(n+1)/2 - (n-c)(n-c+1)/2
        self.counts.into_iter().map(|c| c.load(n)).sum()
    }
}

pub fn total_load(it: impl Iterator<Item = String>) -> usize {
    let mut it = it.peekable();
    let mut load_calculator = LoadCalculator::new(it.peek().unwrap().len());
    for line in it {
        load_calculator.push_row(line.as_str());
    }
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
