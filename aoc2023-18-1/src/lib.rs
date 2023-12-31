use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, u32},
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

struct Rover {
    x: usize,
    y: usize,
    boundary_stack: Vec<usize>,
}

impl Rover {
    pub fn mv(&mut self, instruction: Instruction) {}
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
