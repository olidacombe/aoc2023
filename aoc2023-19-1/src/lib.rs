use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    multi::separated_list0,
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

pub fn accepted_part_rating_sum<S: AsRef<str>>(input: S) -> u64 {
    let Input { workflows, parts } = Input::parse(input.as_ref());
    parts
        .into_iter()
        .filter(|part| workflows.run(part, Target::default()))
        .map(Part::score)
        .sum()
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Parse for Category {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, id) = alt((char('x'), char('m'), char('a'), char('s')))(input)?;
        Ok((
            rest,
            match id {
                'x' => Category::X,
                'm' => Category::M,
                'a' => Category::A,
                's' => Category::S,
                _ => unreachable!(),
            },
        ))
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn score(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl Parse for Part {
    fn parse(value: &str) -> IResult<&str, Self> {
        fn inards(input: &str) -> IResult<&str, Part> {
            use nom::character::complete::u64;
            let (remaining, parsed) = tuple((
                tag("x="),
                u64,
                tag(",m="),
                u64,
                tag(",a="),
                u64,
                tag(",s="),
                u64,
            ))(input)?;
            Ok((
                remaining,
                Part {
                    x: parsed.1,
                    m: parsed.3,
                    a: parsed.5,
                    s: parsed.7,
                },
            ))
        }
        delimited(char('{'), inards, char('}'))(value)
    }
}

#[derive(Debug)]
struct Parts(Vec<Part>);

impl Parse for Parts {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, parts) = separated_list0(newline, Part::parse)(input)?;
        Ok((rest, Self(parts)))
    }
}

impl<'a> IntoIterator for &'a Parts {
    type Item = &'a Part;
    type IntoIter = std::slice::Iter<'a, Part>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

trait Parse
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;
}

#[derive(Debug)]
enum Comparator {
    Gt(u64),
    Lt(u64),
}

impl Comparator {
    fn evaluate(&self, lhs: u64) -> bool {
        match self {
            Comparator::Gt(rhs) => lhs > *rhs,
            Comparator::Lt(rhs) => lhs < *rhs,
        }
    }
}

impl Parse for Comparator {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::character::complete::u64;
        let (rest, (operator, operand)) = pair(alt((char('>'), char('<'))), u64)(input)?;
        let comparator = match operator {
            '>' => Comparator::Gt(operand),
            '<' => Comparator::Lt(operand),
            _ => unreachable!(),
        };
        Ok((rest, comparator))
    }
}

#[derive(Debug)]
struct Condition {
    category: Category,
    comparator: Comparator,
}

impl Condition {
    fn evaluate(&self, part: &Part) -> bool {
        let candidate = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };
        self.comparator.evaluate(candidate)
    }
}

impl Parse for Condition {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, (category, comparator)) = pair(Category::parse, Comparator::parse)(input)?;
        Ok((
            rest,
            Self {
                category,
                comparator,
            },
        ))
    }
}

#[derive(Debug, Clone)]
enum Target {
    Accept,
    Reject,
    Workflow(String),
}

impl Default for Target {
    fn default() -> Self {
        Self::Workflow("in".to_string())
    }
}

impl Parse for Target {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, id) = alpha1(input)?;
        Ok((
            rest,
            match id {
                "A" => Target::Accept,
                "R" => Target::Reject,
                _ => Target::Workflow(id.to_string()),
            },
        ))
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    target: Target,
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<Target> {
        if let Some(ref condition) = self.condition {
            if condition.evaluate(part) {
                return Some(self.target.clone());
            }
            return None;
        }
        Some(self.target.clone())
    }
}

impl Parse for Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        if let Ok((rest, (condition, target))) =
            separated_pair(Condition::parse, char(':'), Target::parse)(input)
        {
            return Ok((
                rest,
                Self {
                    condition: Some(condition),
                    target,
                },
            ));
        }
        let (rest, target) = Target::parse(input)?;
        Ok((
            rest,
            Self {
                condition: None,
                target,
            },
        ))
    }
}

#[derive(Debug)]
struct Rules(Vec<Rule>);

impl<'a> IntoIterator for &'a Rules {
    type Item = &'a Rule;
    type IntoIter = std::slice::Iter<'a, Rule>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Parse for Rules {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, rules) = separated_list0(char(','), Rule::parse)(input)?;
        Ok((rest, Self(rules)))
    }
}

#[derive(Debug)]
struct Workflow(Rules);

impl Parse for Workflow {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, rules) = delimited(char('{'), Rules::parse, char('}'))(input)?;
        Ok((rest, Self(rules)))
    }
}

impl Workflow {
    fn run(&self, part: &Part) -> Target {
        for rule in &self.0 {
            if let Some(target) = rule.evaluate(part) {
                return target;
            }
        }
        Target::Reject
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    // true = Accepted
    fn run(&self, part: &Part, target: Target) -> bool {
        match target {
            Target::Reject => false,
            Target::Accept => true,
            Target::Workflow(id) => {
                let workflow = self.0.get(id.as_str()).unwrap();
                self.run(part, workflow.run(part))
            }
        }
    }
}

#[derive(Debug)]
struct NamedWorkflow<'a>(&'a str, Workflow);

// TODO this with Parse trait?
impl<'a> NamedWorkflow<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (name, workflow)) = tuple((alpha1, Workflow::parse))(input)?;
        Ok((rest, Self(name, workflow)))
    }
}

impl Parse for Workflows {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, named_workflows) = separated_list0(newline, NamedWorkflow::parse)(input)?;
        Ok((
            rest,
            Self(HashMap::from_iter(named_workflows.into_iter().map(
                |NamedWorkflow(name, workflow)| (name.to_owned(), workflow),
            ))),
        ))
    }
}

#[derive(Debug)]
struct Input {
    parts: Parts,
    workflows: Workflows,
}

impl Input {
    pub fn parse(input: &str) -> Self {
        let (_, (workflows, parts)) =
            separated_pair(Workflows::parse, tag("\n\n"), Parts::parse)(input).unwrap();
        Self { workflows, parts }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;
    use indoc::indoc;

    #[test]
    fn parse_rules() -> Result<()> {
        Rules::parse("a<2006:qkq,m>2090:A,rfg")?;
        Ok(())
    }

    #[test]
    fn parse_workflow() -> Result<()> {
        Workflow::parse("{a<2006:qkq,m>2090:A,rfg}")?;
        Ok(())
    }

    #[test]
    fn full_example() {
        let example = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "};
        assert_eq!(accepted_part_rating_sum(example), 19114);
    }
}
