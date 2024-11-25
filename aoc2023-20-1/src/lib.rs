use std::rc::Rc;

pub fn low_pulses_times_high_pulses_1k(mut it: impl Iterator<Item = String>) -> usize {
    usize::default()
}

type Output = Rc<dyn Module>;
struct Outputs(Vec<Output>);

impl Outputs {
    fn push(&mut self, output: Output) {
        self.0.push(output);
    }
}

impl<'a> IntoIterator for &'a Outputs {
    type Item = &'a Output;
    type IntoIter = std::slice::Iter<'a, Output>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

trait Module {
    fn connect_input(&mut self, name: &str) {
        // only does something for Conjunction
    }
    // typically add a pointer to an internal list
    fn connect_output(&mut self, output: Output);
    fn process_input_pulse(&mut self, from: &str, pulse: bool) {}
    fn send_output_pulses(&self);
}

struct Broadcaster {
    outputs: Outputs,
}

impl Module for Broadcaster {
    fn connect_output(&mut self, output: Output) {
        self.outputs.push(output);
    }
    fn send_output_pulses(&self) {
        for output in &self.outputs {
            output.process_input_pulse("broadcaster", false);
        }
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
