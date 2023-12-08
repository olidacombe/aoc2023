#![feature(lazy_cell)]
use std::sync::{LazyLock, Mutex};

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
use petgraph::prelude::{DiGraphMap};

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
    pub fn iter(&self) -> impl Iterator<Item=&Direction> {
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
        let neighbors = separated_pair(alpha1, tag(", "), alpha1);
        delimited(char('('), neighbors, char(')'))(input)
    }

    pub fn parse(input: &str) -> Self {
        let parsed = separated_pair(alpha1, tag(" = "), Self::neighbors)(input).unwrap();
        let (_, (id, (left, right))) = parsed;
        let id = id.to_string();
        let left = left.to_string();
        let right = right.to_string();
        Self { id, left, right }
    }
}

static IDS : Mutex<LazyLock<FrozenIndexSet<String>>> = Mutex::new(LazyLock::new(||
    FrozenIndexSet::new()
));

fn read_graph(it: impl Iterator<Item = String>) -> DiGraphMap<usize, Direction> {
    let mut graph = DiGraphMap::new();
    let ids = IDS.lock().unwrap();
    for line in it {
        dbg!(&line);
        let NodeDef { id, left, right } = NodeDef::from(line.as_str());
        let (id, _) =ids.insert_full(id.to_string());
        let (left, _) =ids.insert_full(left.to_string());
        let (right, _)=ids.insert_full(right.to_string());
        graph.add_edge(id, left, Direction::Left);
        graph.add_edge(id, right, Direction::Right);
        dbg!(&graph);
    }
    graph
}

pub fn count_steps(mut it: impl Iterator<Item = String>) -> u64 {
    let instructions = Instructions::parse(it.next().unwrap().as_str());
    it.next(); // skip a blank line
    let graph = read_graph(it);
    // dbg!(&graph);
    let ids = IDS.lock().unwrap();
    let (a, _) = ids.get_full("AAA").unwrap();
    let (z, _) = ids.get_full("ZZZ").unwrap();
    let mut node = a;
    let mut steps = 0;
    for turning in instructions.iter() {
        // dbg!(&turning);
        // dbg!(&node);
        if node == z {
            break
        }
node = 
    graph.edges(node).into_iter().find(|d| d.2==turning).unwrap().1;
steps +=1;
    }
    steps
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
