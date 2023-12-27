use std::fs;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

enum Comparison {
    LessThan,
    GreaterThan
}

enum Outcome<'a> {
    Workflow(&'a str),
    Accept,
    Reject
}

struct Condition<'a> {
    variable: &'a str,
    comparison: Comparison,
    value: i32
}

enum Rule<'a> {
    Condition(Condition<'a>, Outcome<'a>),
    Direct(Outcome<'a>),
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

lazy_static!{
    static ref WORKFLOW_REGEX: Regex = Regex::new(r"(.*)\{(.*)\}").unwrap();
}

fn parse_condition(condition_str: &str) -> Condition {
    if let Some((variable, value_str)) = condition_str.split_once('<') {
        return Condition {
            variable: variable,
            comparison: Comparison::LessThan,
            value: value_str.parse().unwrap()
        };
    }
    else if let Some((variable, value_str)) = condition_str.split_once('>') {
        return Condition {
            variable: variable,
            comparison: Comparison::GreaterThan,
            value: value_str.parse().unwrap()
        };
    }
    panic!();
}

fn parse_outcome(outcome_str: &str) -> Outcome {
    return match outcome_str {
        "A" => Outcome::Accept,
        "R" => Outcome::Reject,
        workflow => Outcome::Workflow(workflow)
    }
}

fn parse_rule(rule_str: &str) -> Rule {
    if let Some((condition_str, outcome_str)) = rule_str.split_once(':') {
        return Rule::Condition(parse_condition(condition_str), parse_outcome(outcome_str))
    } else {
        return Rule::Direct(parse_outcome(rule_str))
    }
}

fn parse_workflow(line: &str) -> Workflow {
    let (_, [name, rules_str]) = WORKFLOW_REGEX.captures(line).unwrap().extract();

    let rules: Vec<_> = rules_str.split(',').map(parse_rule).collect();

    return Workflow {
        name: name,
        rules: rules
    };
}

fn parse_part(part_str: &str) -> HashMap<&str, i32> {
    part_str[1..part_str.len()-1].split(',').map(|s| {
        let (variable, value) = s.split_once('=').unwrap();
        (variable, value.parse().unwrap())
    }).collect()
}

type Part<'a> = HashMap<&'a str, i32>;

fn apply_condition(condition: &Condition, part: &Part) -> bool {
    match condition.comparison {
        Comparison::LessThan => *part.get(condition.variable).unwrap() < condition.value,
        Comparison::GreaterThan => *part.get(condition.variable).unwrap() > condition.value,
    }
}

fn apply_workflow<'a>(workflow: &'a Workflow<'a>, part: &'a Part<'a>) -> &'a Outcome<'a> {
    for rule in workflow.rules.iter() {
        match rule {
            Rule::Direct(outcome) => {
                return outcome
            },
            Rule::Condition(condition, outcome) => {
                if apply_condition(&condition, &part) {
                    return outcome;
                }
            }
        }
    }
    panic!();
}

fn apply_workflows(workflows: &HashMap<&str, Workflow>, part: &Part) -> bool {
    let mut curr = "in";
    loop {
        let workflow = workflows.get(curr).unwrap();
        match apply_workflow(workflow, part) {
            Outcome::Accept => {
                return true;
            },
            Outcome::Reject => {
                return false;
            }
            Outcome::Workflow(new_workflow) => {
                curr = new_workflow;
            }
        }
    }
}

fn part1() {
    let contents = fs::read_to_string("./src/input19.txt").unwrap();
    
    let (workflow_str, parts_str) = contents.split_once("\n\n").unwrap();

    let workflows: HashMap<_, _> = workflow_str.lines().map(parse_workflow).map(|w| (w.name, w)).collect();

    let parts: Vec<Part> = parts_str.lines().map(parse_part).collect();

    let accepted_part_rating: i32 = parts.iter()
        .filter(|part| apply_workflows(&workflows, part))
        .map(|part| part.values().sum::<i32>()).sum();

    println!("{accepted_part_rating}");
}

fn main() {
    part1();
}