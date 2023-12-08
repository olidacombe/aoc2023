#![feature(lazy_cell)]
use std::{cell::LazyCell, sync::{LazyLock, Mutex}};

use elsa::FrozenIndexSet;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char},
    combinator::value,
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};
use petgraph::prelude::UnGraphMap;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
}

struct NodeDef<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for NodeDef<'a> {
    fn from(input: &'a str) -> Self {
        Self::parse(input)
    }
}

impl<'a> NodeDef<'a> {
    fn neighbors(input: &str) -> IResult<&str, (&str, &str)> {
        let neighbors = separated_pair(alpha1, tag(", "), alpha1);
        delimited(char('('), neighbors, char(')'))(input)
    }

    pub fn parse(input: &'a str) -> Self {
        // let neighbors = separated_pair(alpha1, tag(", "), alpha1);
        // let connections = delimited(char('('), neighbors, char(')'));
        // let (_, (id, (left, right))) =
        let parsed = separated_pair(alpha1, tag(" = "), Self::neighbors)(input).unwrap();
        let (_, (id, (left, right))) = parsed;
        Self { id, left, right }
    }
}

static IDS : Mutex<LazyLock<FrozenIndexSet<String>>> = Mutex::new(LazyLock::new(||
FrozenIndexSet::new()
));

fn read_graph<'a>(it: impl Iterator<Item = String>) -> UnGraphMap<&'static str, Direction> {
    let mut graph = UnGraphMap::new();
    let ids = IDS.lock().unwrap();
    for line in it {
        let NodeDef { id, left, right } = NodeDef::from(line.as_str());
        ids.insert(id.to_string());
        ids.insert(left.to_string());
        ids.insert(right.to_string());
        let id = ids.get(id).unwrap();
        let left = ids.get(left).unwrap();
        let right = ids.get(right).unwrap();
        graph.add_edge(id, left, Direction::Left);
        graph.add_edge(id, right, Direction::Right);
    }
    graph
}

pub fn count_steps(mut it: impl Iterator<Item = String>) -> u64 {
    let instructions = Instructions::parse(it.next().unwrap().as_str());
    let graph = read_graph(it);
    u64::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example_1() {
        let example = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};
        assert_eq!(count_steps(example.lines().map(String::from)), 2);
    }

    #[test]
    fn full_example_2() {
        let example = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};
        assert_eq!(count_steps(example.lines().map(String::from)), 6);
    }
}
