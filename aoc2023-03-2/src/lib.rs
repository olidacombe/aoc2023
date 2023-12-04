use array_macro::array;
use bitvec::vec::BitVec;
use regex::{Match, Matches, Regex};
use std::sync::OnceLock;

struct Gear {
    matrix: [BitVec; 3],
    current: usize,
    pub finished: bool,
    hits: Vec<u32>,
}

fn make_row(size: usize, idx: usize) -> BitVec {
    let mut v = BitVec::with_capacity(size);
    v.resize(size, false);
    if idx > 0 {
        v.set(idx - 1, true);
    }
    v.set(idx, true);
    if idx < size - 1 {
        v.set(idx + 1, true);
    }
    v
}

impl Gear {
    fn prev_mut(&mut self) -> &mut BitVec {
        &mut self.matrix[(self.current + 2) % 3]
    }

    fn new(size: usize, idx: usize) -> Self {
        let matrix = array![_ => make_row(size, idx); 3];
        Self {
            matrix,
            current: 0,
            hits: Vec::new(),
            finished: false,
        }
    }

    pub fn inc(&mut self) {
        if self.current == 2 {
            self.finished = true
        }
        self.current = (self.current + 1) % 3;
        self.prev_mut().fill(false);
    }

    pub fn hit_test(&mut self, mtch: &Match) {
        if self.matrix[self.current][mtch.start()..mtch.end()].any() {
            if self.hits.len() == 2 {
                self.finished = true;
                self.hits.clear();
            }
            self.hits.push(mtch.as_str().parse::<u32>().unwrap());
        }
    }

    pub fn ratio(&self) -> Option<u32> {
        if self.finished && self.hits.len() == 2 {
            Some(self.hits[0] * self.hits[1])
        } else {
            None
        }
    }
}

#[derive(Default)]
struct GearScanner {
    gear_ratio_sum: u32,
    gears: Vec<Gear>,
}

impl GearScanner {
    pub fn read_gears(&mut self, line: &str) {
        // find all occurrences of a symbol on the line
        static SYMBOL: OnceLock<Regex> = OnceLock::new();
        let gear_indices = SYMBOL
            .get_or_init(|| Regex::new(r"\*").unwrap())
            .find_iter(line)
            .map(|m| m.start());
        for idx in gear_indices {
            self.gears.push(Gear::new(line.len(), idx));
        }
    }

    pub fn read_numbers(&mut self, line: &str) {
        for cap in number_captures(line) {
            for matrix in self.gears.iter_mut() {
                matrix.hit_test(&cap);
            }
        }
    }

    pub fn inc(&mut self) {
        for gear in self.gears.iter_mut() {
            if let Some(ratio) = gear.ratio() {
                self.gear_ratio_sum += ratio;
            }
        }
        self.gears.retain(|g| !g.finished);
        for gear in self.gears.iter_mut() {
            gear.inc();
        }
    }
}

fn number_captures(line: &str) -> Matches {
    static NUMBER: OnceLock<Regex> = OnceLock::new();
    NUMBER
        .get_or_init(|| Regex::new(r"\d+").unwrap())
        .find_iter(line)
}

pub fn sum_gear_ratios(it: impl Iterator<Item = String>) -> u32 {
    let mut scanner = GearScanner::default();

    let mut it = it.peekable();

    while let Some(line) = it.next() {
        if let Some(line) = it.peek() {
            scanner.read_gears(&line);
        }
        scanner.read_numbers(&line);
        scanner.inc();
    }
    scanner.gear_ratio_sum
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

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
        assert_eq!(sum_gear_ratios(example.lines().map(String::from)), 467835);
    }
}
