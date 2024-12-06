use crate::DayTask;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Task;

const TI: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        5
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![143]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![123]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(6612)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(4944)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (rules, reverse_rules, cases) = parse(lines);
        cases
            .iter()
            .map(|c| is_valid(c, &rules, &reverse_rules))
            .filter(|&b| b.is_some())
            .map(|b| b.unwrap() as i64)
            .sum::<i64>()
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (rules, reverse_rules, cases) = parse(lines);
        cases
            .iter()
            .filter(|c| is_valid(c, &rules, &reverse_rules).is_none())
            .map(|c| order_pages(c, &rules))
            .sum::<i64>()
    }
}

fn order_pages(case: &Vec<u8>, rules: &HashMap<u8, HashSet<u8>>) -> i64 {
    let case_hs: HashSet<u8> = HashSet::from_iter(case.clone());
    let mut edges: HashMap<u8, HashSet<u8>> = HashMap::from_iter(
        case.iter()
            .map(|c| {
                let common = rules
                    .get(c)
                    .unwrap_or(&HashSet::new())
                    .intersection(&case_hs)
                    .map(|c| *c)
                    .collect();
                (*c, common)
            })
            .collect::<Vec<(u8, HashSet<u8>)>>(),
    );
    let mut no_incoming = case
        .iter()
        .filter(|c| !edges.values().any(|v| v.contains(c)))
        .map(|c| *c)
        .collect::<Vec<u8>>();
    let mut res: Vec<u8> = Vec::new();
    while !no_incoming.is_empty() {
        let n = no_incoming.pop().unwrap();
        res.push(n);
        let next = edges.remove(&n).unwrap();
        no_incoming.extend(
            next.iter()
                .filter(|n| !edges.values().any(|v| v.contains(n))),
        );
    }
    res[res.len() / 2] as i64
}

fn parse(
    lines: &Vec<String>,
) -> (
    HashMap<u8, HashSet<u8>>,
    HashMap<u8, HashSet<u8>>,
    Vec<Vec<u8>>,
) {
    let (rules, reverse_rules) = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split("|");
            let left = parts.next().unwrap().parse::<u8>().unwrap();
            let right = parts.next().unwrap().parse::<u8>().unwrap();
            (left, right)
        })
        .fold((HashMap::new(), HashMap::new()), |mut acc, (l, r)| {
            acc.0.entry(l).or_insert(HashSet::new()).insert(r);
            acc.1.entry(r).or_insert(HashSet::new()).insert(l);
            acc
        });
    let cases = lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    (rules, reverse_rules, cases)
}

fn is_valid(
    c: &[u8],
    rules: &HashMap<u8, HashSet<u8>>,
    reverse_rules: &HashMap<u8, HashSet<u8>>,
) -> Option<u8> {
    for ci in 0..c.len() - 1 {
        let l = c[ci];
        let r = c[ci + 1];
        let rule = rules.get(&l);
        let rev_rule = reverse_rules.get(&l);
        if rule.is_some() && rule.unwrap().contains(&r) {
            continue;
        }
        if rev_rule.is_some() && rev_rule.unwrap().contains(&r) {
            return None;
        }
        panic!("there's no rule!")
    }
    Some(c[c.len() / 2])
}
