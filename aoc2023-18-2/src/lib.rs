use std::ops::{RangeFrom, RangeFull, RangeToInclusive, Sub};

enum Range<T> {
    Full(RangeFull),
    RangeToInclusive(RangeToInclusive<T>),
    RangeFrom(RangeFrom<T>),
    RangeInclusive(std::ops::RangeInclusive<T>),
}

impl Range<i64> {
    fn size(&self) -> Option<usize> {
        match self {
            Range::RangeInclusive(range) => Some((*range.end() - *range.start()).abs() as usize),
            _ => None,
        }
    }
}

struct Limits<T = i64> {
    h: Range<T>,
    v: Range<T>,
}

trait Area {
    fn area(&self) -> Option<usize>;
}

impl Area for Limits<i64> {
    fn area(&self) -> Option<usize> {
        self.v
            .size()
            .zip(self.h.size())
            .map(|(height, width)| height * width)
    }
}

enum Region {
    // Unknown
    U(Limits),
    // Left of path
    L(Limits),
    // Right of path
    R(Limits),
}

impl Area for Region {
    fn area(&self) -> Option<usize> {
        match self {
            Region::U(limits) | Region::L(limits) | Region::R(limits) => limits.area(),
        }
    }
}

enum Instruction {
    R(usize),
    U(usize),
    L(usize),
    D(usize),
}

struct Point {
    x: i64,
    y: i64,
}

struct PathSegment {
    from: Point,
    instruction: Instruction,
}

trait RegionSplitter {
    fn split(self) -> Vec<Region>;
}

impl RegionSplitter for Region {
    fn split(self) -> Vec<Region> {
        todo! {}
    }
}

impl RegionSplitter for Vec<Region> {
    fn split(self) -> Vec<Region> {
        self.into_iter()
            .map(RegionSplitter::split)
            .flatten()
            .collect()
    }
}

pub fn cubic_metres_of_lava(it: impl Iterator<Item = String>) -> usize {
    usize::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};
        assert_eq!(
            cubic_metres_of_lava(example.lines().map(String::from)),
            952408144115
        );
    }
}
