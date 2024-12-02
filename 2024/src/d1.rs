use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

const TI: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        1
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![11]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![31]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(1873376)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(18997088)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (mut l1, mut l2) = get_lists(lines);
        l1.sort();
        l2.sort();
        let res = l1
            .iter()
            .zip(l2.iter())
            .map(|(n1, n2)| (n1 - n2).abs())
            .sum();
        res
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (l1, l2) = get_lists(lines);
        let l2_freq = l2.iter().fold(HashMap::new(), |mut acc, &n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });
        let sum = l1
            .iter()
            .map(|n| l2_freq.get(n).or(Some(&0)).unwrap() * n)
            .sum();
        sum
    }
}

fn get_lists(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let (l1, l2): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|l| {
            let mut it = l.split_whitespace().map(|s| s.parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .unzip();
    (l1, l2)
}
