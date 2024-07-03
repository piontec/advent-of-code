use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

enum Result {
    Valid,
    Invalid,
    Maybe(u8)
}

const TI: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        12
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        21
    }

    fn get_part2_test_result(&self) -> i64 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        lines.iter().map(|line| count_permutations(line)).sum()
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

fn count_permutations(line: &String) -> i64 {
    let (chars_str, counts_str) = line
        .split_once(" ")
        .unwrap();
    let expected_counts: Vec<i32> = counts_str
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let chars: String = String::from(chars_str);

    let mut combinations = 0;
    let mut to_check = Vec::new();
    to_check.push((chars, &expected_counts));

    while to_check.len() > 0 {
        let (to_check_chars, to_check_counts) = to_check.pop().unwrap();

        match is_valid(&to_check_chars, &to_check_counts) {
            Result::Valid => {
                combinations += 1;
            },
            Result::Invalid => {
                continue;
            },
            Result::Maybe(ind) => {
                let has = to_check_chars[..ind as usize].to_string() + "#" + &to_check_chars[(ind as usize + 1)..];
                let has_not = to_check_chars[..ind as usize].to_string() + "." + &to_check_chars[(ind as usize + 1)..];
                to_check.push((has, to_check_counts));
                to_check.push((has_not, to_check_counts));
            } 
        }
    }
    combinations
}

fn is_valid(bytes_str: &str, expected_counts: &Vec<i32>) -> Result {
    let mut in_stream_block = false;
    let mut block_counts = Vec::new();
    let mut block_len = 0;
    for (i, b) in bytes_str.chars().enumerate() {
        if b == '#' {
            if !in_stream_block {
                in_stream_block = true;
            }
            block_len += 1;
            if i == bytes_str.len() - 1 {
                block_counts.push(block_len);
            }
        } else if b == '.' {
            if in_stream_block {
                if expected_counts.len() == block_counts.len() {
                    return Result::Invalid;
                }
                if expected_counts[block_counts.len()] != block_len {
                    return Result::Invalid;
                }

                block_counts.push(block_len);
                block_len = 0;
                in_stream_block = false;
            }
        } else {
            return Result::Maybe(i as u8);
        }
    }
    if block_counts == *expected_counts { Result::Valid } else { Result::Invalid }
}
