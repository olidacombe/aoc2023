use std::ops::{Add, AddAssign};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, hex_digit1, multispace1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
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

pub fn cubic_metres_of_lava(it: impl Iterator<Item = String>) -> usize {
    let instructions = it.map(|s| Instruction::from(s.as_str()));
    // TODO fold isn't right, when we want a sequence of points
    // let points = instructions.fold(Point::default(), |point, ref instruction| {
    //     point + instruction
    // });
    // let pairs = points.tuple_windows();
    // reduce using signed trapezoid

    // let mut space = vec![Region::default()];
    // let mut point = Point::default();
    // let mut turn_score = 0;
    // while let Some(instruction) = instructions.next() {
    //     if let Some(next) = instructions.peek() {
    //         turn_score += next - &instruction;
    //     }
    //     // dbg!(&space);
    //     space = space.split(&PathSegment {
    //         from: point,
    //         instruction,
    //     });
    //     point += &instruction;
    // }
    // dbg!(&space);
    // // dbg!(&point);
    // if turn_score > 0 {
    //     space.area_right().unwrap()
    // } else {
    //     space.area_left().unwrap()
    // }
    0
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
