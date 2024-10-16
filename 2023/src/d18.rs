use crate::{common::Point2D, DayTask};
use std::collections::HashMap;

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

enum EdgeType {
    Horizontal,
    Vertical,
    ULC,
    URC,
    DLC,
    DRC,
}

enum Tile {
    Empty,
    Edge(EdgeType),
    Inner,
}

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
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        let mut current_pos = Point2D::new(0,0);
        let mut map: HashMap<Point2D<i32>, Tile> = HashMap::new();
        let mut last_dir: Option<char> = None;
        for line in lines {
            let mut fields = line.split_ascii_whitespace();
            let dir = fields.next().unwrap().chars().next().unwrap();
            let steps = fields.next().unwrap().parse::<i32>().unwrap();
            // we skip the point (0,0), so we should end up in it
            for _ in 0..steps {
                match dir {
                    'U' => {
                        current_pos = current_pos.move_dxy(0, 1);
                    }
                    'D' => {
                        current_pos = current_pos.move_dxy(0, -1);
                    }
                    'L' => {
                        current_pos = current_pos.move_dxy(-1, 0);
                    }
                    'R' => {
                        current_pos = current_pos.move_dxy(1, 0);
                    }
                    _ => panic!("Unknown direction"),
                }
                let edge_type = match (last_dir, dir) {
                    (Some('U'), 'U') => EdgeType::Vertical,
                    (Some('U'), 'L') => EdgeType::URC,
                    (Some('U'), 'R') => EdgeType::ULC,
                    (Some('D'), 'D') => EdgeType::Vertical,
                    (Some('D'), 'L') => EdgeType::DRC,
                    (Some('D'), 'R') => EdgeType::DLC,
                    (Some('L'), 'L') => EdgeType::Horizontal,
                    (Some('L'), 'U') => EdgeType::DLC,
                    (Some('L'), 'D') => EdgeType::ULC,
                    (Some('R'), 'R') => EdgeType::Horizontal,
                    (Some('R'), 'U') => EdgeType::DRC,
                    (Some('R'), 'D') => EdgeType::URC,
                    _ => match dir {
                        'U' => EdgeType::Vertical,
                        'D' => EdgeType::Vertical,
                        'L' => EdgeType::Horizontal,
                        'R' => EdgeType::Horizontal,
                        _ => panic!("Unknown direction"),
                    },
                };
                map.insert(current_pos, Tile::Edge(edge_type));
            }
            // TODO: handle last element looping to the first one
        }
        todo!()
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}
