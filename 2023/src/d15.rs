use crate::DayTask;

pub struct Task;

const TI: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    power: u8,
}

enum Op {
    Remove,
    Add,
}

impl DayTask<i64> for Task {

    fn day_no(&self) -> u8 {
        15
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        1320
    }

    fn get_part2_test_result(&self) -> i64 {
        145
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let res: usize = lines[0]
            .split(",")
            .map(|s| get_hash(s))
            .sum();
        res as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let instructions: Vec<&str> = lines[0].split(",").collect();
        let mut boxes = vec![Vec::<Lens>::new(); 256];

        for inst in instructions {
            let parts = inst.split(['=', '-']).collect::<Vec<&str>>();
            let label = parts[0];
            let op = if parts[1] == "" { Op::Remove } else { Op::Add };
            let box_index = get_hash(label);
            match op {
                Op::Add => {
                    if let Some(pos) = boxes[box_index].iter().position(|l| l.label == label) {
                        boxes[box_index][pos].power = parts[1].parse::<u8>().unwrap();
                    }
                    else {
                        boxes[box_index].push(
                            Lens {
                                label: label.to_string(),
                                power: parts[1].parse::<u8>().unwrap(),
                            }
                        );
                    }
                }
                Op::Remove => {
                    if let Some(pos) = boxes[box_index].iter().position(|l| l.label == label) {
                        boxes[box_index].remove(pos);
                    }
                }
            }
        }
        let focus_pow = boxes
            .iter()
            .enumerate()
            .map(|(box_index, b)| {
                let box_pow = b
                .iter()
                .enumerate()
                .map(|(lens_index, l)| (box_index + 1)*(lens_index +1)*l.power as usize)
                .sum::<usize>();
                box_pow
            })
            .sum::<usize>();
        focus_pow as i64
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(515495)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(229349)
    }
}

fn get_hash(s: &str) -> usize {
    let mut res: usize = 0;
    for c in s.chars() {
        res = ((res + c as usize) * 17) % 256;
    }
    res
}
