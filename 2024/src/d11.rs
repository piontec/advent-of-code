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
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(183484)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, 25)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, 75)
    }
}

fn run(lines: &Vec<String>, iter: i8) -> i64 {
    let nums = lines[0].split(' ').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut stones = VecDeque::from(nums);
    for i in 0..iter {
        println!("Iter {i}");
        blink(&mut stones);
    }
    stones.len() as i64
}

fn blink(stones: &mut VecDeque<i64>) {
    let mut stone_index = 0;
    while stone_index< stones.len() {
        let stone_str = stones[stone_index].to_string();
        if stones[stone_index] == 0{
            stones[stone_index] = 1;
        }
        else if stone_str.len() % 2 == 0 {
            let high = stone_str[0..stone_str.len() / 2].parse::<i64>().unwrap();
            let low = stone_str[stone_str.len() / 2..].parse::<i64>().unwrap();
            stones[stone_index] = high;
            stones.insert(stone_index + 1, low);
            stone_index += 1;
        }
        else {
            stones[stone_index] *= 2024;
        }
        stone_index += 1;
    }
}

