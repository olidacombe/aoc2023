use std::fmt;

struct Network {
    start: (usize, usize),
    pipes: Vec<Vec<char>>,
    interiousity: Vec<Vec<i32>>,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::default();
        for row in self.interiousity.iter() {
            for col in row {
                ret.push_str(&col.to_string());
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
        for (row, line) in it.enumerate() {
            let chars: Vec<char> = line.chars().collect();
            if let Some(col) = chars.iter().position(|c| *c == 'S') {
                start = Some((row, col));
            }
            interiousity.push(vec![0; chars.len()]);
            pipes.push(chars);
        }
        Self {
            interiousity,
            pipes,
            start: start.unwrap(),
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

    fn tip_column(&mut self, walker: &NetWalker, multiplier: i32) {
        for i in 0..walker.row {
            self.interiousity[i][walker.col] -= multiplier;
        }
        for i in (walker.row + 1)..self.interiousity.len() {
            self.interiousity[i][walker.col] += multiplier;
        }
    }

    fn tip_row(&mut self, walker: &NetWalker, multiplier: i32) {
        for i in 0..walker.col {
            self.interiousity[walker.row][i] -= multiplier;
        }
        for i in (walker.col + 1)..self.interiousity[0].len() {
            self.interiousity[walker.row][i] += multiplier;
        }
    }

    fn tip_start(&mut self, walker: &NetWalker, came_from: Direction) {
        let matchers: [(Direction, (i32, i32)); 4] = [
            (Direction::Left, (0, -1)),
            (Direction::Down, (1, 0)),
            (Direction::Right, (0, 1)),
            (Direction::Up, (-1, 0)),
        ];
        let mut other_dir = None;
        for (dir, (row, col)) in matchers {
            if dir == came_from {
                continue;
            }
            if row == -1 && walker.row == 0 {
                continue;
            }
            if col == -1 && walker.col == 0 {
                continue;
            }
            if self.get(
                (walker.row as i32 + row) as usize,
                (walker.col as i32 + col) as usize,
            ) != Some(&'.')
            {
                other_dir = Some(dir);
                break;
            }
        }
        let other_dir = other_dir.unwrap();
        match came_from {
            Direction::Left => {
                self.tip_column(&walker, 1);
                match other_dir {
                    Direction::Down => self.tip_row(&walker, -1),
                    Direction::Up => self.tip_row(&walker, 1),
                    _ => {}
                }
            }
            Direction::Down => {
                self.tip_row(&walker, 1);
                match other_dir {
                    Direction::Left => self.tip_column(&walker, -1),
                    Direction::Right => self.tip_column(&walker, 1),
                    _ => {}
                }
            }
            Direction::Right => {
                self.tip_column(&walker, -1);
                match other_dir {
                    Direction::Down => self.tip_row(&walker, 1),
                    Direction::Up => self.tip_row(&walker, -1),
                    _ => {}
                }
            }
            Direction::Up => {
                self.tip_row(&walker, -1);
                match other_dir {
                    Direction::Left => self.tip_column(&walker, 1),
                    Direction::Right => self.tip_column(&walker, -1),
                    _ => {}
                }
            }
        }
    }

    // progress a walker, return true if we have hit the start
    pub fn next(&mut self, walker: &mut NetWalker) -> bool {
        walker.age += 1;
        let pipe = self.get(walker.row, walker.col).unwrap();
        match walker.came_from {
            Direction::Left => match pipe {
                '-' => {
                    self.tip_column(&walker, 1);
                    walker.col += 1;
                }
                'J' => {
                    self.tip_column(&walker, 1);
                    self.tip_row(&walker, 1);
                    walker.row -= 1;
                    walker.came_from = Direction::Down;
                }
                '7' => {
                    self.tip_column(&walker, 1);
                    self.tip_row(&walker, -1);
                    walker.row += 1;
                    walker.came_from = Direction::Up;
                }
                'S' => {
                    self.tip_start(&walker, Direction::Left);
                    return true;
                }
                _ => {
                    dbg!(walker);
                }
            },
            Direction::Down => match pipe {
                '|' => {
                    self.tip_row(&walker, 1);
                    walker.row -= 1;
                }
                'F' => {
                    self.tip_row(&walker, 1);
                    self.tip_column(&walker, 1);
                    walker.col += 1;
                    walker.came_from = Direction::Left;
                }
                '7' => {
                    self.tip_row(&walker, 1);
                    self.tip_column(&walker, -1);
                    walker.col -= 1;
                    walker.came_from = Direction::Right;
                }
                'S' => {
                    self.tip_start(&walker, Direction::Down);
                    return true;
                }
                _ => {
                    dbg!(walker);
                }
            },
            Direction::Right => match pipe {
                '-' => {
                    self.tip_column(&walker, -1);
                    walker.col -= 1;
                }
                'L' => {
                    self.tip_column(&walker, -1);
                    self.tip_row(&walker, 1);
                    walker.row -= 1;
                    walker.came_from = Direction::Down;
                }
                'F' => {
                    self.tip_column(&walker, -1);
                    self.tip_row(&walker, -1);
                    walker.row += 1;
                    walker.came_from = Direction::Up;
                }
                'S' => {
                    self.tip_start(&walker, Direction::Right);
                    return true;
                }
                _ => {
                    dbg!(walker);
                }
            },
            Direction::Up => match pipe {
                '|' => {
                    self.tip_row(&walker, -1);
                    walker.row += 1;
                }
                'J' => {
                    self.tip_row(&walker, -1);
                    self.tip_column(&walker, -1);
                    walker.col -= 1;
                    walker.came_from = Direction::Right;
                }
                'L' => {
                    self.tip_row(&walker, -1);
                    self.tip_column(&walker, 1);
                    walker.col += 1;
                    walker.came_from = Direction::Left;
                }
                'S' => {
                    self.tip_start(&walker, Direction::Up);
                    return true;
                }
                _ => {
                    dbg!(walker);
                }
            },
        }
        false
    }

    pub fn normalize(&mut self) {
        for row in self.interiousity.iter_mut() {
            for col in row {
                if *col != 0 {
                    *col = 1;
                }
            }
        }
    }

    pub fn sum(&self) -> i32 {
        self.interiousity
            .iter()
            .map(|row| row.iter().sum::<i32>())
            .sum()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug)]
struct NetWalker {
    pub came_from: Direction,
    pub row: usize,
    pub col: usize,
    pub age: usize,
}

pub fn num_enclosed_tiles(it: impl Iterator<Item = String>) -> u64 {
    let mut network = Network::from(it);
    let mut walkers = network.walkers();
    let mut walker = walkers.first_mut().unwrap();
    while !network.next(&mut walker) {}
    network.normalize();
    dbg!(walker.age);
    dbg!(network.sum());
    println!("{network}");
    u64::default()
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
