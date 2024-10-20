use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

enum Condition {
    LT,
    GT,
}

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
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
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

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
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
                    match condition {
                        Condition::LT => {
                            if prop_value < value {
                                Some(target)
                            } else {
                                None
                            }
                        }
                        Condition::GT => {
                            if prop_value > value {
                                Some(target)
                            } else {
                                None
                            }
                        }
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
                    continue;
                }
                None => continue,
            }
        }
    }
}

fn parse_rule(line: &str) -> (String, Vec<Rule>) {
    let mut sub_rules = vec![];
    let (name, rules_part) = line.split_once("{").unwrap();
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
                let value = chars[2..chars.len() - 1]
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
