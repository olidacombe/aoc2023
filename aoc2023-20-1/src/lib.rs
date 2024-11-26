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

pub fn low_pulses_times_high_pulses_1k(it: impl Iterator<Item = String>) -> usize {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut edge_queue = Vec::new();
    for line in it {
        let NodeSpec {
            name,
            node,
            output_names,
        } = NodeSpec::parse(line.as_str());
        nodes.insert(name.clone(), node);
        for output in output_names {
            edge_queue.push((name.clone(), output));
        }
    }
    for (from, to) in edge_queue {
        println!("{from} -> {to}");
        let to = nodes.get(&to).unwrap();
        let from = nodes.get(&from).unwrap();
        from.borrow_mut().connect_output(to.clone());
    }
    dbg!(&nodes);
    usize::default()
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
    name: String,
    node: Node,
    output_names: NodeNames,
}

impl NodeSpec {
    fn parse(input: &str) -> Self {
        let (_, (Named(name, node), output_names)) =
            separated_pair(Named::<Node>::parse, tag(" -> "), NodeNames::parse)(input).unwrap();
        Self {
            name,
            node,
            output_names,
        }
    }
}

impl Parse for Named<Node> {
    fn parse(input: &str) -> IResult<&str, Self> {
        if let Ok((rest, Named(name, flipflop))) = Named::<FlipFlop>::parse(input) {
            return Ok((rest, Named(name, Rc::new(RefCell::new(flipflop)))));
        }
        if let Ok((rest, Named(name, conjunction))) = Named::<Conjunction>::parse(input) {
            return Ok((rest, Named(name, Rc::new(RefCell::new(conjunction)))));
        }
        if let Ok((rest, Named(name, broadcaster))) = Named::<Broadcaster>::parse(input) {
            return Ok((rest, Named(name, Rc::new(RefCell::new(broadcaster)))));
        }
        todo!("handle parse fail here");
    }
}

impl Outputs {
    fn push(&mut self, output: Node) {
        self.0.push(output);
    }
}

impl<'a> IntoIterator for &'a Outputs {
    type Item = &'a Node;
    type IntoIter = std::slice::Iter<'a, Node>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

struct Pulse {
    value: bool,
    destination: Node,
}

trait Module: std::fmt::Debug {
    fn connect_input(&mut self, name: &str) {
        // only does something for Conjunction
    }
    // typically add a pointer to an internal list
    fn connect_output(&mut self, output: Node);
    fn outputs(&self) -> &Outputs;
    fn process_input_pulse(&mut self, from: &str, pulse: bool) {}
    fn compute_pulse(&self) -> bool;
    fn send_output_pulses(&self) {
        let pulse = self.compute_pulse();
        for output in self.outputs() {}
    }
}

struct Named<T>(String, T);

#[derive(Debug, Default)]
struct Broadcaster {
    outputs: Outputs,
}

impl Parse for Named<Broadcaster> {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, name) = tag("broadcaster")(input)?;
        Ok((rest, Named(name.into(), Broadcaster::default())))
    }
}

impl Module for Broadcaster {
    fn connect_output(&mut self, output: Node) {
        todo!()
    }
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn process_input_pulse(&mut self, from: &str, pulse: bool) {
        todo!()
    }
    fn compute_pulse(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Default)]
struct FlipFlop {
    state: bool,
    outputs: Outputs,
}

impl Parse for Named<FlipFlop> {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::character::complete::char;
        let (rest, name) = preceded(char('&'), alpha1)(input)?;
        Ok((rest, Named(name.into(), FlipFlop::default())))
    }
}

impl Module for FlipFlop {
    fn connect_output(&mut self, output: Node) {
        todo!()
    }
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn process_input_pulse(&mut self, from: &str, pulse: bool) {
        todo!()
    }
    fn compute_pulse(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Default)]
struct Conjunction {
    outputs: Outputs,
    inputs: NodeNames,
}

impl Parse for Named<Conjunction> {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::character::complete::char;
        let (rest, name) = preceded(char('%'), alpha1)(input)?;
        Ok((rest, Named(name.into(), Conjunction::default())))
    }
}

impl Module for Conjunction {
    fn connect_output(&mut self, output: Node) {
        todo!()
    }
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn process_input_pulse(&mut self, from: &str, pulse: bool) {
        todo!()
    }
    fn compute_pulse(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example_1() {
        let example = indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "};
        assert_eq!(
            low_pulses_times_high_pulses_1k(example.lines().map(String::from)),
            32000000
        );
    }
    #[test]
    fn full_example_2() {
        let example = indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "};
        assert_eq!(
            low_pulses_times_high_pulses_1k(example.lines().map(String::from)),
            11687500
        );
    }
}
