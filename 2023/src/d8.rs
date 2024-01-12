use crate::DayTask;
use std::collections::HashMap;
use num::integer::lcm;

pub struct Task;

const TI: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const TI2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

struct ZInfo {
    first_z_index: usize,
    cycle_length: usize,
}

impl DayTask<usize> for Task {
    fn day_no(&self) -> u8 {
        8
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI2
    }

    fn get_part1_test_result(&self) -> usize {
        6
    }

    fn get_part2_test_result(&self) -> usize {
        6
    }

    fn run_p1(&self, lines: &Vec<String>) -> usize {
        do_steps(lines, |e| e == "AAA", |e| e == "ZZZ")
    }

    fn run_p2(&self, lines: &Vec<String>) -> usize {
        do_cycles(lines)
    }

    fn get_part1_result(&self) -> Option<usize> {
        Some(20659)
    }

    fn get_part2_result(&self) -> Option<usize> {
        Some(15690466351717)
    }
}

fn do_cycles(lines: &Vec<String>) -> usize {
    let (choices, map) = parse(lines);
    let start_elements = map
        .keys()
        .filter(|e| e.ends_with("A"))
        .map(|e| *e)
        .collect::<Vec<&str>>();

    let mut cycle_info: HashMap<&str, ZInfo> = HashMap::new();
    for el in &start_elements {
        let mut choice_index = 0;
        let mut first_z = None;
        let mut current = *el;
        let mut counter = 0;
        let mut z_info = ZInfo {
            first_z_index: 0,
            cycle_length: 0,
        };
        loop {
            let choice = choices[choice_index];
            current = if choice == 'L' {
                map[current].0
            } else {
                map[current].1
            };
            counter += 1;
            if current.ends_with("Z") {
                if first_z.is_none() {
                    first_z = Some(current.to_string());
                    z_info.first_z_index = counter;
                } else {
                    z_info.cycle_length = counter - z_info.first_z_index;
                    cycle_info.insert(el, z_info);
                    if current != first_z.unwrap() {
                        panic!("Found 2nd z, but it's not the same as the first one");
                    }
                    break;
                }
            }
            choice_index = (choice_index + 1) % choices.len();
        }
    }

    let cycle_lengths = cycle_info
        .values()
        .map(|z| z.cycle_length)
        .collect::<Vec<usize>>();
    cycle_lengths.iter().fold(1, |acc, &x| lcm(acc, x))
}

fn do_steps(
    lines: &Vec<String>,
    starter_predicate: fn(&str) -> bool,
    stop_condition: fn(&str) -> bool,
) -> usize {
    let (choices, map) = parse(lines);
    let mut index = 0;
    let mut counter = 0;
    let mut current_elements = map
        .keys()
        .filter(|e| starter_predicate(e))
        .map(|e| *e)
        .collect::<Vec<&str>>();
    loop {
        let mut new_elements = Vec::new();
        let choice = choices[index];
        for current in &current_elements {
            let next = if choice == 'L' {
                map[*current].0
            } else {
                map[*current].1
            };
            new_elements.push(next);
        }
        counter += 1;
        current_elements = new_elements;
        if current_elements.iter().all(|e| stop_condition(e)) {
            break;
        }
        index = (index + 1) % choices.len();
    }

    counter
}

fn parse(lines: &Vec<String>) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let choices = lines[0].chars().collect::<Vec<char>>();
    let map = lines[2..]
        .iter()
        .map(|l| {
            let parts = l.split(" = ").collect::<Vec<&str>>();
            let lr = parts[1]
                .trim_matches(&['(', ')'])
                .split(", ")
                .collect::<Vec<&str>>();
            (parts[0], (lr[0], lr[1]))
        })
        .collect::<HashMap<&str, (&str, &str)>>();
    (choices, map)
}
