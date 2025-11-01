use crate::{common::Point2D, DayTask};

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
        vec![875318608908]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(37686)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(77204516023437)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let machines = parse_machines(lines);
        machines.iter().map(|m| solve_one(&m, false)).sum()
    }

    fn run_p2(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        let machines = parse_machines(lines);
        machines.iter().map(|m| solve_one(&m, true)).sum()
    }
}

fn solve_one(m: &&Machine, add_factor: bool) -> i64 {
    let fact: isize = 10000000000000;
    let px = if add_factor {
        m.prize.x + fact
    } else {
        m.prize.x
    };
    let py = if add_factor {
        m.prize.y + fact
    } else {
        m.prize.y
    };

    let b_presses = (px as f64 - py as f64 * m.button_a.x as f64 / m.button_a.y as f64)
        / (m.button_b.x as f64 - m.button_b.y as f64 * m.button_a.x as f64 / m.button_a.y as f64);
    let a_presses = (py as f64 - b_presses * m.button_b.y as f64) / m.button_a.y as f64;
    let a = a_presses.round() as isize;
    let b = b_presses.round() as isize;
    let cost =
        if px == a * m.button_a.x + b * m.button_b.x && py == a * m.button_a.y + b * m.button_b.y {
            (a as i64) * 3 + (b as i64)
        } else {
            0
        };
    cost
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
