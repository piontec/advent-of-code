use crate::{common::Point2D, DayTask};
use std::{collections::HashMap, vec};

pub struct Task;

const TI: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

struct Machine {
    button_a: Point2D<isize>,
    button_b: Point2D<isize>,
    prize: Point2D<isize>,
}

struct Solution {
    a_presses: isize,
    b_presses: isize,
    cost: i64,
}

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        13
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![480]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(37686)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let machines = parse_machines(lines);
        machines.iter().map(|m| solve_one(&m)).sum()
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }
}

fn solve_one(m: &&Machine) -> i64 {
    let b_presses = (m.prize.x as f64
        - m.prize.x as f64 * m.button_a.x as f64 / m.button_a.y as f64)
        / (m.button_b.x as f64 - m.button_b.y as f64 * m.button_a.x as f64 / m.button_a.y as f64);
    let a_presses = (m.prize.y as f64 - b_presses * m.button_b.y as f64) / m.button_a.y as f64;
    if a_presses.fract() != 0.0 || b_presses.fract() != 0.0 {
        0
    } else {
        (a_presses as i64) * 3 + (b_presses as i64)
    }
}

fn solve_one_old(m: &&Machine) -> i64 {
    let mut cur_a_presses = 0;
    let mut solutions = vec![];

    while cur_a_presses * m.button_a.x <= m.prize.x && cur_a_presses * m.button_a.y <= m.prize.y {
        // A costs 3, B costs 1
        let missing_x = m.prize.x - cur_a_presses * m.button_a.x;
        let missing_y = m.prize.y - cur_a_presses * m.button_a.y;
        let missing_b_x = missing_x / m.button_b.x;
        let missing_b_y = missing_y / m.button_b.y;
        if missing_b_x == missing_b_y
            && cur_a_presses * m.button_a.x + missing_b_x * m.button_b.x == m.prize.x
            && cur_a_presses * m.button_a.y + missing_b_y * m.button_b.y == m.prize.y
        {
            let solution = Solution {
                a_presses: cur_a_presses,
                b_presses: missing_b_x as isize,
                cost: cur_a_presses as i64 * 3 + missing_b_x as i64,
            };
            solutions.push(solution);
        }
        cur_a_presses += 1;
    }

    if solutions.is_empty() {
        0
    } else {
        solutions.iter().min_by_key(|s| s.cost).unwrap().cost
    }
}

fn parse_machines(lines: &[String]) -> Vec<Machine> {
    let mut machines = Vec::new();
    let mut li = 0;
    loop {
        let mut current_machine = Machine {
            button_a: Point2D::new(0, 0),
            button_b: Point2D::new(0, 0),
            prize: Point2D::new(0, 0),
        };
        let line_a = &lines[li];
        let line_b = &lines[li + 1];
        let line_p = &lines[li + 2];

        line_a[10..].split(", ").for_each(|part| {
            let value: isize = part[1..].parse().unwrap();
            match &part[0..1] {
                "X" => current_machine.button_a.x = value,
                "Y" => current_machine.button_a.y = value,
                _ => panic!("Unexpected"),
            }
        });
        line_b[10..].split(", ").for_each(|part| {
            let value: isize = part[1..].parse().unwrap();
            match &part[0..1] {
                "X" => current_machine.button_b.x = value,
                "Y" => current_machine.button_b.y = value,
                _ => panic!("Unexpected"),
            }
        });
        line_p[7..].split(", ").for_each(|part| {
            let value: isize = part[2..].parse().unwrap();
            match &part[0..1] {
                "X" => current_machine.prize.x = value,
                "Y" => current_machine.prize.y = value,
                _ => panic!("Unexpected"),
            }
        });
        machines.push(current_machine);
        if li + 6 <= lines.len() {
            li += 4;
        } else {
            break;
        }
    }

    machines
}
