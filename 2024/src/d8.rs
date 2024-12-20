use itertools::Itertools;

use crate::{common::Point2D, DayTask};
use std::collections::{HashMap, HashSet};

pub struct Task;

const TI: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        8
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![14]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![34]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(379)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(1339)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        run_it(lines, true)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        run_it(lines, false)
    }
}

fn run_it(lines: &Vec<String>, just_first: bool) -> i64 {
    let (max_y, max_x) = (lines.len(), lines[0].len());
    let antennas = parse(lines);
    let mut res = HashSet::new();
    for antenna in antennas.keys() {
        let c = antennas[antenna]
            .iter()
            .combinations(2)
            .map(|locs| get_antinodes_in_range(locs[0], locs[1], max_x, max_y, just_first))
            .flatten()
            .collect::<HashSet<Point2D<usize>>>();
        res.extend(c);
    }
    res.len() as i64
}

fn parse(lines: &Vec<String>) -> HashMap<char, Vec<Point2D<usize>>> {
    let antennas = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c != '.' {
                        Some((c, Point2D::new(x, y)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(char, Point2D<usize>)>>()
        })
        .flatten()
        .fold(
            HashMap::<char, Vec<Point2D<usize>>>::new(),
            |mut acc, (c, p)| {
                acc.entry(c).or_insert(Vec::new()).push(p);
                acc
            },
        );
    antennas
}

fn get_antinodes_in_range(
    loc1: &Point2D<usize>,
    loc2: &Point2D<usize>,
    max_x: usize,
    max_y: usize,
    just_first: bool,
) -> HashSet<Point2D<usize>> {
    let dx1 = loc1.x as isize - loc2.x as isize;
    let dy1 = loc1.y as isize - loc2.y as isize;
    let mut res = HashSet::new();
    let mut in_range = |x: isize, y: isize| {
        if !(x < 0 || x >= max_x as isize || y < 0 || y >= max_y as isize) {
            res.insert(Point2D::new(x as usize, y as usize));
            return true;
        }
        return false;
    };

    if just_first {
        let (p1x, p1y) = (loc1.x as isize + dx1, loc1.y as isize + dy1);
        let (p2x, p2y) = (loc2.x as isize - dx1, loc2.y as isize - dy1);
        in_range(p1x, p1y);
        in_range(p2x, p2y);
    } else {
        let mut mul = 0;
        loop {
            let (p1x, p1y) = (loc1.x as isize + dx1 * mul, loc1.y as isize + dy1 * mul);
            let (p2x, p2y) = (loc2.x as isize - dx1 * mul, loc2.y as isize - dy1 * mul);
            let in1 = in_range(p1x, p1y);
            let in2 = in_range(p2x, p2y);
            if !in1 && !in2 {
                break;
            }
            mul += 1;
        }
    }

    res
}
