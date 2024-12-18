use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

const TI: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        7
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![3749]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![11387]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(850435817339)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(104824810233437)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let res = lines
            .iter()
            .map(|l| can_match(l, false))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .sum();
        res
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let res = lines
            .iter()
            .map(|l| can_match(l, true))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .sum();
        res
    }
}

fn can_match(line: &str, use_concat: bool) -> Option<i64> {
    let mut parts = line.split(':');
    let wanted = parts.next().unwrap().parse::<i64>().unwrap();
    let nums = parts
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    if eval(wanted, 0, &nums, Op::Add, use_concat)
        || eval(wanted, 1, &nums, Op::Mul, use_concat)
        || (use_concat && eval(wanted, 1, &nums, Op::Concat, use_concat))
    {
        Some(wanted)
    } else {
        None
    }
}

fn eval(target: i64, left: i64, rest: &[i64], op: Op, use_concat: bool) -> bool {
    if rest.is_empty() {
        return left == target;
    }
    let next_left = match op {
        Op::Add => left + rest[0],
        Op::Mul => left * rest[0],
        Op::Concat => format!("{}{}", left, rest[0]).parse::<i64>().unwrap(),
    };
    let rest = &rest[1..];
    eval(target, next_left, rest, Op::Add, use_concat)
        || eval(target, next_left, rest, Op::Mul, use_concat)
        || (use_concat && eval(target, next_left, rest, Op::Concat, use_concat))
}

enum Op {
    Add,
    Mul,
    Concat,
}
