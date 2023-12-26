use std::collections::HashMap;
use crate::DayTask;

pub struct Task;

impl DayTask<i32> for Task {

    fn day_no(&self) -> u8 {
        1
    }

    fn get_part1_test_input(&self) -> &'static str {
        "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"
    }

    fn get_part2_test_input(&self) -> &'static str {
        "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"
    }

    fn get_part1_test_result(&self) -> i32 {
        142
    }

    fn get_part2_test_result(&self) -> i32 {
        281
    }


    fn run_p1(&self, lines: &Vec<String>) -> i32 {
        lines.iter().map(|l| self.find_numbers(l)).fold(0, |sum, num| sum + num)
    }

    fn run_p2(&self, lines: &Vec<String>) -> i32 {
        let digits: HashMap<&str, &str> = HashMap::from([
                        ("one", "1"),
                        ("two", "2"),
                        ("three", "3"),
                        ("four", "4"),
                        ("five", "5"),
                        ("six", "6"),
                        ("seven", "7"),
                        ("eight", "8"),
                        ("nine", "9"),
        ]);

        let modified_lines = lines.iter().map(|l| {
            let mut line = l.to_string();

            let mut found_index: Option<usize> = Option::None;
            let mut found_key: Option<&str> = Option::None;
            for (key, _value) in &digits {
                match line.find(key) {
                    Some(index) => {
                        if found_index.is_none() || index < found_index.unwrap() {
                            found_index = Some(index);
                            found_key = Some(key);
                        }
                    },
                    None => {}
                }
            }
            let digit_index = line.chars().position(|c| c.is_numeric());
            if digit_index.is_some() && found_index.is_some() && digit_index.unwrap() < found_index.unwrap() {
                found_key = Option::None;
            }
            if found_key.is_some() {
                line = line.replacen(found_key.unwrap(), digits[found_key.unwrap()], 1);
            }

            found_index = Option::None;
            found_key = Option::None;
            for (key, _value) in &digits {
                match line.rfind(key) {
                    Some(index) => {
                        if found_index.is_none() || index > found_index.unwrap() {
                            found_index = Some(index);
                            found_key = Some(key);
                        }
                    },
                    None => {}
                }
            }
            if found_key.is_some() {
                line = line.replace(found_key.unwrap(), digits[found_key.unwrap()]);
            }
            line
        }).collect();
        self.run_p1(&modified_lines)
    }

    fn get_part1_result(&self) -> Option<i32> {
        None
    }

    fn get_part2_result(&self) -> Option<i32> {
        None
    }
}

impl Task {
    fn find_numbers(&self, line: &String) -> i32 {
        let digits: Vec<i32> = line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        digits[0] * 10 + digits.last().unwrap()
    }
}