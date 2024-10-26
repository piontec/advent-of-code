use crate::DayTask;
use std::collections::HashSet;

pub struct Task;

const TI: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

impl DayTask<i32> for Task {
    fn day_no(&self) -> u8 {
        4
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i32 {
        13
    }

    fn get_part2_test_result(&self) -> i32 {
        30
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i32 {
        let mut total = 0;
        for line in lines {
            let match_nums_count = get_winning_numbers(&line);
            if match_nums_count > 0 {
                let base: i32 = 2;
                total += base.pow(match_nums_count - 1);
            }
        }
        total
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i32 {
        let mut counts = vec![1; lines.len()];
        for (idx, line) in lines.iter().enumerate() {
            let match_nums_count = get_winning_numbers(line);
            let card_count = counts[idx];
            for i in idx + 1..=idx + match_nums_count as usize {
                counts[i] += card_count;
            }
        }
        counts.iter().sum()
    }

    fn get_part1_result(&self) -> Option<i32> {
        Some(24175)
    }

    fn get_part2_result(&self) -> Option<i32> {
        Some(18846301)
    }
}

fn get_winning_numbers(line: &String) -> u32 {
    let (left, right) = line.split_once(" | ").unwrap();
    let left = left.split_once(": ").unwrap().1;
    let left_set: HashSet<i32> = HashSet::from_iter(
        left.split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap()),
    );
    let right_set: HashSet<i32> = HashSet::from_iter(
        right
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap()),
    );
    let common_len = left_set.intersection(&right_set).count() as u32;
    common_len
}
