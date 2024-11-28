use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn min_button_presses_to_trigger_rx(it: impl Iterator<Item = String>) -> usize {
    // First, set up our graph
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut edge_queue: Vec<(Node, String)> = Vec::new();
    for line in it {
        let NodeSpec { node, output_names } = NodeSpec::parse(line.as_str());
        let name = node.borrow().name().to_string();
        nodes.insert(name, node.clone());
        for output in output_names {
            edge_queue.push((node.clone(), output));
        }
    }
    for (from, to_name) in edge_queue {
        let from_name = from.borrow().name().to_string();
        let to = nodes
            .entry(to_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Sink::new(to_name.to_string()))));
        from.borrow_mut().connect_output(to.clone());
        to.borrow_mut().connect_input(&from_name);
    }

    // Now process pulses
    let mut counts = HashMap::<bool, usize>::from([(false, 0), (true, 0)]);
    let broadcaster = nodes.get("broadcaster").unwrap();
    let rx = nodes.get("rx").unwrap();
    let mut pulses = VecDeque::new();
    let mut count = 0;
    loop {
        pulses.push_back(Pulse::button(broadcaster.clone()));
        count += 1;
        while let Some(Pulse {
            value,
            origin,
            destination,
        }) = pulses.pop_front()
        {
            *counts.get_mut(&value).unwrap() += 1;
            let mut destination = destination.borrow_mut();
            destination.process_input_pulse(&origin, value);
            for pulse in destination.get_output_pulses() {
                pulses.push_back(pulse);
            }
        }
        if rx.borrow().is_on() == Some(true) {
            return count;
        }
    }
}

pub trait Parse
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;
}

type Node = Rc<RefCell<dyn Module>>;

#[derive(Debug, Default)]
struct Outputs(Vec<Node>);

impl Outputs {
    fn push(&mut self, output: Node) {
        self.0.push(output);
    }
}

impl IntoIterator for Outputs {
    type Item = Node;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Outputs {
    type Item = &'a Node;
    type IntoIter = std::slice::Iter<'a, Node>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Debug, Default)]
struct NodeNames(Vec<String>);

impl IntoIterator for NodeNames {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a NodeNames {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Parse for NodeNames {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, outputs) = separated_list1(tag(", "), alpha1)(input)?;
        Ok((rest, Self(outputs.into_iter().map(String::from).collect())))
    }
}

struct NodeSpec {
    node: Node,
    output_names: NodeNames,
}

impl NodeSpec {
    fn parse(input: &str) -> Self {
        let (_, (node, output_names)) =
            separated_pair(Node::parse, tag(" -> "), NodeNames::parse)(input).unwrap();
        Self { node, output_names }
    }
}

impl Parse for Node {
    fn parse(input: &str) -> IResult<&str, Self> {
        if let Ok((rest, flipflop)) = FlipFlop::parse(input) {
            return Ok((rest, Rc::new(RefCell::new(flipflop))));
        }
        if let Ok((rest, conjunction)) = Conjunction::parse(input) {
            return Ok((rest, Rc::new(RefCell::new(conjunction))));
        }
        if let Ok((rest, broadcaster)) = Broadcaster::parse(input) {
            return Ok((rest, Rc::new(RefCell::new(broadcaster))));
        }
        todo!("handle parse fail here");
    }
}

struct Pulse {
    value: bool,
    origin: String,
    destination: Node,
}

impl Pulse {
    fn button(broadcaster: Node) -> Self {
        Self {
            value: false,
            origin: "button".into(),
            destination: broadcaster,
        }
    }
}

trait Module: std::fmt::Debug {
    fn connect_input(&mut self, _name: &str) {
        // only does something for Conjunction
    }
    // typically add a pointer to an internal list
    fn connect_output(&mut self, output: Node);
    fn outputs(&self) -> &Outputs;
    fn process_input_pulse(&mut self, from: &str, pulse: bool);
    fn compute_pulse(&mut self) -> Option<bool>;
    fn name(&self) -> &str;
    fn get_output_pulses(&mut self) -> Vec<Pulse> {
        let Some(pulse) = self.compute_pulse() else {
            return Vec::new();
        };
        self.outputs()
            .into_iter()
            .map(|output| Pulse {
                value: pulse,
                origin: self.name().into(),
                destination: output.clone(),
            })
            .collect()
    }
    fn is_on(&self) -> Option<bool> {
        None
    }
}

#[derive(Debug, Default)]
struct Broadcaster {
    state: bool,
    outputs: Outputs,
}

impl Parse for Broadcaster {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, _) = tag("broadcaster")(input)?;
        Ok((rest, Broadcaster::default()))
    }
}

