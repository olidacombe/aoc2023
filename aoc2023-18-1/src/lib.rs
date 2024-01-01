use std::{
    collections::{HashMap, HashSet},
    ops::{RangeBounds, RangeFrom, RangeTo},
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    character::complete::{hex_digit1, multispace1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    H,
    V,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::H,
            "U" => Direction::V,
            "L" => Direction::H,
            "D" => Direction::V,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl From<&str> for Colour {
    fn from(value: &str) -> Self {
        let r = u8::from_str_radix(&value[..2], 16).unwrap();
        let g = u8::from_str_radix(&value[2..4], 16).unwrap();
        let b = u8::from_str_radix(&value[4..6], 16).unwrap();
        Self { r, g, b }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: i64,
    colour: Colour,
}

fn parse_instruciton(input: &str) -> IResult<&str, ((&str, &str), &str)> {
    separated_pair(
        separated_pair(alpha1, multispace1, digit1),
        multispace1,
        delimited(tag("(#"), hex_digit1, tag(")")),
    )(input)
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let ((direction, count), colour) = parse_instruciton(value.as_str()).unwrap().1;
        let mut count = count.parse().unwrap();
        match direction {
            "L" | "U" => {
                count *= -1;
            }
            _ => {}
        }
        Self {
            direction: Direction::from(direction),
            count,
            colour: Colour::from(colour),
        }
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

enum Range {
    From(RangeFrom<i64>),
    To(RangeTo<i64>),
}

impl RangeBounds<i64> for Range {
    fn start_bound(&self) -> std::ops::Bound<&i64> {
        match self {
            Self::From(r) => r.start_bound(),
            Self::To(r) => r.start_bound(),
        }
    }
    fn end_bound(&self) -> std::ops::Bound<&i64> {
        match self {
            Self::From(r) => r.end_bound(),
            Self::To(r) => r.end_bound(),
        }
    }
}

impl From<RangeFrom<i64>> for Range {
    fn from(value: RangeFrom<i64>) -> Self {
        Self::From(value)
    }
}

impl From<RangeTo<i64>> for Range {
    fn from(value: RangeTo<i64>) -> Self {
        Self::To(value)
    }
}

struct Rover {
    location: Point,
    /// row-size ranges adding 1
    exterior_plus: HashMap<i64, Vec<Range>>,
    /// row-size ranges adding -1
    exterior_minus: HashMap<i64, Vec<Range>>,
    path: HashSet<Point>,
}

impl Default for Rover {
    fn default() -> Self {
        Self {
            location: Point::default(),
            exterior_plus: HashMap::default(),
            exterior_minus: HashMap::default(),
            path: HashSet::from([Point::default()]),
        }
    }
}

impl Rover {
    fn is_exterior(&self, point: Point) -> bool {
        !self.path.contains(&point)
            && self.exterior_plus[&point.y]
                .iter()
                .filter(|range| range.contains(&point.x))
                .count()
                - self.exterior_minus[&point.y]
                    .iter()
                    .filter(|range| range.contains(&point.x))
                    .count()
                == 0
    }

    pub fn rove(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::H => {
                self.location.x += instruction.count;
            }
            Direction::V => {
                self.location.y += instruction.count;
                let plus = self
                    .exterior_plus
                    .entry(self.location.y)
                    .or_insert_with(Vec::default);
                let minus = self
                    .exterior_minus
                    .entry(self.location.y)
                    .or_insert_with(Vec::default);
                if instruction.count > 0 {
                    plus.push((self.location.x..).into());
                    minus.push((..self.location.x).into());
                } else {
                    plus.push((..self.location.x).into());
                    minus.push((self.location.x..).into());
                }
            }
        }
        self.path.insert(self.location);
    }

    fn origin(&self) -> Point {
        // todo
        (0, 0).into()
    }
}

pub fn cubic_meters_of_lava(it: impl Iterator<Item = String>) -> usize {
    let instructions: Vec<_> = it.map(Instruction::from).collect();
    dbg!(&instructions);
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
        assert_eq!(cubic_meters_of_lava(example.lines().map(String::from)), 62);
    }
}
