use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, u32},
    character::complete::{hex_digit1, multispace1},
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

enum Step {
    R(u32),
    U(u32),
    L(u32),
    D(u32),
}

fn parse_step(input: &str) -> IResult<&str, (&str, u32)> {
    separated_pair(
        alt((tag("R"), tag("U"), tag("L"), tag("D"))),
        multispace1,
        u32,
    )(input)
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let (direction, length) = parse_step(value).unwrap().1;
        match direction {
            "R" => Step::R(length),
            "U" => Step::U(length),
            "L" => Step::L(length),
            "D" => Step::D(length),
            _ => unreachable!(),
        }
    }
}

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

struct Instruction {
    step: Step,
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
        let ((direction, length), colour) = parse_instruciton(value.as_str()).unwrap().1;
        Self {
            step: Step::from(format!("{direction} {length}").as_str()),
            colour: Colour::from(colour),
        }
    }
}

pub fn cubic_meters_of_lava(it: impl Iterator<Item = String>) -> usize {
    let instructions: Vec<_> = it.map(Instruction::from).collect();
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
