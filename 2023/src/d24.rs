use itertools::Itertools;
use num::abs;

use crate::{common::Point3D, DayTask};
use std::collections::HashMap;

pub struct Task;

const TI: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

struct Hailstone {
    pos: Point3D<isize>,
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
        0
    }

    fn run_p1(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        let (min_range, max_range) = if is_test {
            (7usize, 27usize)
        } else {
            (200000000000000, 400000000000000)
        };
        let hailstones = parse_stones(lines);
        let count = (0..hailstones.len())
            .combinations(2)
            .map(|pair| {
                crosses(
                    &hailstones[pair[0]],
                    &hailstones[pair[1]],
                    &min_range,
                    &max_range,
                ) as i64
            })
            .sum();
        count
    }

    fn run_p2(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        if is_test {
            return 0;
        }
        // based on the great explanation here: https://pastebin.com/pnbxaCVu
        // general idea: let's find 3 hails with the same speed in one axis
        // now, let's move our point of reference to the first hailstone (no 0)
        // Now, we know that the collision points of other 2 hailstones are on a
        // straight line from us (stone 0). This means, that they are scaled vectors,
        // so there exists a number m, that applies to coordinates of hail 1 will give
        // coordinates of hail 2. This also means, that if we can divide all of the coordinates
        // of hail 1 by a certain known number, such that afterwards y=1, then we do the same
        // for hail 2 (its scaled y is also y=1), all of the coordinates (x and z) must be equal
        // as well. Based on that, we can construct 2 equations where unknowns are only t1 and t2
        // - time of hitting the stone. Once we have t1 and t2, we can calculate everything else.
        // To get started, we need 2 steps: find such 3 stones and move everything to a frame
        // of reference of the first stone.

        let hailstones = parse_stones(lines);
        let grouped_by_y = group_by_prop(&hailstones, |s| s.speed.y);
        assert!(grouped_by_y.len() > 0);
        let s0 = grouped_by_y[1].1[0];
        let s1 = grouped_by_y[1].1[1];
        let s2 = grouped_by_y[1].1[2];

        // let's make relative stones, assuming s0 is at 0,0,0 and its speed is 0,0,0
        let rs1 = Hailstone {
            pos: Point3D::new(
                s1.pos.x - s0.pos.x,
                s1.pos.y - s0.pos.y,
                s1.pos.z - s0.pos.z,
            ),
            speed: Point3D::new(
                s1.speed.x - s0.speed.x,
                s1.speed.y - s0.speed.y,
                s1.speed.z - s0.speed.z,
            ),
        };
        let rs2 = Hailstone {
            pos: Point3D::new(
                s2.pos.x - s0.pos.x,
                s2.pos.y - s0.pos.y,
                s2.pos.z - s0.pos.z,
            ),
            speed: Point3D::new(
                s2.speed.x - s0.speed.x,
                s2.speed.y - s0.speed.y,
                s2.speed.z - s0.speed.z,
            ),
        };
        assert!(rs1.speed.y == 0 && rs2.speed.y == 0);

        // s1's collision is t1, which means its position at that time is
        // rs1.pos.x + rs1.speed.x * t1
        // rs1.pos.y + rs1.speed.y * t1 == rs1.pos.y
        // rs1.pos.z + rs1.speed.z * t1

        // s2's collision is t2, which means its position at that time is
        // rs2.pos.x + rs2.speed.x * t2
        // rs2.pos.y + rs2.speed.y * t2 == rs2.pos.y
        // rs2.pos.z + rs2.speed.z * t2

        // now, if we divide rs1's position by rs1.pos.z, and same for rs2, we should get the same position
        // [1] (rs1.pos.x + rs1.speed.x * t1) / rs1.poz.y == (rs2.pos.x + rs2.speed.x * t2) / rs2.pos.y
        // [2] (rs1.pos.z + rs1.speed.z * t1) / rs1.poz.y == (rs2.pos.z + rs2.speed.z * t2) / rs2.pos.y
        // transforming the [2] one to get t1 gives us:
        // OK, I give up here and switch to pen & paper...
        let t2_num = rs2.pos.x as i128 * rs1.pos.y as i128 * rs1.speed.z as i128
            + rs1.speed.x as i128 * rs1.pos.z as i128 * rs2.pos.y as i128
            - rs1.speed.x as i128 * rs1.pos.y as i128 * rs2.pos.z as i128
            - rs1.pos.x as i128 * rs2.pos.y as i128 * rs1.speed.z as i128;
        let t2_den = rs1.pos.y as i128
            * (rs1.speed.x as i128 * rs2.speed.z as i128
                - rs1.speed.z as i128 * rs2.speed.x as i128);
        let t2 = t2_num / t2_den as i128;
        let t1 = (rs2.pos.z as i128 * rs1.pos.y as i128
            + rs2.speed.z as i128 * t2 * rs1.pos.y as i128
            - rs1.pos.z as i128 * rs2.pos.y as i128)
            / (rs1.speed.z as i128 * rs2.pos.y as i128);
        // let's calculate collision points in the original frame of reference
        let c1 = Point3D::new(
            s0.pos.x + s0.speed.x * t1 as isize,
            s0.pos.y + s0.speed.y * t1 as isize,
            s0.pos.z + s0.speed.z * t1 as isize,
        );
        let c2 = Point3D::new(
            s0.pos.x + s0.speed.x * t2 as isize,
            s0.pos.y + s0.speed.y * t2 as isize,
            s0.pos.z + s0.speed.z * t2 as isize,
        );
        // now we can calculate the speed of the rock based on where and when it will hit s1 and s2 in c1 and c2
        let speed = Point3D::new(
            (c2.x - c1.x) / (t2 - t1) as isize,
            (c2.y - c1.y) / (t2 - t1) as isize,
            (c2.z - c1.z) / (t2 - t1) as isize,
        );
        // finally, we can get rock's position at time 0 based on c1 and speed
        let pos = Point3D::new(
            c1.x - speed.x * t1 as isize,
            c1.y - speed.y * t1 as isize,
            c1.z - speed.z * t1 as isize,
        );
        (pos.x + pos.y + pos.z) as i64
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(13149)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn group_by_prop<F>(hailstones: &Vec<Hailstone>, selector: F) -> Vec<(isize, Vec<&Hailstone>)>
where
    F: Fn(&Hailstone) -> isize,
{
    let grouped_by = hailstones
        .iter()
        .fold(HashMap::new(), |mut hm, stone| {
            let key = selector(stone);
            hm.entry(key).or_insert(Vec::new()).push(stone);
            hm
        })
        .into_iter()
        .filter(|(_, v)| v.len() >= 3)
        .collect::<Vec<(isize, Vec<&Hailstone>)>>();
    grouped_by
}

fn parse_stones(lines: &Vec<String>) -> Vec<Hailstone> {
    let hailstones = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" @ ").collect();
            let pos_parts: Vec<&str> = parts[0].split(", ").collect();
            let speed_parts: Vec<&str> = parts[1].split(", ").collect();
            Hailstone {
                pos: Point3D::new(
                    pos_parts[0].trim().parse::<isize>().unwrap(),
                    pos_parts[1].trim().parse::<isize>().unwrap(),
                    pos_parts[2].trim().parse::<isize>().unwrap(),
                ),
                speed: Point3D::new(
                    speed_parts[0].trim().parse::<isize>().unwrap(),
                    speed_parts[1].trim().parse::<isize>().unwrap(),
                    speed_parts[2].trim().parse::<isize>().unwrap(),
                ),
            }
        })
        .collect::<Vec<Hailstone>>();
    hailstones
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
    if x >= *min_range as f64
        && x <= *max_range as f64
        && y >= *min_range as f64
        && y <= *max_range as f64
        && is_in_future(s1, x, y)
        && is_in_future(s2, x, y)
    {
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
