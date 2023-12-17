use std::{collections::VecDeque, ops::Range};

enum Direction {
    North,
    West,
    South,
    East,
}

enum Rock {
    None,
    Rounded,
    Cube,
}

struct LoadCalculator {
    cols: usize,
    rocks: Vec<Rock>,
}

impl LoadCalculator {
    pub fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    pub fn load(&self) -> usize {
        0
    }

    pub fn new(cols: usize) -> Self {
        let rocks = Vec::new();
        Self { cols, rocks }
    }

    pub fn push_row(&mut self, line: &str) {
        for c in line.chars() {
            self.rocks.push(match c {
                'O' => Rock::Rounded,
                '#' => Rock::Cube,
                _ => Rock::None,
            });
        }
    }

    fn tilt(&mut self, direction: Direction) {
        let rows = self.rocks.len() / self.cols;
        let (outer, inner): (
            Range<usize>,
            Box<dyn Fn(usize) -> Box<dyn Iterator<Item = usize>>>,
        ) = match direction {
            Direction::North => (
                0..self.cols,
                Box::new(|col| Box::new((col..col + (rows - 1) * self.cols).step_by(self.cols))),
            ),
            Direction::West => (
                0..rows,
                Box::new(|row| Box::new(row * self.cols..(row + 1) * self.cols - 1)),
            ),
            Direction::South => (
                0..self.cols,
                Box::new(|col| {
                    Box::new((col..col + (rows - 1) * self.cols).rev().step_by(self.cols))
                }),
            ),
            Direction::East => (
                0..rows,
                Box::new(|row| Box::new((row * self.cols..(row + 1) * self.cols - 1).rev())),
            ),
        };

        for o in outer {
            let mut queue = VecDeque::new();
            for i in inner(o) {
                match self.rocks[i] {
                    Rock::None => {
                        queue.push_back(i);
                    }
                    Rock::Rounded => {
                        if let Some(rolled_position) = queue.pop_front() {
                            self.rocks.swap(i, rolled_position);
                        }
                    }
                    Rock::Cube => {
                        queue.clear();
                    }
                }
            }
        }
    }
}

pub fn total_load(it: impl Iterator<Item = String>) -> usize {
    let mut it = it.peekable();
    let mut load_calculator = LoadCalculator::new(it.peek().unwrap().len());
    for line in it {
        load_calculator.push_row(line.as_str());
    }
    for _ in 0..1000000000 {
        load_calculator.cycle();
    }
    load_calculator.load()
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
        assert_eq!(total_load(example.lines().map(String::from)), 64);
    }
}
