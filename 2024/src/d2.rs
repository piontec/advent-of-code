use itertools::Itertools;

use crate::DayTask;

pub struct Task;

const TI: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        2
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![2]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![4]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(631)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(665)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let sets = lines
            .iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|s| is_safe(&s))
            .sum();
        sets
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let sets = lines
            .iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut count = 0;
        for s in sets {
            if is_safe(&s) == 1 {
                count += 1;
                continue;
            }
            for i in 0..s.len() {
                let mut s = s.clone();
                s.remove(i);
                if is_safe(&s) == 1 {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}

fn is_safe(s: &Vec<i64>) -> i64 {
    if s.iter().tuple_windows().all(|(a, b)| a < b && (b - a) <= 3)
        || s.iter().tuple_windows().all(|(a, b)| a > b && (a - b) <= 3)
    {
        1
    } else {
        0
    }
}
