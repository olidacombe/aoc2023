use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, u32},
    character::complete::{hex_digit1, multispace1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    R,
    U,
    L,
    D,
}

impl Direction {
    pub fn flipped(self) -> Self {
        match self {
            Direction::R => Direction::L,
            Direction::U => Direction::D,
            Direction::L => Direction::R,
            Direction::D => Direction::U,
        }
    }

    pub fn parallel(&self, rhs: &Self) -> i8 {
        match self {
            Direction::R => match rhs {
                Direction::R => 1,
                Direction::L => -1,
                _ => 0,
            },
            Direction::U => match rhs {
                Direction::U => 1,
                Direction::D => -1,
                _ => 0,
            },
            Direction::L => match rhs {
                Direction::L => 1,
                Direction::R => -1,
                _ => 0,
            },
            Direction::D => match rhs {
                Direction::D => 1,
                Direction::U => -1,
                _ => 0,
            },
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::R,
            "U" => Direction::U,
            "L" => Direction::L,
            "D" => Direction::D,
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
    direction: Direction,
    length: usize,
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
            direction: Direction::from(direction),
            length: length.parse().unwrap(),
            colour: Colour::from(colour),
        }
    }
}

struct Rover {
    x: usize,
    y: usize,
    last_direction: Option<Direction>,
    boundary_stack: Vec<usize>,
}

impl Rover {
    fn boundary_coord(&self) -> usize {
        match self.last_direction.as_ref().unwrap() {
            Direction::U | Direction::D => self.y,
            Direction::R | Direction::L => self.x,
        }
    }

    pub fn mv(&mut self, instruction: Instruction) {
        if self.last_direction.is_none() {
            self.last_direction = Some(instruction.direction.clone());
        }
        if instruction.direction == Direction::U && instruction.length > self.y {
            if self
                .last_direction
                .as_ref()
                .unwrap()
                .parallel(&instruction.direction)
                != 0
            {
                let diff = instruction.length.abs_diff(self.y);
                self.boundary_stack.iter_mut().for_each(|i| *i = *i + diff);
                self.y += diff;
            }
        } else if instruction.direction == Direction::L && instruction.length > self.x {
            if self
                .last_direction
                .as_ref()
                .unwrap()
                .parallel(&instruction.direction)
                != 0
            {
                let diff = instruction.length.abs_diff(self.x);
                self.boundary_stack.iter_mut().for_each(|i| *i = *i + diff);
                self.x += diff;
            }
        }
        if self.last_direction.as_ref().unwrap().parallel(&instruction.direction) == 1 {
        }
        else if self.last_direction.as_ref().unwrap().parallel(&instruction.direction) == -1 {
        self.boundary_stack.push(self.boundary_coord());
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
