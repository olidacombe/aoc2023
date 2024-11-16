use std::{
    cmp::Ordering,
    ffi::IntoStringError,
    ops::{Add, Mul},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, hex_digit1, multispace1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone, PartialEq)]
enum Rotation {
    Clockwise,
    CounterClockwise,
    Colinear,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    R(usize),
    U(usize),
    L(usize),
    D(usize),
}

impl Instruction {
    fn length(&self) -> &usize {
        match self {
            Self::R(len) => len,
            Self::U(len) => len,
            Self::L(len) => len,
            Self::D(len) => len,
        }
    }
}

impl Mul for &Instruction {
    type Output = Rotation;

    fn mul(self, rhs: Self) -> Self::Output {
        use Instruction::{D, L, R, U};
        use Rotation::{Clockwise, Colinear, CounterClockwise};
        match self {
            R(_) => match rhs {
                D(_) => Clockwise,
                U(_) => CounterClockwise,
                _ => Colinear,
            },
            U(_) => match rhs {
                R(_) => Clockwise,
                L(_) => CounterClockwise,
                _ => Colinear,
            },
            L(_) => match rhs {
                U(_) => Clockwise,
                D(_) => CounterClockwise,
                _ => Colinear,
            },
            D(_) => match rhs {
                L(_) => Clockwise,
                R(_) => CounterClockwise,
                _ => Colinear,
            },
        }
    }
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

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Add<&Instruction> for Point {
    type Output = Self;
    fn add(mut self, rhs: &Instruction) -> Self::Output {
        match rhs {
            Instruction::R(x) => self.x += *x as i64,
            Instruction::U(y) => self.y -= *y as i64,
            Instruction::L(x) => self.x -= *x as i64,
            Instruction::D(y) => self.y += *y as i64,
        };
        self
    }
}

impl Mul for &Point {
    type Output = i64;

    fn mul(self, rhs: Self) -> Self::Output {
        // Note this is "double what we want"
        (rhs.y + self.y) * (rhs.x - self.x)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct RotationCounter {
    l: usize,
    r: usize,
    consecutive_l: usize,
    consecutive_r: usize,
}

impl RotationCounter {
    fn rotation_bonus(&self) -> usize {
        match self.l.cmp(&self.r) {
            Ordering::Greater => self.consecutive_l,
            Ordering::Less => self.consecutive_r,
            _ => 0,
        }
    }
}

pub fn cubic_metres_of_lava(it: impl Iterator<Item = String>) -> usize {
    let instructions: Vec<_> = it.map(|s| Instruction::from(s.as_str())).collect();
    let rotations = instructions
        .iter()
        .circular_tuple_windows()
        .map(|(i1, i2)| i1 * i2);
    let rotation_scores = rotations.circular_tuple_windows().fold(
        RotationCounter::default(),
        |mut acc, (prev, next)| match prev {
            Rotation::Colinear => acc,
            Rotation::Clockwise => {
                acc.r += 1;
                if next == prev {
                    acc.consecutive_r += 1;
                }
                acc
            }
            Rotation::CounterClockwise => {
                acc.l += 1;
                if next == prev {
                    acc.consecutive_l += 1;
                }
                acc
            }
        },
    );
    let double_length_score = instructions
        .iter()
        .fold(0, |acc, instruction| acc + instruction.length());
    // convert instructions to a sequence of points
    let points: Vec<Point> = instructions
        .iter()
        .scan(Point::default(), |point, instruction| {
            Some(*point + instruction)
        })
        .collect();
    // iterate over pairs of points
    let pairs = points.into_iter().circular_tuple_windows();
    // reduce using signed trapezoid
    let double_internal_area = pairs
        .fold(0, |area, (ref p, ref q)| area + p * q)
        .unsigned_abs() as usize;

    dbg!(rotation_scores);
    dbg!(rotation_scores.rotation_bonus());

    let mut total_score = 2 * double_internal_area;
    total_score += 2 * double_length_score;
    total_score += rotation_scores.rotation_bonus();

    total_score / 4
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
