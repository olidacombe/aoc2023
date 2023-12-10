//! ```
//! 1 Zs at 14363 (0)
//! 2 Zs at 847417 (0)
//! 3 Zs at 51692437 (0)
//! ```
use rayon::prelude::*;
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

#[derive(Debug)]
struct Instructions(Vec<Direction>);

impl Instructions {
    pub fn parse(input: &str) -> Self {
        Self(many0(Direction::parse)(input).unwrap().1)
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Direction)> {
        self.0.iter().enumerate().cycle()
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

#[derive(Debug)]
struct NodeFollower<'a> {
    pub node: &'a str,
    counter: usize,
    mu_lambda: Option<(usize, usize)>,
    seen: HashMap<(&'a str, usize), usize>,
    step: usize,
    zs: Vec<usize>,
}

impl<'a> NodeFollower<'a> {
    pub fn new<S: AsRef<str>>(node: &'a S) -> Self {
        Self {
            counter: 0,
            mu_lambda: None,
            node: node.as_ref(),
            seen: HashMap::from_iter([((node.as_ref(), 0), 0)]),
            step: 0,
            zs: Vec::new(),
        }
    }

    /// returns true if this follower has detected the node path's cycle
    pub fn cycling(&self) -> bool {
        self.mu_lambda.is_some()
    }

    pub fn is_z(&self) -> bool {
        self.node.ends_with("Z")
    }

    pub fn z_test(&self, candidate: usize) -> bool {
        let (mu, lambda) = self.mu_lambda.unwrap();
        let candidate = (candidate - mu) % lambda;
        self.zs.iter().find(|v| *v == &candidate).is_some()
    }

    pub fn update<S: AsRef<str>>(&mut self, node: &'a S, step: usize) {
        self.counter += 1;
        let node = node.as_ref();
        self.node = node;
        self.step = step;
        if self.cycling() {
            return;
        }
        if let Some(mu) = self.seen.get(&(node, step)) {
            // we have found cycle!
            self.mu_lambda = Some((*mu, self.counter - mu));
            self.zs.retain(|i| i >= &mu);
            self.zs.iter_mut().for_each(|i| *i -= mu);
            self.seen.clear();
            dbg!(&self.zs);
        } else {
            if self.is_z() {
                self.zs.push(self.counter);
            }
            self.seen.insert((self.node, step), self.counter);
        }
    }
}

impl<'a> IntoIterator for NodeFollower<'a> {
    type Item = usize;
    type IntoIter = CycleIter;

    fn into_iter(self) -> Self::IntoIter {
        let (mu, lambda) = self.mu_lambda.unwrap();
        CycleIter {
            current: mu,
            lambda,
            zs: self.zs,
            z_idx: 0,
        }
    }
}

struct CycleIter {
    current: usize,
    lambda: usize,
    zs: Vec<usize>,
    z_idx: usize,
}

impl Iterator for CycleIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.current + self.zs[self.z_idx];
        self.z_idx += 1;
        if self.z_idx >= self.zs.len() {
            self.z_idx = 0;
            self.current += self.lambda;
        }
        Some(v)
    }
}

pub fn count_steps(mut it: impl Iterator<Item = String>) -> u64 {
    let instructions = Instructions::parse(it.next().unwrap().as_str());
    it.next(); // skip a blank line
    let (graph, initial_nodes) = read_graph(it);
    let mut nodes: Vec<NodeFollower> = initial_nodes.iter().map(NodeFollower::new).collect();
    println!("Directions len: {}", &instructions.0.len());
    println!("Nodes len: {}", &graph.keys().len());
    let mut steps = 0;
    for (instruction_num, turning) in instructions.iter() {
        if nodes.iter().all(|n| n.is_z()) {
            // we got pretty lucky
            return steps;
        }
        if nodes.iter().all(|n| n.cycling()) {
            // stop brute-force running, we've seen all we need for each node path
            break;
        }
        nodes.par_iter_mut().for_each(|node| {
            node.update(
                match turning {
                    Direction::Left => &graph[node.node].left,
                    Direction::Right => &graph[node.node].right,
                },
                instruction_num,
            )
        });
        steps += 1;
    }

    let mut hit_count_record = 0;
    for idx in nodes.pop().unwrap() {
        let hit_count = nodes.iter().filter(|n| n.z_test(idx)).count();
        if hit_count > hit_count_record {
            hit_count_record = hit_count;
            println!("{hit_count} hits at {idx}");
        }
        if nodes.iter().all(|n| n.z_test(idx)) {
            return idx as u64;
        }
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
