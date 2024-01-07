use std::ops::{Bound, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

#[derive(Clone)]
enum Range<T> {
    Full(RangeFull),
    RangeToInclusive(RangeToInclusive<T>),
    RangeFrom(RangeFrom<T>),
    RangeInclusive(RangeInclusive<T>),
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

impl<T> Transpose for Limits<T> {
    fn transposed(self) -> Self {
        let Self { h, v } = self;
        Self { h: v, v: h }
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
                (Self::L(lower), Self::R(upper))
            }
            Self::L(limits) => {
                let (lower, upper) = limits.split_h(at);
                (Self::L(lower), Self::R(upper))
            }
            Self::R(limits) => {
                let (lower, upper) = limits.split_h(at);
                (Self::L(lower), Self::R(upper))
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

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
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
            Instruction::U(_) => self.split(segment).mirrored(),
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
