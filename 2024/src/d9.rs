use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

const TI: &str = "2333133121414131402";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        9
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![1928]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![2858]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(6471961544878)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut sectors = parse(lines);
        let mut left = 0;
        let mut right = sectors.len() - 1;
        loop {
            if left >= right {
                break;
            }
            if sectors[left] == -1 {
                while sectors[right] == -1 {
                    right -= 1;
                }
                sectors[left] = sectors[right];
                sectors[right] = -1;
                right -= 1;
            }
            left += 1;
        }
        checksum(sectors)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut sectors = parse(lines);
        let mut right_index = sectors.len() - 1;
        loop {
            // find the next file id from the right
            while sectors[right_index] == -1 {
                right_index -= 1;
            }
            let file_id = sectors[right_index];
            let file_end = right_index;
            while sectors[right_index] == file_id {
                right_index -= 1;
            }
            let file_len = file_end - right_index;
            // go from the left and check if we can find space to fit it
            let mut left_index = 0;
            loop {
                while sectors[left_index] != -1 {
                    left_index += 1;
                }
                let empty_start = left_index;
                while sectors[left_index] == -1 {
                    left_index += 1;
                }
            }

            // if we checked the file with id 0, we're done
            if file_id == 0 {
                break;
            }
        }
        todo!()
    }
}

fn checksum(sectors: Vec<isize>) -> i64 {
    let res = sectors
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != -1)
        .map(|(i, v)| i as i64 * *v as i64)
        .sum::<i64>();
    res
}

fn parse(lines: &Vec<String>) -> Vec<isize> {
    let nums = lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let mut is_file = true;
    let mut file_id: isize = 0;
    let mut sectors: Vec<isize> = Vec::new();
    for num in nums {
        if is_file {
            for _ in 0..num {
                sectors.push(file_id);
            }
            is_file = false;
            file_id += 1;
        } else {
            for _ in 0..num {
                sectors.push(-1);
            }
            is_file = true;
        }
    }
    sectors
}
