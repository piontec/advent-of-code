use crate::DayTask;
use std::{collections::HashMap, io::Lines, usize};
use crate::common::Point2D;

pub struct Task;

const TI: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

impl DayTask<i32> for Task {

    fn day_no(&self) -> u8 {
        11
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i32 {
        374
    }

    fn get_part2_test_result(&self) -> i32 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i32 {
        let expanded = expand(lines);
        let star_map = get_star_map(&expanded);

        let mut total_dist = 0;

        for source_index in 0..(star_map.len() - 1) {
            for dest_index in (source_index + 1)..star_map.len() {
                total_dist += star_map[source_index].manhattan_distance(&star_map[dest_index]);
            }
        }

        total_dist as i32
    }

    fn run_p2(&self, lines: &Vec<String>) -> i32 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i32> {
        Some(9509330)
    }

    fn get_part2_result(&self) -> Option<i32> {
        None
    }
}

fn get_star_map(expanded: &Vec<String>) -> Vec<Point2D<isize>> {
    let mut star_map = Vec::new();
    for y in 0..expanded.len() {
        for x in 0..expanded[y].len() {
            if expanded[y].as_bytes()[x] == b'#' {
                star_map.push(Point2D::new(x as isize, y as isize));
            }
        }
    }

    star_map
}

fn expand(lines: &Vec<String>) -> Vec<String> {
    let mut expanded = Vec::new();

    // expand rows
    for line in lines {
        expanded.push(line.clone());
        if line.chars().all(|c| c == '.') {
            expanded.push(line.clone());
        }
    }

    // expand columns
    for ci in (0..expanded[0].len()).rev() {
        let mut all_empty = true;
        for line in &expanded {
            if line.as_bytes()[ci] == b'#' {
                all_empty = false;
                break;
            }
        }
        if !all_empty {
            continue;
        }
        for line in &mut expanded {
            line.insert(ci, '.');
        }
    } 

    expanded
}