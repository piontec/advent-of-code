use crate::common::Point2D;
use crate::DayTask;
use core::panic;
use std::collections::HashMap;
use std::hash::Hash;

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
        1
    }

    fn run_p1(&self, lines: &Vec<String>) -> i32 {
        get_path(lines).len() as i32 / 2
    }

    fn run_p2(&self, lines: &Vec<String>) -> i32 {
        let path = get_path(lines);
        let mut in_counter = 0;

        for y in 0..lines.len() {
            let mut wall_count = 0;
            let mut horizontal_start = None;
            for x in 0..lines[y].len() {
                let p = Point2D::new(x, y);
                if path.contains_key(&p) {
                    let path_element = path.get(&p).unwrap();
                    match path_element {
                        'F' | 'L' => {
                            horizontal_start = Some(path_element);
                        }
                        'J' => {
                            if *horizontal_start.unwrap() == 'F' {
                                wall_count += 1;
                            }
                        }
                        '7' => {
                            if *horizontal_start.unwrap() == 'L' {
                                wall_count += 1;
                            }
                        }
                        '-' => { }
                        '|' => {
                            wall_count += 1;
                        }
                        _ => panic!("Invalid character"),
                        
                    }
                }
                else if wall_count % 2 == 1 {
                    in_counter += 1;
                }
            }
        }

        in_counter
    }

    fn get_part1_result(&self) -> Option<i32> {
        Some(6864)
    }

    fn get_part2_result(&self) -> Option<i32> {
        None
    }
}

fn get_path(lines: &Vec<String>) -> HashMap<Point2D<usize>, char> {
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
    // I checked and in all cases 'S' is 'F'
    let mut path: HashMap<Point2D<usize>, char> = HashMap::new();
    path.insert(s, 'F');

    loop {
        // assuming going clockwise
        let current_symbol = lines[current.y].chars().nth(current.x).unwrap();
        let next_pos = match current_symbol {
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
        path.insert(current.clone(), current_symbol);
        prev = current;
        current = next_pos;

        if current == s {
            return path;
        }
    }
}
