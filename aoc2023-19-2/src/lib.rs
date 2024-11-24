use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    multi::separated_list0,
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

pub fn acceptable_parts_sum<S: AsRef<str>>(input: S) -> u64 {
    let Input { workflows, .. } = Input::parse(input.as_ref());

    workflows.sum_acceptable(PartSpace::default(), Target::default())
}

#[derive(Clone, Copy)]
struct Range {
    from: u64,
    to: u64,
}

impl Default for Range {
    fn default() -> Self {
        Self { from: 1, to: 4000 }
    }
}

impl Range {
    fn size(&self) -> u64 {
        self.to - self.from + 1
    }
}

#[derive(Clone, Default)]
struct PartSpace {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartSpace {
    fn volume(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }

    fn with(&self, category: &Category, range: Range) -> Self {
        let mut ret = self.clone();
        ret.set(category, range);
        ret
    }

    fn set(&mut self, category: &Category, range: Range) {
        match category {
            Category::X => self.x = range,
            Category::M => self.m = range,
            Category::A => self.a = range,
            Category::S => self.s = range,
        }
    }
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

#[allow(dead_code)]
#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
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
    fn split(&self, range: Range) -> (Option<Range>, Option<Range>) {
        match self {
            Self::Gt(threshold) => {
                if *threshold >= range.to {
                    return (None, Some(range));
                }
                if *threshold < range.from {
                    return (Some(range), None);
                }
                (
                    Some(Range {
                        from: threshold + 1,
                        to: range.to,
                    }),
                    Some(Range {
                        from: range.from,
                        to: *threshold,
                    }),
                )
            }
            Self::Lt(threshold) => {
                if *threshold <= range.from {
                    return (None, Some(range));
                }
                if *threshold > range.to {
                    return (Some(range), None);
                }
                (
                    Some(Range {
                        from: range.from,
                        to: threshold - 1,
                    }),
                    Some(Range {
                        from: *threshold,
                        to: range.to,
                    }),
                )
            }
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
    fn split(&self, mut space: PartSpace) -> (Option<PartSpace>, Option<PartSpace>) {
        let mut ret = (None, None);
        let candidate = match self.category {
            Category::X => space.x,
            Category::M => space.m,
            Category::A => space.a,
            Category::S => space.s,
        };
        let (intersection, complement) = self.comparator.split(candidate);
        if let Some(range) = intersection {
            ret.0 = Some(space.with(&self.category, range))
        }
        if let Some(range) = complement {
            space.set(&self.category, range);
            ret.1 = Some(space);
        }
        ret
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

struct SpaceTarget {
    space: PartSpace,
    target: Target,
}

struct SpaceTargets(Vec<SpaceTarget>);

impl IntoIterator for SpaceTargets {
    type Item = SpaceTarget;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    target: Target,
}

impl Rule {
    fn split(&self, space: PartSpace) -> (Option<SpaceTarget>, Option<PartSpace>) {
        if let Some(ref condition) = self.condition {
            let (intersection, complement) = condition.split(space);
            return (
                intersection.map(|space| SpaceTarget {
                    space,
                    target: self.target.clone(),
                }),
                complement,
            );
        }
        // unconditionally whole space send to target
        (
            Some(SpaceTarget {
                space,
                target: self.target.clone(),
            }),
            None,
        )
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
    fn sum_acceptable(&self, space: PartSpace) -> SpaceTargets {
        // what is this spaghetti???
        let mut ret = SpaceTargets(Vec::new());
        let mut space = Some(space);
        for rule in &self.0 {
            let Some(prev) = space else {
                break;
            };
            let (intersection, complement) = rule.split(prev);
            if let Some(space_target) = intersection {
                ret.0.push(space_target);
            }
            space = complement;
        }
        ret
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    fn sum_acceptable(&self, space: PartSpace, target: Target) -> u64 {
        match target {
            Target::Reject => 0,
            Target::Accept => space.volume(),
            Target::Workflow(id) => {
                let workflow = self.0.get(id.as_str()).unwrap();
                workflow
                    .sum_acceptable(space)
                    .into_iter()
                    .map(|SpaceTarget { space, target }| self.sum_acceptable(space, target))
                    .sum()
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
    workflows: Workflows,
}

impl Input {
    pub fn parse(input: &str) -> Self {
        let (_, (workflows, _)) =
            separated_pair(Workflows::parse, tag("\n\n"), Parts::parse)(input).unwrap();
        Self { workflows }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

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
        assert_eq!(acceptable_parts_sum(example), 167409079868000);
    }
}
