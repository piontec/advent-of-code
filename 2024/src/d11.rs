use crate::DayTask;
use std::collections::{HashMap, VecDeque};

pub struct Task;

const TI: &str = "125 17";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        11
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![55312]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![-1]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(183484)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(218817038947400)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, 25)
    }

    fn run_p2(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        if _is_test {
            return -1;
        }
        run(lines, 75)
    }
}

fn run(lines: &Vec<String>, iter: i8) -> i64 {
    let nums = lines[0]
        .split(' ')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    nums.iter().map(|n| count_stones(*n, iter)).sum()
}

#[memoize::memoize]
fn count_stones(n: i64, iter: i8) -> i64 {
    if iter == 0 {
        return 1;
    }

    if n == 0 {
        return count_stones(1, iter - 1);
    }

    let stone_str = n.to_string();
    if stone_str.len() % 2 == 0 {
        let high = stone_str[0..stone_str.len() / 2].parse::<i64>().unwrap();
        let low = stone_str[stone_str.len() / 2..].parse::<i64>().unwrap();
        return count_stones(high, iter - 1) + count_stones(low, iter - 1);
    }
    count_stones(n * 2024, iter - 1)
}
