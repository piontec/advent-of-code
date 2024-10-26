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

impl DayTask<i64> for Task {

    fn day_no(&self) -> u8 {
        11
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        374
    }

    fn get_part2_test_result(&self) -> i64 {
        82000210
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, 2)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, 1000000)
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(9509330)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn run(lines: &Vec<String>, factor: isize) -> i64 {
    let star_map = get_star_map(lines);
    let (rows, cols) = get_empty(lines);
    let expanded_star_map = expand(&star_map, rows, cols, factor);

    let mut total_dist = 0;

    for source_index in 0..(expanded_star_map.len() - 1) {
        for dest_index in (source_index + 1)..expanded_star_map.len() {
            total_dist += expanded_star_map[source_index].manhattan_distance(&expanded_star_map[dest_index]);
        }
    }

    total_dist as i64
}

fn expand(star_map: &[Point2D<isize>], 
    rows: Vec<usize>, 
    cols: Vec<usize>,
    factor: isize) -> Vec<Point2D<isize>> {
    let mut expanded = Vec::new();
    for p in star_map {
        let new_x = cols.iter().filter(|c| (**c as isize) < p.x).count() as isize * (factor - 1) + p.x;
        let new_y = rows.iter().filter(|c| (**c as isize) < p.y).count() as isize * (factor - 1) + p.y;
        expanded.push(Point2D::new(new_x, new_y));
    }

    expanded
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

fn get_empty(lines: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let rows = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|c| c == '.'))
        .map(|(i, _)| i)
        .collect();

    let mut cols = Vec::new();
    // expand columns
    for ci in (0..lines[0].len()).rev() {
        let mut all_empty = true;
        for line in lines {
            if line.as_bytes()[ci] == b'#' {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            cols.push(ci);
        }
    } 

    (rows, cols)
}