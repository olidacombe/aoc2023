use std::sync::OnceLock;

use bitvec::vec::BitVec;
use regex::{Match, Matches, Regex};

#[derive(Debug, Default)]
struct HitMatrix {
    matrix: [BitVec; 3],
    current: usize,
}

impl HitMatrix {
    fn prev_mut(&mut self) -> &mut BitVec {
        &mut self.matrix[(self.current + 2) % 3]
    }

    fn size_up(&mut self, len: usize) {
        for row in self.matrix.iter_mut() {
            row.resize(len, false);
        }
    }

    fn set_hit(&mut self, idx: usize) {
        for row in self.matrix.iter_mut() {
            if idx > 0 {
                row.set(idx - 1, true);
            }
            row.set(idx, true);
            if idx < row.len() - 1 {
                row.set(idx + 1, true);
            }
        }
    }

    pub fn read(&mut self, line: &str) {
        // make sure we have capacity
        self.size_up(line.len());
        // find all occurrences of a symbol on the line
        static SYMBOL: OnceLock<Regex> = OnceLock::new();
        let symbol_indices = SYMBOL
            .get_or_init(|| Regex::new(r"[^.0-9]").unwrap())
            .find_iter(line)
            .map(|m| m.start());
        self.prev_mut().fill(false);
        for i in symbol_indices {
            self.set_hit(i);
        }
    }

    pub fn inc(&mut self) {
        self.current = (self.current + 1) % 3;
    }

    pub fn hit_test(&self, mtch: &Match) -> Option<u32> {
        match self.matrix[self.current][mtch.start()..mtch.end()].any() {
            true => Some(mtch.as_str().parse::<u32>().unwrap()),
            false => None,
        }
    }
}

fn number_captures(line: &str) -> Matches {
    static NUMBER: OnceLock<Regex> = OnceLock::new();
    NUMBER
        .get_or_init(|| Regex::new(r"\d+").unwrap())
        .find_iter(line)
}

pub fn sum_part_numbers(it: impl Iterator<Item = String>) -> u32 {
    let mut matrix = HitMatrix::default();
    let mut total = 0;
    let mut it = it.peekable();
    matrix.inc();
    if let Some(ref line) = it.peek() {
        matrix.read(&line);
    }
    while let Some(line) = it.next() {
        if let Some(line) = it.peek() {
            matrix.read(&line);
        }
        for cap in number_captures(&line) {
            if let Some(num) = matrix.hit_test(&cap) {
                total += num;
            }
        }
        matrix.inc()
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    use bitvec::prelude::*;
    use indoc::indoc;

    #[test]
    fn hit_matrix_basic_read() {
        let mut matrix = HitMatrix::default();
        let line = "617*......";
        matrix.read(line);

        assert_eq!(matrix.matrix[0], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]);
        assert_eq!(matrix.matrix[1], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]);
        assert_eq!(matrix.matrix[2], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn hit_matrix_cycle() {
        let mut matrix = HitMatrix::default();
        matrix.inc();
        matrix.read("467..114..");
        assert_eq!(matrix.current, 1);
        assert_eq!(matrix.matrix[0], bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(matrix.matrix[1], bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]); // <-- current
        assert_eq!(matrix.matrix[2], bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        matrix.inc();
        matrix.read("...*......");
        assert_eq!(matrix.current, 2);
        assert_eq!(matrix.matrix[0], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]);
        assert_eq!(matrix.matrix[1], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]);
        assert_eq!(matrix.matrix[2], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]); // <-- current
        matrix.inc();
        matrix.read("..35..633.");
        assert_eq!(matrix.current, 0);
        assert_eq!(matrix.matrix[0], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]); // <-- current
        assert_eq!(matrix.matrix[1], bitvec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]);
        assert_eq!(matrix.matrix[2], bitvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        matrix.inc();
        matrix.read("......#...");
        assert_eq!(matrix.current, 1);
        assert_eq!(matrix.matrix[0], bitvec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0]);
        assert_eq!(matrix.matrix[1], bitvec![0, 0, 1, 1, 1, 1, 1, 1, 0, 0]); // <-- current
        assert_eq!(matrix.matrix[2], bitvec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0]);
    }

    #[test]
    fn full_example() {
        let example = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        assert_eq!(sum_part_numbers(example.lines().map(String::from)), 4361);
    }
}
