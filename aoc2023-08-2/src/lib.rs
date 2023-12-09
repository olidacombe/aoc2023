use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    combinator::value,
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((value(Self::Left, tag("L")), value(Self::Right, tag("R"))))(input)
    }
}

struct Instructions(Vec<Direction>);

impl Instructions {
    pub fn parse(input: &str) -> Self {
        Self(many0(Direction::parse)(input).unwrap().1)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Direction> {
        self.0.iter().cycle()
    }
}

struct NodeDef {
    id: String,
    left: String,
    right: String,
}

impl From<&str> for NodeDef {
    fn from(input: &str) -> Self {
        Self::parse(input)
    }
}

impl NodeDef {
    fn neighbors(input: &str) -> IResult<&str, (&str, &str)> {
        let neighbors = separated_pair(alphanumeric1, tag(", "), alphanumeric1);
        delimited(char('('), neighbors, char(')'))(input)
    }

    pub fn parse(input: &str) -> Self {
        let parsed = separated_pair(alphanumeric1, tag(" = "), Self::neighbors)(input).unwrap();
        let (_, (id, (left, right))) = parsed;
        let id = id.to_string();
        let left = left.to_string();
        let right = right.to_string();
        Self { id, left, right }
    }
}

struct Neighbours {
    left: String,
    right: String,
}

fn read_graph(it: impl Iterator<Item = String>) -> (HashMap<String, Neighbours>, Vec<String>) {
    let mut map = HashMap::new();
    let mut start_nodes = Vec::new();

    for line in it {
        let NodeDef { id, left, right } = NodeDef::from(line.as_str());
        if id.ends_with("A") {
            start_nodes.push(id.clone());
        }
        map.insert(id, Neighbours { left, right });
    }
    (map, start_nodes)
}

pub fn count_steps(mut it: impl Iterator<Item = String>) -> u64 {
    let instructions = Instructions::parse(it.next().unwrap().as_str());
    it.next(); // skip a blank line
    let (graph, mut nodes) = read_graph(it);
    let mut steps = 0;
    for turning in instructions.iter() {
        if nodes.iter().all(|n| n.ends_with("Z")) {
            break;
        }
        for node in nodes.iter_mut() {
            *node = match turning {
                Direction::Left => graph[node].left.clone(),
                Direction::Right => graph[node].right.clone(),
            };
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        assert_eq!(count_steps(example.lines().map(String::from)), 6);
    }
}
