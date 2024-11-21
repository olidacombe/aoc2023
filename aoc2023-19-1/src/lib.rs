use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn accepted_part_rating_sum<S: AsRef<str>>(input: S) -> u64 {
    let input = Input::parse(input.as_ref());
    dbg!(input);
    u64::default()
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
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

impl Parts {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, parts) = separated_list0(newline, Part::parse)(input)?;
        Ok((rest, Self(parts)))
    }
}

#[derive(Debug)]
struct Rule(String);

impl Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        // consume the whole string for now
        let (nothing, everything) = input.split_at(0);
        Ok((nothing, Self(everything.to_string())))
    }
}

#[derive(Debug)]
struct Rules(Vec<Rule>);

impl Rules {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, rules) = separated_list0(char(','), Rule::parse)(input)?;
        Ok((rest, Self(rules)))
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Rules,
}

impl Workflow {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, rules) = delimited(char('{'), Rules::parse, char('}'))(input)?;
        Ok((rest, Self { rules }))
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Workflow>);

#[derive(Debug)]
struct NamedWorkflow<'a>(&'a str, Workflow);

impl<'a> NamedWorkflow<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (name, workflow)) = tuple((alpha1, Workflow::parse))(input)?;
        Ok((rest, Self(name, workflow)))
    }
}

impl Workflows {
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
            separated_pair(Workflows::parse, newline, Parts::parse)(input).unwrap();
        Self { workflows, parts }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_one_workflow() {
        let example = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
        "};
        let input = Input::parse(example);
        dbg!(input);
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