impl Module for Broadcaster {
    fn name(&self) -> &str {
        "broadcaster"
    }
    fn connect_output(&mut self, output: Node) {
        self.outputs.push(output);
    }
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn process_input_pulse(&mut self, _from: &str, pulse: bool) {
        self.state = pulse;
    }
    fn compute_pulse(&mut self) -> Option<bool> {
        Some(self.state)
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    to_send: Option<bool>,
    state: bool,
    outputs: Outputs,
}

impl FlipFlop {
    fn new(name: String) -> Self {
        Self {
            name,
            to_send: None,
            state: false,
            outputs: Outputs::default(),
        }
    }
}

impl Parse for FlipFlop {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::character::complete::char;
        let (rest, name) = preceded(char('%'), alpha1)(input)?;
        Ok((rest, FlipFlop::new(name.into())))
    }
}

impl Module for FlipFlop {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn connect_output(&mut self, output: Node) {
        self.outputs.push(output);
    }
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn process_input_pulse(&mut self, _from: &str, pulse: bool) {
        if !pulse {
            self.state = !self.state;
            self.to_send = Some(self.state);
        } else {
            self.to_send = None;
        }
    }
    fn compute_pulse(&mut self) -> Option<bool> {
        self.to_send.take()
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    outputs: Outputs,
    inputs: HashMap<String, bool>,
}

impl Conjunction {
    fn new(name: String) -> Self {
        Self {
            name,
            outputs: Outputs::default(),
            inputs: HashMap::new(),
        }
    }
}

impl Parse for Conjunction {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::character::complete::char;
        let (rest, name) = preceded(char('&'), alpha1)(input)?;
        Ok((rest, Conjunction::new(name.into())))
    }
}

impl Module for Conjunction {
    fn name(&self) -> &str {
        &self.name
    }
    fn connect_output(&mut self, output: Node) {
        self.outputs.push(output);
    }
    fn connect_input(&mut self, name: &str) {
        self.inputs.insert(name.to_string(), false);
    }
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn process_input_pulse(&mut self, from: &str, pulse: bool) {
        self.inputs.insert(from.into(), pulse);
    }
    fn compute_pulse(&mut self) -> Option<bool> {
        Some(!self.inputs.values().all(|i| *i))
    }
}

#[derive(Debug)]
struct Sink {
    name: String,
    outputs: Outputs,
    state: bool,
}

impl Sink {
    fn new(name: String) -> Self {
        Self {
            name,
            outputs: Outputs::default(),
            state: false,
        }
    }
}

impl Module for Sink {
    fn name(&self) -> &str {
        &self.name
    }
    fn connect_output(&mut self, _output: Node) {}
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn compute_pulse(&mut self) -> Option<bool> {
        None
    }
    fn process_input_pulse(&mut self, _from: &str, pulse: bool) {
        if !pulse {
            self.state = true;
        }
    }
    fn is_on(&self) -> Option<bool> {
        Some(self.state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example_2() {
        let example = indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> rx
        "};
        assert_eq!(
            min_button_presses_to_trigger_rx(example.lines().map(String::from)),
            1
        );
    }
}
