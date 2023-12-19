use arr_macro::arr;
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit0},
    sequence::tuple,
    IResult,
};

fn ascii_value(c: char) -> usize {
    c as usize
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, v| ((acc + ascii_value(v)) * 17) % 256)
}

#[derive(Default)]
struct LensBox(
    /// Label -> focal length
    IndexMap<String, u32>,
);

impl LensBox {
    pub fn remove(&mut self, label: &str) {
        self.0.shift_remove(label);
    }

    pub fn insert(&mut self, label: String, focal_length: u32) {
        if let Some(lens) = self.0.get_mut(label.as_str()) {
            *lens = focal_length;
        } else {
            self.0.insert(label, focal_length);
        }
    }

    pub fn power(&self, box_number: u32) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, (_, focal_length))| (1 + idx as u32) * focal_length)
            .sum::<u32>()
            * (box_number + 1)
    }
}

struct Boxes([LensBox; 256]);

impl Default for Boxes {
    fn default() -> Self {
        Self(arr![LensBox::default();256])
    }
}

impl Boxes {
    pub fn proceed(&mut self, operation: Operation) {
        match operation {
            Operation::Remove(label) => {
                let target_box = hash(label.as_str());
                self.0[target_box].remove(label.as_str());
            }
            Operation::Insert(label, focal_length) => {
                let target_box = hash(label.as_str());
                self.0[target_box].insert(label, focal_length);
            }
        }
    }

    pub fn focusing_power(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, lenses)| lenses.power(idx as u32))
            .sum()
    }
}

enum Operation {
    Remove(String),
    Insert(String, u32),
}

fn parse_entry(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((alpha1, alt((tag("-"), tag("="))), digit0))(input)
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let (label, op, focal_length) = parse_entry(value).unwrap().1;
        match op {
            "-" => Operation::Remove(label.into()),
            "=" => Operation::Insert(label.into(), focal_length.parse().unwrap()),
            _ => unreachable!("op must be \"-\" or \"=\""),
        }
    }
}

pub fn focusing_power(mut it: impl Iterator<Item = String>) -> u32 {
    let operations: Vec<Operation> = it.next().unwrap().split(",").map(Operation::from).collect();
    let mut boxes = Boxes::default();
    for operation in operations {
        boxes.proceed(operation);
    }
    boxes.focusing_power()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "};
        assert_eq!(focusing_power(example.lines().map(String::from)), 145);
    }
}
