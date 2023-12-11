use std::collections::HashSet;

#[derive(Default)]
struct Universe {
    size: (usize, usize),
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    pub fn from(it: impl Iterator<Item = String>) -> Self {
        let mut me = Self::default();
        for (row, line) in it.enumerate() {
            me.size.0 += 1;
            me.size.1 = line.len();
            for (col, char) in line.chars().enumerate() {
                if char == '#' {
                    me.galaxies.push((row, col));
                }
            }
        }
        me
    }

    pub fn expand(&mut self, expansion_factor: usize) {
        let rows = HashSet::<usize>::from_iter(self.galaxies.iter().map(|(row, _)| *row));
        let cols = HashSet::<usize>::from_iter(self.galaxies.iter().map(|(_, col)| *col));
        let mut row_expansions = Vec::new();
        let mut col_expansions = Vec::new();
        for i in 0..self.size.0 {
            if rows.get(&i).is_none() {
                row_expansions.push(i);
            }
        }
        for j in 0..self.size.1 {
            if cols.get(&j).is_none() {
                col_expansions.push(j);
            }
        }
        row_expansions.sort();
        col_expansions.sort();
        for to_insert in row_expansions.iter().rev() {
            for (row, _) in self.galaxies.iter_mut().filter(|(row, _)| row > to_insert) {
                *row += expansion_factor - 1;
            }
        }
        for to_insert in col_expansions.iter().rev() {
            for (_, col) in self.galaxies.iter_mut().filter(|(_, col)| col > to_insert) {
                *col += expansion_factor - 1;
            }
        }
    }

    pub fn sum_shortest_paths(&self) -> u64 {
        let mut sum = 0;
        for (i, a) in self.galaxies.iter().enumerate() {
            for b in self.galaxies.iter().skip(i) {
                sum += (b.0 as i64 - a.0 as i64).abs() + (b.1 as i64 - a.1 as i64).abs();
            }
        }
        sum as u64
    }
}

pub fn sum_of_shortest_paths(it: impl Iterator<Item = String>, expansion_factor: usize) -> u64 {
    let mut universe = Universe::from(it);
    universe.expand(expansion_factor);
    universe.sum_shortest_paths()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example1() {
        let example = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        assert_eq!(
            sum_of_shortest_paths(example.lines().map(String::from), 10),
            1030
        );
    }

    #[test]
    fn full_example2() {
        let example = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        assert_eq!(
            sum_of_shortest_paths(example.lines().map(String::from), 100),
            8410
        );
    }
}
