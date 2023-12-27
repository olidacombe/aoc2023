use std::{
    collections::{BinaryHeap, HashMap},
    ops::{Add, AddAssign},
};
use tracing::debug;

#[derive(Eq, Default, Hash, PartialEq, Clone, Debug)]
struct History(String);

impl Add<char> for &History {
    type Output = History;
    fn add(self, rhs: char) -> Self::Output {
        if !self.0.ends_with(rhs) {
            return History(rhs.to_string());
        }
        let mut inner = self.0.clone();
        inner.push(rhs);
        if inner.len() > 3 {
            inner = inner.split_off(inner.len() - 3);
        }
        History(inner)
    }
}

#[derive(Eq, PartialEq, Default, Debug)]
struct Cost(HashMap<History, usize>);

impl Cost {
    pub fn beats(&self, other: &Self) -> bool {
        // return true if self gains nothing (no new history, or better cost) from other
        other.0.iter().any(|(k, v)| {
            if let Some(reigning) = self.0.get(k) {
                return v > reigning;
            }
            false
        })
    }

    pub fn best(&self) -> Option<&usize> {
        self.0.values().min()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn zero() -> Self {
        Self(HashMap::from([(History::default(), 0)]))
    }
}

impl Add<(char, usize)> for &Cost {
    type Output = Cost;

    fn add(self, rhs: (char, usize)) -> Self::Output {
        let (ch, cost) = rhs;
        let mut inner = HashMap::new();
        let bad_endings = [
            format!("{ch}{ch}{ch}"),
            match ch {
                '>' => "<",
                '^' => "v",
                '<' => ">",
                'v' => "^",
                _ => "",
            }
            .to_string(),
        ];

        for (k, v) in self
            .0
            .iter()
            .filter(|(k, _)| !bad_endings.iter().any(|bad| k.0.ends_with(bad)))
        {
            let history = k + ch;
            let new_cost = v + cost;
            if let Some(existing) = inner.get_mut(&history) {
                if new_cost < *existing {
                    *existing = new_cost;
                }
            } else {
                inner.insert(k + ch, v + cost);
            }
        }

        Cost(inner)
    }
}

impl AddAssign<&Cost> for Cost {
    fn add_assign(&mut self, rhs: &Self) {
        for (k, v) in rhs.0.iter() {
            if let Some(existing) = self.0.get_mut(k) {
                if v < existing {
                    *existing = *v;
                }
            } else {
                self.0.insert(k.clone(), *v);
            }
        }
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ours) = self.best() {
            return other.best().cmp(&Some(ours));
        }
        std::cmp::Ordering::Less
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct State {
    cost: Cost,
    node: usize,
}

impl State {
    pub fn init(node: usize) -> Self {
        Self {
            cost: Cost::zero(),
            node,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    blocks: Vec<usize>,
    width: usize,
}

impl<I: Iterator<Item = String>> From<I> for Map {
    fn from(lines: I) -> Self {
        let mut blocks = Vec::new();
        let lines = lines.peekable();
        let mut width = 0;
        for line in lines {
            width = line.len();
            blocks.append(
                &mut line
                    .chars()
                    .map(|digit| digit.to_string().parse::<usize>().unwrap())
                    .collect(),
            );
        }

        Self { blocks, width }
    }
}

impl Map {
    pub fn minimum_heat_loss(&self) -> usize {
        let start = 0;
        let end = self.blocks.len() - 1;
        let mut dist: Vec<_> = (0..self.blocks.len()).map(|_| Cost::default()).collect();
        dist[start] = Cost::zero();
        let mut heap = BinaryHeap::new();
        heap.push(State::init(start));

        debug!("Searching {}-node graph", self.blocks.len());
        let mut greatest_visit = 0;

        while let Some(State { cost, node }) = heap.pop() {
            if node > greatest_visit {
                debug!("Reached node {node}");
                greatest_visit = node;
            }

            if node == end {
                dbg!(&cost);
                return *cost.best().unwrap();
            }

            if dist[node].beats(&cost) {
                continue;
            }

            for (direction, node) in self.neighbours(node) {
                let cost = &cost + (direction, self.blocks[node]);
                if cost.is_empty() {
                    continue;
                }
                let next = State { cost, node };

                if !dist[node].beats(&next.cost) {
                    dist[node] += &next.cost;
                    if next.node == end {
                        dbg!(&next.cost);
                    }
                    heap.push(next);
                }
            }
        }
        unreachable!("We the graph is connected!");
    }

    fn neighbours(&self, node: usize) -> Vec<(char, usize)> {
        let mut neighbours = Vec::new();
        if node >= self.width {
            neighbours.push(('^', node - self.width));
        }
        if node % self.width > 0 {
            neighbours.push(('<', node - 1));
        }
        if node < self.blocks.len() - self.width {
            neighbours.push(('v', node + self.width));
        }
        if node % self.width < self.width - 1 {
            neighbours.push(('>', node + 1));
        }
        neighbours
    }
}

pub fn minimum_heat_loss(it: impl Iterator<Item = String>) -> usize {
    let map = Map::from(it);
    map.minimum_heat_loss()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn option_ord() {
        // good because we are using max heap and None represents infinite cost
        assert!(None < Some(1));
    }

    #[test]
    fn empty_cost_ord() {
        let empty = Cost::default();
        let non_emtpy = Cost(HashMap::from([(History::default(), 1 as usize)]));
        // good because we are using max heap and None represents infinite cost
        assert!(empty < non_emtpy);
    }

    #[test]
    fn cost_ord() {
        let a = Cost(HashMap::from([
            (History::default(), 1 as usize),
            (History::default(), 2),
        ]));
        let b = Cost(HashMap::from([
            (History::default(), 2 as usize),
            (History::default(), 3),
        ]));
        // good because we are using max heap so small is preferred
        assert!(a > b);
    }

    #[test]
    fn full_example() {
        let example = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};
        assert_eq!(minimum_heat_loss(example.lines().map(String::from)), 102);
    }
}
