use std::{collections::HashSet, ops::AddAssign};

use common::parse::{self};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error("missing start")]
    MissingStart,
}

pub fn number_of_reachable_garden_plots(
    mut it: impl Iterator<Item = String>,
    steps: usize,
) -> Result<usize> {
    let mut map_builder = MapBuilder::default();
    for line in it {
        map_builder += MapRow::from(line.as_str());
    }
    let map = map_builder.build()?;
    let mut walker = MapWalker::new(map);
    for _ in 0..steps {
        walker.explore();
    }
    dbg!(&walker);
    Ok(walker.size())
}

#[derive(Debug)]
struct MapWalker {
    map: Map,
    reachable: HashSet<usize>,
}

impl MapWalker {
    fn new(map: Map) -> Self {
        let reachable = HashSet::from([map.start]);
        Self { map, reachable }
    }
    fn explore(&mut self) {
        let mut new_positions = HashSet::new();
        for position in &self.reachable {
            new_positions.extend(self.map.reachable_from(*position));
        }
        self.reachable.extend(new_positions);
    }
    fn size(&self) -> usize {
        self.reachable.len()
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    start: usize,
    rocks: HashSet<usize>,
}

impl Map {
    fn reachable_from(&self, position: usize) -> HashSet<usize> {
        let mut positions = HashSet::new();
        let height = self.height;
        let width = self.width;
        let x = position % width;
        let y = position / width;
        if y > 0 && !self.rocks.contains(&(position - width)) {
            positions.insert(position - self.width);
        }
        if y < height - 1 && !self.rocks.contains(&(position + width)) {
            positions.insert(position + self.width);
        }
        if x > 0 && !self.rocks.contains(&(x - 1)) {
            positions.insert(x - 1);
        }
        if x < width - 1 && !self.rocks.contains(&(x + 1)) {
            positions.insert(x + 1);
        }
        positions
    }
}

#[derive(Default)]
struct MapBuilder {
    width: usize,
    height: usize,
    start: Option<usize>,
    rocks: HashSet<usize>,
}

impl AddAssign<MapRow> for MapBuilder {
    fn add_assign(&mut self, mut row: MapRow) {
        row += self.width * self.height;
        self.height += 1;
        self.width = row.width;
        if row.start.is_some() {
            self.start = row.start;
        }
        self.rocks.extend(row.rocks);
    }
}

impl MapBuilder {
    fn build(self) -> Result<Map> {
        let Self {
            width,
            height,
            start,
            rocks,
        } = self;
        let start = start.ok_or(Error::MissingStart)?;
        Ok(Map {
            width,
            height,
            start,
            rocks,
        })
    }
}

struct MapRow {
    width: usize,
    start: Option<usize>,
    rocks: HashSet<usize>,
}

impl From<&str> for MapRow {
    fn from(value: &str) -> Self {
        let width = value.len();
        let mut start = None;
        let mut rocks = HashSet::new();
        for (i, char) in value.chars().enumerate() {
            match char {
                'S' => {
                    start = Some(i);
                }
                '#' => {
                    rocks.insert(i);
                }
                _ => {}
            }
        }
        Self {
            width,
            start,
            rocks,
        }
    }
}

impl AddAssign<usize> for MapRow {
    fn add_assign(&mut self, offset: usize) {
        if let Some(ref mut start) = self.start {
            *start += offset;
        }
        self.rocks = self.rocks.iter().map(|rock| rock + offset).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "};
        assert_eq!(
            number_of_reachable_garden_plots(example.lines().map(String::from), 6)?,
            16
        );
        Ok(())
    }
}
