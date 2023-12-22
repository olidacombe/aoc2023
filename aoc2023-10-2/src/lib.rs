use std::{
    collections::HashSet,
    fmt,
    ops::{Add, RangeBounds},
};

struct Network {
    start: (usize, usize),
    pipes: Vec<Vec<char>>,
    interiousity: Vec<Vec<i32>>,
    path: HashSet<(usize, usize)>,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::default();
        for (i, row) in self.interiousity.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if self.path.contains(&(i, j)) || *col == 0 {
                    ret.push_str("0");
                } else {
                    ret.push_str("1");
                }
            }
            ret.push('\n');
        }
        write!(f, "{ret}")
    }
}

impl Network {
    pub fn from(it: impl Iterator<Item = String>) -> Self {
        let mut pipes = Vec::new();
        let mut interiousity = Vec::new();
        let mut start = None;
        let mut path = HashSet::new();
        for (row, line) in it.enumerate() {
            let chars: Vec<char> = line.chars().collect();
            if let Some(col) = chars.iter().position(|c| *c == 'S') {
                start = Some((row, col));
                path.insert((row, col));
            }
            interiousity.push(vec![0; chars.len()]);
            pipes.push(chars);
        }
        Self {
            interiousity,
            pipes,
            start: start.unwrap(),
            path,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&char> {
        self.pipes.get(row).map(|row| row.get(col)).flatten()
    }

    fn walkers(&self) -> Vec<NetWalker> {
        let mut walkers = Vec::new();
        let (row, col) = self.start;
        if let Some('-' | 'J' | '7') = self.get(row, col + 1) {
            walkers.push(NetWalker {
                came_from: Direction::Left,
                row,
                col: col + 1,
                age: 1,
            })
        }
        if row > 0 {
            if let Some('|' | 'F' | '7') = self.get(row - 1, col) {
                walkers.push(NetWalker {
                    came_from: Direction::Down,
                    row: row - 1,
                    col,
                    age: 1,
                })
            }
        }
        if col > 0 {
            if let Some('-' | 'L' | 'F') = self.get(row, col - 1) {
                walkers.push(NetWalker {
                    came_from: Direction::Right,
                    row,
                    col: col - 1,
                    age: 1,
                })
            }
        }
        if let Some('|' | 'J' | 'L') = self.get(row + 1, col) {
            walkers.push(NetWalker {
                came_from: Direction::Up,
                row: row + 1,
                col,
                age: 1,
            })
        }
        walkers
    }

    fn tip_row(&mut self, walker: &NetWalker, multiplier: i32) {
        for i in 0..walker.col {
            self.interiousity[walker.row][i] -= multiplier;
        }
        for i in walker.col..self.interiousity[0].len() {
            self.interiousity[walker.row][i] += multiplier;
        }
    }

    // progress a walker, return true if we have hit the start
    pub fn next(&mut self, walker: &mut NetWalker) -> bool {
        walker.age += 1;
        let pipe = self.get(walker.row, walker.col).unwrap();
        match walker.came_from {
            Direction::Left => match pipe {
                '-' => {
                    walker.col += 1;
                }
                'J' => {
                    self.tip_row(&walker, 1);
                    walker.row -= 1;
                    walker.came_from = Direction::Down;
                }
                '7' => {
                    walker.row += 1;
                    self.tip_row(&walker, -1);
                    walker.came_from = Direction::Up;
                }
                'S' => {
                    return true;
                }
                _ => {
                    dbg!(&walker);
                }
            },
            Direction::Down => match pipe {
                '|' => {
                    self.tip_row(&walker, 1);
                    walker.row -= 1;
                }
                'F' => {
                    walker.col += 1;
                    walker.came_from = Direction::Left;
                }
                '7' => {
                    walker.col -= 1;
                    walker.came_from = Direction::Right;
                }
                'S' => {
                    return true;
                }
                _ => {
                    dbg!(&walker);
                }
            },
            Direction::Right => match pipe {
                '-' => {
                    walker.col -= 1;
                }
                'L' => {
                    self.tip_row(&walker, 1);
                    walker.row -= 1;
                    walker.came_from = Direction::Down;
                }
                'F' => {
                    walker.row += 1;
                    self.tip_row(&walker, -1);
                    walker.came_from = Direction::Up;
                }
                'S' => {
                    return true;
                }
                _ => {
                    dbg!(&walker);
                }
            },
            Direction::Up => match pipe {
                '|' => {
                    walker.row += 1;
                    self.tip_row(&walker, -1);
                }
                'J' => {
                    walker.col -= 1;
                    walker.came_from = Direction::Right;
                }
                'L' => {
                    walker.col += 1;
                    walker.came_from = Direction::Left;
                }
                'S' => {
                    self.tip_row(&walker, -1);
                    return true;
                }
                _ => {
                    dbg!(&walker);
                }
            },
        }
        self.path.insert(walker.coords());
        false
    }

    pub fn sum(&self) -> usize {
        let mut count = 0;
        for (i, row) in self.interiousity.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if !self.path.contains(&(i, j)) && *col != 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Clone, Debug)]
struct NetWalker {
    pub came_from: Direction,
    pub row: usize,
    pub col: usize,
    pub age: usize,
}

impl Add<(i32, i32)> for &NetWalker {
    type Output = NetWalker;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        let mut ret = self.clone();
        ret.row = (ret.row as i32 + rhs.0) as usize;
        ret.col = (ret.col as i32 + rhs.1) as usize;
        ret
    }
}

impl NetWalker {
    pub fn coords(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

pub fn num_enclosed_tiles(it: impl Iterator<Item = String>) -> usize {
    let mut network = Network::from(it);
    let mut walkers = network.walkers();
    let mut walker = walkers.first_mut().unwrap();
    while !network.next(&mut walker) {}
    dbg!(walker.age);
    dbg!(network.sum());
    println!("{network}");
    network.sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example_1() {
        let example = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};
        assert_eq!(num_enclosed_tiles(example.lines().map(String::from)), 4);
    }

    #[test]
    fn full_example_2() {
        let example = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};
        assert_eq!(num_enclosed_tiles(example.lines().map(String::from)), 8);
    }

    #[test]
    fn full_example_3() {
        let example = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        assert_eq!(num_enclosed_tiles(example.lines().map(String::from)), 10);
    }
}
