use itertools::Itertools;

use crate::{
    common::{Direction, Point2D},
    DayTask,
};

pub struct Task;

const TI: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        18
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        62
    }

    fn get_part2_test_result(&self) -> i64 {
        952408144115
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        count_inner_new(lines, true)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        count_inner_new(lines, false)
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(35991)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(54058824661845)
    }
}

fn parse_line_part1(line: &str) -> (char, i32) {
    let mut fields = line.split_ascii_whitespace();
    let dir = fields.next().unwrap().chars().next().unwrap();
    let steps = fields.next().unwrap().parse::<i32>().unwrap();
    (dir, steps)
}

fn parse_line_part2(line: &str) -> (char, i32) {
    let mut fields = line.split_ascii_whitespace();
    let hex = fields.nth(2).unwrap();
    let steps = i32::from_str_radix(&hex[2..=6], 16).unwrap();
    let raw_step = hex.as_bytes()[7] as char;
    let dir = match raw_step {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("Unknown direction"),
    };
    (dir, steps)
}

fn count_inner_new(lines: &Vec<String>, is_part1: bool) -> i64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let points = get_points_in_order(lines, is_part1);
    let mut total = 0i64;
    let mut border = 0;
    for (p1, p2) in points.iter().tuple_windows() {
        total += p1.x * p2.y - p2.x * p1.y;
        border += p1.manhattan_distance(p2)
    }
    let area = total.abs() as f64 / 2f64;
    // internal area + border length
    let res = (area + 0.5 * border as f64 + 1f64) + border as f64;
    res as i64
}

fn get_points_in_order(lines: &Vec<String>, use_part1_parser: bool) -> Vec<Point2D<i64>> {
    let mut current_pos = Point2D::new(0i64, 0);
    let mut points = vec![Point2D::new(0, 0)];
    points.extend(lines.iter().map(|line| {
        let (dir, steps) = if use_part1_parser {
            parse_line_part1(line)
        } else {
            parse_line_part2(line)
        };
        current_pos = current_pos.move_dir(Direction::from_char(dir), steps as i64);
        current_pos
    }));
    points
}
