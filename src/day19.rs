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

type PartSpec<'a> = HashMap<&'a str, (i32, i32)>;

fn split_spec_on_condition<'a>(condition: &Condition, mut part_spec: PartSpec<'a>) -> (Option<PartSpec<'a>>, Option<PartSpec<'a>>) {
    match condition.comparison {
        Comparison::LessThan => {
            let value = part_spec.get_mut(condition.variable).unwrap();
            if value.0 >= condition.value {
                return (None, Some(part_spec));
            }
            else if value.1 < condition.value {
                return (Some(part_spec), None);
            }
            else {
                // value.0 < condition.value <= value.1
                let mut spec2 = part_spec.clone();
                spec2.get_mut(condition.variable).unwrap().1 = condition.value - 1;
                part_spec.get_mut(condition.variable).unwrap().0 = condition.value;
                return (Some(spec2), Some(part_spec));
            }
        }
        Comparison::GreaterThan => {
            let value = part_spec.get_mut(condition.variable).unwrap();
            if value.1 <= condition.value {
                return (None, Some(part_spec));
            }
            else if value.0 > condition.value {
                return (Some(part_spec), None);
            }
            else {
                // value.0 <= condition.value < value.1
                let mut spec2 = part_spec.clone();
                spec2.get_mut(condition.variable).unwrap().0 = condition.value + 1;
                part_spec.get_mut(condition.variable).unwrap().1 = condition.value;
                return (Some(spec2), Some(part_spec));
            }
        }
    }
}

fn combinations_in_spec(part_spec: &PartSpec) -> i64 {
    part_spec.values().map(|v| {
        let range: i64 = (v.1 - v.0 + 1).into(); range
    }).product::<i64>()
}

fn combinations_from_outcome(outcome: &Outcome, workflows: &HashMap<&str, Workflow>, part_spec: PartSpec) -> i64 {
    match outcome {
        Outcome::Accept => combinations_in_spec(&part_spec),
        Outcome::Reject => 0,
        Outcome::Workflow(workflow) => accepted_combinations(workflow, &workflows, part_spec)
    }
}

fn accepted_combinations(start: &str, workflows: &HashMap<&str, Workflow>, part_spec: PartSpec) -> i64 {
    let mut combinations = 0;

    let workflow = workflows.get(start).unwrap();

    let mut current_spec = part_spec;

    for rule in workflow.rules.iter() {
        match rule {
            Rule::Direct(outcome) => {
                combinations += combinations_from_outcome(outcome, &workflows, current_spec);
                break;
            },
            Rule::Condition(condition, outcome) => {
                let (spec1, spec2) = 
                    split_spec_on_condition(&condition, current_spec);

                if let Some(spec1) = spec1 {
                    combinations += combinations_from_outcome(outcome, &workflows, spec1);
                }

                if let Some(spec2) = spec2 {
                    current_spec = spec2;
                } else {
                    break;
                }
            },
        }
    }

    return combinations;
}

fn part2() {
    let contents = fs::read_to_string("./src/input19.txt").unwrap();
    
    let (workflow_str, _) = contents.split_once("\n\n").unwrap();

    let workflows: HashMap<_, _> = workflow_str.lines().map(parse_workflow).map(|w| (w.name, w)).collect();

    let part_spec = 
        HashMap::from_iter(vec!["x", "m", "a", "s"].iter().map(|k| (*k, (1, 4000))));

    let combinations = accepted_combinations("in", &workflows, part_spec);

    println!("{combinations}");
}

fn main() {
    part1();
}