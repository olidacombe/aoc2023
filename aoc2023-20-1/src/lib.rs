use std::{
    cell::{LazyCell, RefCell},
    collections::VecDeque,
    rc::Rc,
    sync::{Arc, Mutex, OnceLock},
};

use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1, sequence::preceded,
    IResult,
};

pub fn low_pulses_times_high_pulses_1k(mut it: impl Iterator<Item = String>) -> usize {
    usize::default()
}

trait Parse
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;
}

type Node = Rc<RefCell<dyn Module>>;
// impl<M> From<M> for Node
// where
//     M: Module,
// {
//     fn from(value: M) -> Self {
//         Arc::new(RefCell::new(value))
//     }
// }

#[derive(Default)]
struct Outputs(Vec<Node>);
#[derive(Default)]
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
    output_names: Vec<String>,
}

impl Parse for Named<Node> {
    fn parse(input: &str) -> IResult<&str, Self> {
        if let Ok((rest, Named(name, broadcaster))) = Named::<Broadcaster>::parse(input) {
            return Ok((rest, Named(name, Rc::new(RefCell::new(broadcaster)))));
        }
        if let Ok((rest, Named(name, flipflop))) = Named::<FlipFlop>::parse(input) {
            return Ok((rest, Named(name, Rc::new(RefCell::new(flipflop)))));
        }
        todo!()
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

trait Module {
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

#[derive(Default)]
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

#[derive(Default)]
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

#[derive(Default)]
struct Conjunction {
    outputs: Outputs,
    inputs: NodeNames,
}

impl Parse for Named<Conjunction> {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::character::complete::char;
        let (rest, name) = preceded(char('&'), alpha1)(input)?;
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
