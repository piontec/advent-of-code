use itertools::Itertools;
use num::{abs, traits::float};

use crate::{common::Point3D, DayTask};
use std::collections::HashMap;

pub struct Task;

const TI: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

struct Hailstone {
    pos: Point3D<usize>,
    speed: Point3D<isize>,
}

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        24
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        2
    }

    fn get_part2_test_result(&self) -> i64 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        let (min_range, max_range) = if is_test { (7usize, 27usize) } else { (200000000000000, 400000000000000) };
        let mut hailstones = lines.iter().map(|line| {
            let parts: Vec<&str> = line.split(" @ ").collect();
            let pos_parts: Vec<&str> = parts[0].split(", ").collect();
            let speed_parts: Vec<&str> = parts[1].split(", ").collect();
            Hailstone {
                pos: Point3D::new(
                    pos_parts[0].trim().parse::<usize>().unwrap(),
                    pos_parts[1].trim().parse::<usize>().unwrap(),
                    pos_parts[2].trim().parse::<usize>().unwrap(),
                ),
                speed: Point3D::new(
                    speed_parts[0].trim().parse::<isize>().unwrap(),
                    speed_parts[1].trim().parse::<isize>().unwrap(),
                    speed_parts[2].trim().parse::<isize>().unwrap(),
                ),
            }
        }).collect::<Vec<Hailstone>>();
        let count = (0..hailstones.len())
            .combinations(2)
            .map(|pair| crosses(&hailstones[pair[0]], &hailstones[pair[1]], &min_range, &max_range) as i64)
            .sum();
        count
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(13149)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn crosses(s1: &Hailstone, s2: &Hailstone, min_range: &usize, max_range: &usize) -> u8 {
    let (a1, b1) = find_ab(s1);
    let (a2, b2) = find_ab(s2);
    if a1 == a2 && b1 == b2 {
        panic!("Same line");
    }
    if a1 == a2 {
        return 0;
    }
    let (x, y) = find_intersection(a1, b1, a2, b2);
    if x >= *min_range as f64 && x <= *max_range as f64 && y >= *min_range as f64 && y <= *max_range as f64 
        && is_in_future(s1, x, y) && is_in_future(s2, x, y) {
        return 1;
    }
    return 0;
}

fn is_in_future(s: &Hailstone, x: f64, y: f64) -> bool {
    abs(s.pos.x as isize + s.speed.x - x as isize) < abs(s.pos.x as isize - x as isize) 
        && abs(s.pos.y as isize + s.speed.y - y as isize) < abs(s.pos.y as isize - y as isize)
}

fn find_intersection(a1: f64, b1: f64, a2: f64, b2: f64) -> (f64, f64) {
    let x = (b2 - b1) / (a1 - a2);
    let y = a1 * x + b1;
    (x, y)
}

fn find_ab(s1: &Hailstone) -> (f64, f64) {
    let a = s1.speed.y as f64 / s1.speed.x as f64;
    let b = s1.pos.y as f64 - a * s1.pos.x as f64;
    (a, b)
}
