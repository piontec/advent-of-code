use crate::DayTask;
use std::vec;

pub struct Task;

const TI: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

impl DayTask<i32> for Task {

    fn day_no(&self) -> u8 {
        9
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i32 {
        114
    }

    fn get_part2_test_result(&self) -> i32 {
        2
    }

    fn run_p1(&self, lines: &Vec<String>) -> i32 {
        lines.iter().map(|l| do_one_line(l)).sum()
    }

    fn run_p2(&self, lines: &Vec<String>) -> i32 {
        lines.iter().map(|l| do_one_line_part2(l)).sum()
    }

    fn get_part1_result(&self) -> Option<i32> {
        Some(1731106378)
    }

    fn get_part2_result(&self) -> Option<i32> {
        Some(1087)
    }
}

fn do_one_line_part2(l: &str) -> i32 {
    let series = get_series(l);

    let mut extras = vec![0];

    let sl = series.len() - 2;
    for si in (0..=sl).rev() {
        let next_num = series[si][0] - extras.last().unwrap();
        extras.push(next_num);
    }

    extras.last().unwrap().clone()
}

fn do_one_line(l: &str) -> i32 {
    let mut series = get_series(l);

    let series_len = series.len();
    let last = &mut series[series_len - 1];
    last.push(0);

    for si in series.len() - 2..0 {
        let next_num = series[si + 1].last().unwrap() + series[si].last().unwrap();
        series[si].push(next_num);
    }

    let res = series.iter().map(|s| s.last().unwrap()).sum::<i32>();
    res
}

fn get_series(l: &str) -> Vec<Vec<i32>> {
    let nums: Vec<i32> = l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut prev = nums.clone();
    let mut next: Vec<i32>;
    let mut series = vec![nums];
    loop {
        next = prev.windows(2).map(|s| s[1] - s[0]).collect::<Vec<i32>>();
        prev = next.clone();
        series.push(next);

        if prev.iter().all(|e| *e == 0) {
            break;
        }
    }
    series
}
