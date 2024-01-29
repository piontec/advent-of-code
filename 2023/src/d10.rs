use crate::common::Point2D;
use crate::DayTask;
use core::panic;
use std::collections::HashMap;

pub struct Task;

const TI: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

impl DayTask<i32> for Task {
    fn day_no(&self) -> u8 {
        10
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i32 {
        8
    }

    fn get_part2_test_result(&self) -> i32 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i32 {
        let path = get_path(lines);
        path.len() as i32 / 2
    }

    fn run_p2(&self, lines: &Vec<String>) -> i32 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i32> {
        Some(6864)
    }

    fn get_part2_result(&self) -> Option<i32> {
        None
    }
}

fn get_path(lines: &Vec<String>) -> Vec<Point2D<usize>> {
    let mut s = Point2D::new(0, 0);
    for li in 0..lines.len() {
        let l = &lines[li];
        if l.contains("S") {
            s = Point2D::new(l.find("S").unwrap(), li);
            break;
        }
    }
    let mut current = Point2D::new(s.x + 1, s.y);
    let mut prev = s.clone();
    let mut path: Vec<Point2D<usize>> = vec![s.clone()];
    loop {
        // assuming going clockwise
        let next_symbol = lines[current.y].chars().nth(current.x).unwrap();
        let next_pos = match next_symbol {
            'F' => {
                // we come from south
                if prev.y == current.y + 1 {
                    Point2D::new(current.x + 1, current.y)
                }
                // we come from east
                else if prev.x == current.x + 1 {
                    Point2D::new(current.x, current.y + 1)
                } else {
                    panic!("Invalid direction")
                }
            }
            'J' => {
                // we come from north
                if prev.y == current.y - 1 {
                    Point2D::new(current.x - 1, current.y)
                }
                // we come from west
                else if prev.x == current.x - 1 {
                    Point2D::new(current.x, current.y - 1)
                } else {
                    panic!("Invalid direction")
                }
            }
            '7' => {
                // we come from south
                if prev.y == current.y + 1 {
                    Point2D::new(current.x - 1, current.y)
                }
                // we come from west
                else if prev.x == current.x - 1 {
                    Point2D::new(current.x, current.y + 1)
                } else {
                    panic!("Invalid direction")
                }
            }
            'L' => {
                // we come from north
                if prev.y == current.y - 1 {
                    Point2D::new(current.x + 1, current.y)
                }
                // we come from east
                else if prev.x == current.x + 1 {
                    Point2D::new(current.x, current.y - 1)
                } else {
                    panic!("Invalid direction")
                }
            }
            '-' => {
                // we come from west
                if prev.x == current.x - 1 {
                    Point2D::new(current.x + 1, current.y)
                }
                // we come from east
                else if prev.x == current.x + 1 {
                    Point2D::new(current.x - 1, current.y)
                } else {
                    panic!("Invalid direction")
                }
            }
            '|' => {
                // we come from north
                if prev.y == current.y - 1 {
                    Point2D::new(current.x, current.y + 1)
                }
                // we come from south
                else if prev.y == current.y + 1 {
                    Point2D::new(current.x, current.y - 1)
                } else {
                    panic!("Invalid direction")
                }
            }
            _ => panic!("Invalid character"),
        };
        path.push(current.clone());
        prev = current;
        current = next_pos;

        if current == s {
            return path;
        }
    }
}
