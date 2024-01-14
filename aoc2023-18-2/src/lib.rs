use std::ops::{
    AddAssign, Bound, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeToInclusive,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, hex_digit1, multispace1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Range<T = i64> {
    Full(RangeFull),
    RangeToInclusive(RangeToInclusive<T>),
    RangeFrom(RangeFrom<T>),
    RangeInclusive(RangeInclusive<T>),
}

impl<T> Default for Range<T> {
    fn default() -> Self {
        Self::Full(RangeFull::default())
    }
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        match self {
            Self::Full(range) => range.start_bound(),
            Self::RangeToInclusive(range) => range.start_bound(),
            Self::RangeFrom(range) => range.start_bound(),
            Self::RangeInclusive(range) => range.start_bound(),
        }
    }
    fn end_bound(&self) -> Bound<&T> {
        match self {
            Self::Full(range) => range.end_bound(),
            Self::RangeToInclusive(range) => range.end_bound(),
            Self::RangeFrom(range) => range.end_bound(),
            Self::RangeInclusive(range) => range.end_bound(),
        }
    }
}

trait Split {
    type T;
    fn split_v(&self, at: Self::T) -> (Self, Self)
    where
        Self: Sized;

    fn split_h(&self, at: Self::T) -> (Self, Self)
    where
        Self: Sized;
}

impl Range<i64> {
    fn split(&self, at: i64) -> (Self, Self) {
        match self {
            Self::Full(_) => (
                Self::RangeToInclusive(RangeToInclusive { end: at }),
                Self::RangeFrom(RangeFrom { start: at }),
            ),
            Self::RangeToInclusive(range) => (
                Self::RangeToInclusive(RangeToInclusive { end: at }),
                Self::RangeInclusive(
                    at..=match range.end_bound() {
                        Bound::Included(end) => *end,
                        _ => unreachable!(),
                    },
                ),
            ),
            Self::RangeFrom(range) => (
                Self::RangeInclusive(
                    match range.start_bound() {
                        Bound::Included(start) => *start,
                        _ => unreachable!(),
                    }..=at,
                ),
                Self::RangeFrom(at..),
            ),
            Self::RangeInclusive(range) => (
                Self::RangeInclusive(
                    match range.start_bound() {
                        Bound::Included(start) => *start,
                        _ => unreachable!(),
                    }..=at,
                ),
                Self::RangeInclusive(
                    at..=match range.end_bound() {
                        Bound::Included(end) => *end,
                        _ => unreachable!(),
                    },
                ),
            ),
        }
    }

