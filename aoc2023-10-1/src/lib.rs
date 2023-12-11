struct Network {
    start: (usize, usize),
    pipes: Vec<Vec<char>>,
}

impl Network {
    pub fn from(it: impl Iterator<Item = String>) -> Self {
        let mut pipes = Vec::new();
        let mut start = None;
        for (row, line) in it.enumerate() {
            let chars: Vec<char> = line.chars().collect();
            if let Some(col) = chars.iter().position(|c| *c == 'S') {
                start = Some((row, col));
            }
            pipes.push(chars);
        }
        Self {
            start: start.unwrap(),
            pipes,
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

    // progress a walker, return true if we have hit the start
    pub fn next(&self, walker: &mut NetWalker) -> bool {
        walker.age += 1;
        let pipe = self.get(walker.row, walker.col).unwrap();
        dbg!(pipe);
        match walker.came_from {
            Direction::Left => match pipe {
                '-' => {
                    walker.col += 1;
                }
                'J' => {
                    walker.row -= 1;
                    walker.came_from = Direction::Down;
                }
                '7' => {
                    walker.row += 1;
                    walker.came_from = Direction::Up;
                }
                'S' => {
                    return true;
                }
                _ => {
                    dbg!(walker);
                }
            },
            Direction::Down => match pipe {
                '|' => {
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
                    dbg!(walker);
                }
            },
            Direction::Right => match pipe {
                '-' => {
                    walker.col -= 1;
                }
                'L' => {
                    walker.row -= 1;
                    walker.came_from = Direction::Down;
                }
                'F' => {
                    walker.row += 1;
                    walker.came_from = Direction::Up;
                }
                'S' => {
                    return true;
                }
                _ => {
                    dbg!(walker);
                }
            },
            Direction::Up => match pipe {
                '|' => {
                    walker.row += 1;
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
                    return true;
                }
                _ => {
                    dbg!(walker);
                }
            },
        }
        false
    }
}

#[derive(Debug)]
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

pub fn farthest_point(it: impl Iterator<Item = String>) -> u64 {
    let network = Network::from(it);
    let mut walkers = network.walkers();
    let mut walker = walkers.first_mut().unwrap();
    while !network.next(&mut walker) {}
    walker.age as u64 / 2
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        assert_eq!(farthest_point(example.lines().map(String::from)), 8);
    }
}
