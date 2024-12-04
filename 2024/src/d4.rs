use regex::Regex;

use crate::DayTask;
use std::vec;

pub struct Task;

// const TI: &str = "ABCD
// EFGH
// IJKL
// MNOP";
const TI: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        4
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![18]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(2344)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let line_len = lines.len();
        let char_lines = lines
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert!(char_lines.len() == char_lines[0].len());

        let mut all_strings = lines.clone();
        for oi in 0..lines.len() {
            let mut vertical = String::with_capacity(line_len);
            let mut diag_dr = String::with_capacity(line_len);
            let mut diag_dr2 = String::with_capacity(line_len);
            let mut diag_dl = String::with_capacity(line_len);
            let mut diag_dl2 = String::with_capacity(line_len);
            for ii in 0..lines.len() {
                vertical.push(char_lines[ii][oi]);

                let ddr_y = oi + ii;
                let ddr_x = ii;
                let ddr_y2 = ii;
                let ddr_x2 = oi + ii;
                if ddr_y < line_len {
                    diag_dr.push(char_lines[ddr_y][ddr_x]);
                }
                if ddr_x2 < line_len {
                    diag_dr2.push(char_lines[ddr_y2][ddr_x2]);
                }

                let ddl_y = ii;
                let ddl_x: isize = line_len as isize - (oi + ii) as isize - 1;
                let ddl_y2 = oi + ii;
                let ddl_x2 = line_len - ii - 1;
                if ddl_x >= 0 {
                    diag_dl.push(char_lines[ddl_y][ddl_x as usize]);
                }
                if ddl_y2 < line_len {
                    diag_dl2.push(char_lines[ddl_y2][ddl_x2]);
                }
            }
            all_strings.push(char_lines[oi].iter().rev().collect::<String>());
            all_strings.push(vertical.chars().rev().collect::<String>());
            all_strings.push(vertical);
            all_strings.push(diag_dr.chars().rev().collect::<String>());
            all_strings.push(diag_dr);
            all_strings.push(diag_dl.chars().rev().collect::<String>());
            all_strings.push(diag_dl);
            if oi > 0 {
                all_strings.push(diag_dr2.chars().rev().collect::<String>());
                all_strings.push(diag_dr2);
                all_strings.push(diag_dl2.chars().rev().collect::<String>());
                all_strings.push(diag_dl2);
            }
        }
        let xmas = Regex::new(r"XMAS").unwrap();
        let res = all_strings
            .iter()
            .map(|l| xmas.find_iter(l).count())
            .sum::<usize>();
        res as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }
}
