use crate::DayTask;
use regex::Regex;

pub struct Task;

const TI: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TI2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        3
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI2]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![161]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![48]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(185797128)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(89798695)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        lines
            .iter()
            .map(|l| {
                let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
                let mut mul: i64 = 0;
                for (_, [a, b]) in regex.captures_iter(l).map(|c| c.extract()) {
                    mul += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
                }
                mul
            })
            .sum()
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let regex = Regex::new(
            r"(?P<mul>mul\((?P<a>\d{1,3}),(?P<b>\d{1,3})\))|(?P<do>do\(\))|(?P<dont>don\'t\(\))",
        )
        .unwrap();
        let mut enabled = true;
        lines
            .iter()
            .map(|l| {
                let mut mul: i64 = 0;
                for capt in regex.captures_iter(l) {
                    if capt.name("do").is_some() {
                        enabled = true;
                        continue;
                    }
                    if capt.name("dont").is_some() {
                        enabled = false;
                        continue;
                    }
                    if enabled && capt.name("mul").is_some() {
                        let a = capt.name("a").unwrap().as_str().parse::<i64>().unwrap();
                        let b = capt.name("b").unwrap().as_str().parse::<i64>().unwrap();
                        mul += a * b;
                    }
                }
                mul
            })
            .sum()
    }
}