    fn size(&self) -> Option<usize> {
        match self {
            Range::RangeInclusive(range) => Some((*range.end() - *range.start()).abs() as usize),
            _ => None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Limits<T = i64> {
    h: Range<T>,
    v: Range<T>,
}

impl Split for Limits {
    type T = i64;
    fn split_v(&self, at: Self::T) -> (Self, Self) {
        let (lower, upper) = self.v.split(at);
        (
            Self {
                h: self.h.clone(),
                v: lower,
            },
            Self {
                h: self.h.clone(),
                v: upper,
            },
        )
    }

    fn split_h(&self, at: Self::T) -> (Self, Self)
    where
        Self: Sized,
    {
        let (lower, upper) = self.h.split(at);
        (
            Self {
                h: lower,
                v: self.v.clone(),
            },
            Self {
                h: upper,
                v: self.v.clone(),
            },
        )
    }
}

trait Transpose {
    fn transposed(self) -> Self;
}

trait Area {
    fn area_left(&self) -> Option<usize> {
        None
    }
    fn area_right(&self) -> Option<usize> {
        None
    }
    fn area(&self) -> Option<usize> {
        self.area_left().or_else(|| self.area_right())
    }
}

impl Area for Limits<i64> {
    fn area(&self) -> Option<usize> {
        self.v
            .size()
            .zip(self.h.size())
            .map(|(height, width)| height * width)
    }
}

impl<T> Transpose for Limits<T> {
    fn transposed(self) -> Self {
        let Self { h, v } = self;
        Self { h: v, v: h }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Region {
    // Unknown
    U(Limits),
    // Left of path
    L(Limits),
    // Right of path
    R(Limits),
}

impl Default for Region {
    fn default() -> Self {
        Self::U(Limits::default())
    }
}

impl Split for Region {
    type T = i64;
    fn split_v(&self, at: Self::T) -> (Self, Self)
    where
        Self: Sized,
    {
        match self {
            Self::U(limits) => {
                let (lower, upper) = limits.split_v(at);
                (Self::U(lower), Self::U(upper))
            }
            Self::L(limits) => {
                let (lower, upper) = limits.split_v(at);
                (Self::L(lower), Self::L(upper))
            }
            Self::R(limits) => {
                let (lower, upper) = limits.split_v(at);
                (Self::R(lower), Self::R(upper))
            }
        }
    }
    /// unconditionally split L|R
    fn split_h(&self, at: Self::T) -> (Self, Self)
    where
        Self: Sized,
    {
        match self {
            Self::U(limits) => {
                let (lower, upper) = limits.split_h(at);
                (Self::L(upper), Self::R(lower))
            }
            Self::L(limits) => {
                let (lower, upper) = limits.split_h(at);
                (Self::L(upper), Self::R(lower))
            }
            Self::R(limits) => {
                let (lower, upper) = limits.split_h(at);
                (Self::L(upper), Self::R(lower))
            }
        }
    }
}

impl Region {
    pub fn limits(&self) -> &Limits {
        match self {
            Self::U(limits) | Self::L(limits) | Self::R(limits) => limits,
        }
    }
}

trait Mirror {
    fn mirrored(self) -> Self;
}

impl Area for Region {
    fn area(&self) -> Option<usize> {
        match self {
            Region::U(limits) | Region::L(limits) | Region::R(limits) => limits.area(),
        }
    }
}

impl Mirror for Region {
    fn mirrored(self) -> Self {
        match self {
            Region::U(limits) => Region::U(limits),
            Region::L(limits) => Region::R(limits),
            Region::R(limits) => Region::L(limits),
        }
    }
}

impl Transpose for Region {
    fn transposed(self) -> Self {
        match self {
            Region::U(limits) => Region::U(limits.transposed()),
            Region::R(limits) => Region::R(limits.transposed()),
            Region::L(limits) => Region::L(limits.transposed()),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    R(usize),
    U(usize),
    L(usize),
    D(usize),
}

fn parse_instruction(input: &str) -> IResult<&str, ((&str, &str), &str)> {
    separated_pair(
        separated_pair(alpha1, multispace1, digit1),
        multispace1,
        delimited(tag("(#"), hex_digit1, tag(")")),
    )(input)
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let ((_, _), hex) = parse_instruction(input).unwrap().1;
        let (length, direction) = hex.split_at(hex.len() - 1);
        let length = usize::from_str_radix(length, 16).unwrap();
        match direction {
            "0" => Self::R(length),
            "1" => Self::D(length),
            "2" => Self::L(length),
            "3" => Self::U(length),
            _ => {
                unimplemented!();
            }
        }
    }
}

impl Transpose for Instruction {
    fn transposed(self) -> Self {
        match self {
            Instruction::R(len) => Instruction::D(len),
            Instruction::U(len) => Instruction::L(len),
            Instruction::L(len) => Instruction::U(len),
            Instruction::D(len) => Instruction::R(len),
        }
    }
}

impl Mirror for Instruction {
    fn mirrored(self) -> Self {
        match self {
            Instruction::R(len) => Instruction::L(len),
            Instruction::U(len) => Instruction::D(len),
            Instruction::L(len) => Instruction::R(len),
            Instruction::D(len) => Instruction::U(len),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl AddAssign<&Instruction> for Point {
    fn add_assign(&mut self, rhs: &Instruction) {
        match rhs {
            Instruction::R(x) => self.x += *x as i64,
            Instruction::U(y) => self.y -= *y as i64,
            Instruction::L(x) => self.x -= *x as i64,
            Instruction::D(y) => self.y += *y as i64,
        }
    }
}

impl Transpose for Point {
    fn transposed(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

#[derive(Clone, Copy)]
struct PathSegment {
    from: Point,
    instruction: Instruction,
}

impl Mirror for PathSegment {
    fn mirrored(self) -> Self {
        Self {
            from: self.from,
            instruction: self.instruction.mirrored(),
        }
    }
}

impl Transpose for PathSegment {
    fn transposed(self) -> Self {
        Self {
            from: self.from.transposed(),
            instruction: self.instruction.transposed(),
        }
    }
}

trait RegionSplitter {
    fn split(self, segment: &PathSegment) -> Vec<Region>;
}

impl RegionSplitter for Region {
    fn split(self, segment: &PathSegment) -> Vec<Region> {
        match segment.instruction {
            Instruction::R(_) | Instruction::L(_) => {
                self.transposed().split(&segment.transposed()).transposed()
            }
            Instruction::U(_) => self.mirrored().split(&segment.mirrored()).mirrored(),
            Instruction::D(count) => {
                if !self.limits().h.contains(&segment.from.x) {
                    return vec![self];
                }
                let start = segment.from.y;
                let end = segment.from.y + count as i64;
                if self.limits().v.contains(&start) && self.limits().v.contains(&end) {
                    let (lower, upper) = self.split_v(start);
                    let (mid, upper) = upper.split_v(end);
                    let (l, r) = mid.split_h(segment.from.x);
                    return vec![lower, l, r, upper];
                }
                if self.limits().v.contains(&start) {
                    let (lower, upper) = self.split_v(start);
                    let (l, r) = upper.split_h(segment.from.x);
                    return vec![lower, l, r];
                }
                if self.limits().v.contains(&end) {
                    let (lower, upper) = self.split_v(end);
                    let (l, r) = lower.split_h(segment.from.x);
                    return vec![l, r, upper];
                }
                vec![self]
            }
        }
    }
}

impl Area for Vec<Region> {
    fn area_left(&self) -> Option<usize> {
        let mut total = 0;
        for region in self.iter().filter(|r| match r {
            Region::L(_) => true,
            _ => false,
        }) {
            match region.area() {
                Some(a) => total += a,
                None => {
                    return None;
                }
            }
        }
        Some(total)
    }

    fn area_right(&self) -> Option<usize> {
        let mut total = 0;
        for region in self.iter().filter(|r| match r {
            Region::R(_) => true,
            _ => false,
        }) {
            match region.area() {
                Some(a) => total += a,
                None => {
                    return None;
                }
            }
        }
        Some(total)
    }
}

impl RegionSplitter for Vec<Region> {
    fn split(self, segment: &PathSegment) -> Vec<Region> {
        self.into_iter()
            .map(|region| region.split(segment))
            .flatten()
            .collect()
    }
}

impl Mirror for Vec<Region> {
    fn mirrored(self) -> Self {
        self.into_iter().map(Mirror::mirrored).collect()
    }
}

impl Transpose for Vec<Region> {
    fn transposed(self) -> Self {
        self.into_iter().map(Transpose::transposed).collect()
    }
}

pub fn cubic_metres_of_lava(it: impl Iterator<Item = String>) -> usize {
    let instructions = it.map(|s| Instruction::from(s.as_str()));
    let mut space = vec![Region::default()];
    let mut point = Point::default();
    for instruction in instructions {
        // dbg!(&space);
        space = space.split(&PathSegment {
            from: point,
            instruction,
        });
        point += &instruction;
    }
    // dbg!(&space);
    // dbg!(&point);
    space.area().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn right() {
        let region = Region::default();
        let path_segment = PathSegment {
            from: Point::default(),
            instruction: Instruction::R(2),
        };
        assert_eq!(
            region.split(&path_segment),
            vec![
                Region::U(Limits {
                    h: Range::RangeToInclusive(..=0),
                    v: Range::Full(..)
                }),
                Region::L(Limits {
                    h: Range::RangeInclusive(0..=2),
                    v: Range::RangeToInclusive(..=0)
                }),
                Region::R(Limits {
                    h: Range::RangeInclusive(0..=2),
                    v: Range::RangeFrom(0..)
                }),
                Region::U(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::Full(..)
                })
            ]
        );
    }

    #[test]
    fn down() {
        let region = Region::default();
        let path_segment = PathSegment {
            from: Point::default(),
            instruction: Instruction::D(2),
        };
        assert_eq!(
            region.split(&path_segment),
            vec![
                Region::U(Limits {
                    h: Range::Full(..),
                    v: Range::RangeToInclusive(..=0),
                }),
                Region::L(Limits {
                    h: Range::RangeFrom(0..),
                    v: Range::RangeInclusive(0..=2),
                }),
                Region::R(Limits {
                    h: Range::RangeToInclusive(..=0),
                    v: Range::RangeInclusive(0..=2),
                }),
                Region::U(Limits {
                    h: Range::Full(..),
                    v: Range::RangeFrom(2..),
                })
            ]
        );
    }

    #[test]
    fn left() {
        let region = Region::default();
        let path_segment = PathSegment {
            from: Point::default(),
            instruction: Instruction::L(2),
        };
        assert_eq!(
            region.split(&path_segment),
            vec![
                Region::U(Limits {
                    h: Range::RangeToInclusive(..=-2),
                    v: Range::Full(..)
                }),
                Region::L(Limits {
                    h: Range::RangeInclusive(-2..=0),
                    v: Range::RangeFrom(0..)
                }),
                Region::R(Limits {
                    h: Range::RangeInclusive(-2..=0),
                    v: Range::RangeToInclusive(..=0)
                }),
                Region::U(Limits {
                    h: Range::RangeFrom(-2..),
                    v: Range::Full(..)
                })
            ]
        );
    }

    #[test]
    fn up() {
        let region = Region::default();
        let path_segment = PathSegment {
            from: Point::default(),
            instruction: Instruction::U(2),
        };
        assert_eq!(
            region.split(&path_segment),
            vec![
                Region::U(Limits {
                    h: Range::Full(..),
                    v: Range::RangeToInclusive(..=-2),
                }),
                Region::L(Limits {
                    h: Range::RangeToInclusive(..=0),
                    v: Range::RangeInclusive(-2..=0),
                }),
                Region::R(Limits {
                    h: Range::RangeFrom(0..),
                    v: Range::RangeInclusive(-2..=0),
                }),
                Region::U(Limits {
                    h: Range::Full(..),
                    v: Range::RangeFrom(-2..),
                })
            ]
        );
    }

    #[test]
    fn problem_1() {
        let region = Region::U(Limits {
            h: Range::RangeFrom(2..),
            v: Range::Full(..),
        });
        let path_segment = PathSegment {
            from: Point { x: 2, y: 0 },
            instruction: Instruction::D(2),
        };
        assert_eq!(
            region.split(&path_segment),
            vec![
                Region::U(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::RangeToInclusive(..=0)
                }),
                Region::L(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::RangeInclusive(0..=2)
                }),
                Region::U(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::RangeFrom(2..)
                }),
            ]
        );
    }

    #[test]
    fn problem_2() {
        let region = Region::R(Limits {
            h: Range::RangeInclusive(0..=2),
            v: Range::RangeFrom(0..),
        });
        let path_segment = PathSegment {
            from: Point { x: 2, y: 0 },
            instruction: Instruction::D(2),
        };
        assert_eq!(
            region.split(&path_segment),
            vec![
                Region::R(Limits {
                    h: Range::RangeInclusive(0..=2),
                    v: Range::RangeInclusive(0..=2)
                }),
                Region::R(Limits {
                    h: Range::RangeInclusive(0..=2),
                    v: Range::RangeFrom(2..)
                }),
            ]
        );
    }

    #[test]
    fn h_2() {
        let space = vec![
            Region::U(Limits {
                h: Range::RangeToInclusive(..=0),
                v: Range::Full(..),
            }),
            Region::L(Limits {
                h: Range::RangeInclusive(0..=2),
                v: Range::RangeToInclusive(..=0),
            }),
            Region::R(Limits {
                h: Range::RangeInclusive(0..=2),
                v: Range::RangeFrom(0..),
            }),
            Region::U(Limits {
                h: Range::RangeFrom(2..),
                v: Range::Full(..),
            }),
        ];
        let path_segment = PathSegment {
            from: Point { x: 2, y: 0 },
            instruction: Instruction::D(2),
        };
        assert_eq!(
            space.split(&path_segment),
            vec![
                Region::U(Limits {
                    h: Range::RangeToInclusive(..=0),
                    v: Range::Full(..)
                }),
                Region::L(Limits {
                    h: Range::RangeInclusive(0..=2),
                    v: Range::RangeToInclusive(..=0)
                }),
                Region::R(Limits {
                    h: Range::RangeInclusive(0..=2),
                    v: Range::RangeInclusive(0..=2)
                }),
                Region::L(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::RangeInclusive(0..=2)
                }),
                Region::R(Limits {
                    h: Range::RangeInclusive(0..=2),
                    v: Range::RangeFrom(2..)
                }),
                Region::U(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::RangeToInclusive(..=0)
                }),
                Region::L(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::RangeInclusive(0..=2)
                }),
                Region::U(Limits {
                    h: Range::RangeFrom(2..),
                    v: Range::RangeFrom(2..)
                }),
            ]
        );
    }

    #[test]
    fn mini_example_1() {
        let example = indoc! {"
            R 2 (#000020)
            D 2 (#000021)
            L 2 (#000022)
            U 2 (#000023)
        "};
        assert_eq!(cubic_metres_of_lava(example.lines().map(String::from)), 4);
    }

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
