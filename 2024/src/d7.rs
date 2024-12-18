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
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let res = lines
            .iter()
            .map(|l| can_match(l))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .sum();
        res
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }
}

fn can_match(line: &str) -> Option<i64> {
    let mut parts = line.split(':');
    let wanted = parts.next().unwrap().parse::<i64>().unwrap();
    let nums = parts
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    if eval(wanted, 0, &nums, Op::Add) || eval(wanted, 1, &nums, Op::Mul) {
        Some(wanted)
    } else {
        None
    }
}

fn eval(target: i64, left: i64, rest: &[i64], op: Op) -> bool {
    if rest.is_empty() {
        return left == target;
    }
    let next_left = match op {
        Op::Add => left + rest[0],
        Op::Mul => left * rest[0],
    };
    let rest = &rest[1..];
    eval(target, next_left, rest, Op::Add) || eval(target, next_left, rest, Op::Mul)
}

enum Op {
    Add,
    Mul,
}
