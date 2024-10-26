use std::{collections::VecDeque, ops::RangeInclusive};

use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

#[derive(Debug, Eq, PartialEq)]
enum Condition {
    LT,
    GT,
}

#[derive(Debug)]
enum Rule {
    Absolute(String),
    Complex(char, Condition, i64, String),
}

const TI: &str = "px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";

type Ranges = HashMap<char, RangeInclusive<u16>>;

#[derive(Debug)]
struct RuleRanges {
    rule_name: String,
    ranges: Ranges,
}

impl RuleRanges {
    fn new(rule_name: String, ranges: HashMap<char, RangeInclusive<u16>>) -> Self {
        RuleRanges { rule_name, ranges }
    }

    fn starting() -> Self {
        RuleRanges {
            rule_name: "in".to_string(),
            ranges: HashMap::from([
                ('x', 1..=4000),
                ('m', 1..=4000),
                ('a', 1..=4000),
                ('s', 1..=4000),
            ]),
        }
    }

    fn product(&self) -> i64 {
        self.ranges
            .values()
            .map(|r| *r.end() as i64 - *r.start() as i64 + 1)
            .product()
    }
}

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        19
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        19114
    }

    fn get_part2_test_result(&self) -> i64 {
        167409079868000
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut total = 0;
        let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();

        let mut in_objects = false;
        for line in lines {
            if line.is_empty() {
                in_objects = true;
                continue;
            }
            if !in_objects {
                let (rule_name, rules_seq) = parse_rule(line);
                rules.insert(rule_name, rules_seq);
            } else {
                total += eval_object(line, &rules);
            }
        }

        total
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();

        for line in lines {
            if line.is_empty() {
                break;
            }
            let (rule_name, rules_seq) = parse_rule(line);
            rules.insert(rule_name, rules_seq);
        }

        let mut to_check = VecDeque::from([RuleRanges::starting()]);
        let mut result: i64 = 0;
        while let Some(rule_ranges) = to_check.pop_front() {
            let rule = rules.get(&rule_ranges.rule_name).unwrap();
            let mut left = Some(rule_ranges.ranges);
            for sub_rule in rule {
                let consumed: Option<RuleRanges>;
                (consumed, left) = apply_rule_ranges(sub_rule, &left.unwrap());
                if let Some(consumed) = consumed {
                    match consumed.rule_name.as_str() {
                        "A" => {
                            result += consumed.product();
                        }
                        // ignoring rejected
                        "R" => {}
                        _ => {
                            to_check.push_back(consumed);
                        }
                    }
                }
                if left.is_none() {
                    break;
                }
            }
        }

        result
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(397061)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(125657431183201)
    }
}

fn apply_rule_ranges(sub_rule: &Rule, ranges: &Ranges) -> (Option<RuleRanges>, Option<Ranges>) {
    match sub_rule {
        Rule::Absolute(target) => {
            return (
                Some(RuleRanges::new(target.to_string(), ranges.clone())),
                None,
            )
        }
        Rule::Complex(property, condition, value, target) => {
            let range = &ranges[property];
            // everything matches
            if (*condition == Condition::LT && *range.end() < *value as u16)
                || (*condition == Condition::GT && *range.start() > *value as u16)
            {
                return (
                    Some(RuleRanges::new(target.to_string(), ranges.clone())),
                    None,
                );
            }
            // nothing matches
            if (*condition == Condition::LT && *range.start() >= *value as u16)
                || (*condition == Condition::GT && *range.end() <= *value as u16)
            {
                return (None, Some(ranges.clone()));
            }
            // rule value is in range and splits it in two
            let mut new_ranges = ranges.clone();
            let mut left = ranges.clone();
            if *condition == Condition::LT {
                new_ranges.insert(*property, *range.start()..=*value as u16 - 1);
                left.insert(*property, *value as u16..=*range.end());
            } else if *condition == Condition::GT {
                new_ranges.insert(*property, *value as u16 + 1..=*range.end());
                left.insert(*property, *range.start()..=*value as u16);
            } else {
                panic!("Invalid condition")
            }
            return (
                Some(RuleRanges::new(target.to_string(), new_ranges)),
                Some(left),
            );
        }
    }
}

fn eval_object(line: &str, rules: &HashMap<String, Vec<Rule>>) -> i64 {
    let object = line.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
    let props = object
        .split(",")
        .map(|prop| {
            let (name, value) = prop.split_once("=").unwrap();
            (name.chars().nth(0).unwrap(), value.parse::<i64>().unwrap())
        })
        .collect::<HashMap<char, i64>>();
    let mut current_rule = "in";
    loop {
        let rule = rules.get(current_rule).unwrap();
        for sub_rule in rule {
            let target_rule = match sub_rule {
                Rule::Absolute(target) => Some(target),
                Rule::Complex(property, condition, value, target) => {
                    let prop_value = props.get(property).unwrap();
                    if (*condition == Condition::LT && prop_value < value)
                        || (*condition == Condition::GT && prop_value > value)
                    {
                        Some(target)
                    } else {
                        None
                    }
                }
            };
            match target_rule {
                Some(target) => {
                    if target == "R" {
                        return 0;
                    } else if target == "A" {
                        return props.into_values().sum();
                    }
                    current_rule = target;
                    break;
                }
                None => continue,
            }
        }
    }
}

fn parse_rule(line: &str) -> (String, Vec<Rule>) {
    let mut sub_rules = vec![];
    let (name, rules_part) = line.split_once("{").unwrap();
    let rules_part = rules_part.strip_suffix("}").unwrap();
    for rule_str in rules_part.split(",") {
        let rule = match rule_str.split_once(":") {
            Some((condition, target)) => {
                let chars = condition.chars().collect::<Vec<char>>();
                let property = chars[0];
                let condition = match chars[1] {
                    '<' => Condition::LT,
                    '>' => Condition::GT,
                    _ => panic!("Invalid condition"),
                };
                let value = chars[2..chars.len()]
                    .into_iter()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                Rule::Complex(property, condition, value, target.to_string())
            }
            None => Rule::Absolute(rule_str.to_string()),
        };
        sub_rules.push(rule);
    }
    (name.to_string(), sub_rules)
}
