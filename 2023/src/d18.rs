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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum EdgeType {
    Horizontal,
    Vertical,
    ULC,
    URC,
    DLC,
    DRC,
}

impl EdgeType {
    fn is_corner(&self) -> bool {
        match self {
            EdgeType::ULC => true,
            EdgeType::URC => true,
            EdgeType::DLC => true,
            EdgeType::DRC => true,
            _ => false,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            EdgeType::Horizontal => true,
            _ => false,
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            EdgeType::Vertical => true,
            _ => false,
        }
    }
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
        let mut first_dir: Option<char> = None;
        let mut map: HashMap<Point2D<i32>, Tile> = HashMap::new();
        let mut last_dir: Option<char> = None;
        for line in lines {
            let mut fields = line.split_ascii_whitespace();
            let dir = fields.next().unwrap().chars().next().unwrap();
            let steps = fields.next().unwrap().parse::<i32>().unwrap();
            if last_dir.is_some_and(|ld| ld != dir) {
                *map.get_mut(&current_pos).unwrap() = Tile::Edge(get_edge_type(last_dir.unwrap(), dir));
            }
            last_dir = Some(dir);
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
                if first_dir.is_none() {
                    first_dir = Some(dir);
                }
                map.insert(current_pos, Tile::Edge(
                    if ['U', 'D'].contains(&dir) { 
                        EdgeType::Vertical
                    } else { 
                        EdgeType::Horizontal
                    }));
            }
        }
        *map.get_mut(&current_pos).unwrap() = Tile::Edge(get_edge_type(last_dir.unwrap(), first_dir.unwrap()));

        count_inner(&map)
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

fn count_inner(map: &HashMap<Point2D<i32>, Tile>) -> i64 {
    let mut count = 0;
    let y_min = map.keys().map(|p| p.y).min().unwrap();
    let y_max = map.keys().map(|p| p.y).max().unwrap();
    let x_min = map.keys().map(|p| p.x).min().unwrap();
    let x_max = map.keys().map(|p| p.x).max().unwrap();
    let mut inside = false;
    for y in y_min..=y_max {
        let mut last_corner: Option<EdgeType> = None;
        for x in x_min..=x_max {
            if let Some(tile) = map.get(&Point2D::new(x, y)) {
                match tile {
                    Tile::Edge(et) => {
                        if et.is_vertical() {
                            inside = !inside;
                        }
                        if et.is_corner() {
                            if last_corner.is_some() {
                                let lc = last_corner.unwrap();
                                if (lc == EdgeType::ULC && *et == EdgeType::DRC) || (lc == EdgeType::DLC && *et == EdgeType::URC) {
                                    inside = !inside;
                                }
                                last_corner = None;
                            }
                            else {
                                last_corner = Some(*et);
                            }
                        }
                        // and ignoring et.is_horizontal() as they don't change inside value
                    }
                    _ => {panic!("Unexpected tile")}
                }
                count += 1;
                continue;
            }
            if inside {
                count += 1;
            }
        }
    }
    count
}

fn get_edge_type(last_dir: char, dir: char) -> EdgeType {
    match (last_dir, dir) {
        ('U', 'L') => EdgeType::URC,
        ('U', 'R') => EdgeType::ULC,
        ('U', 'U') => EdgeType::Vertical,
        ('D', 'L') => EdgeType::DRC,
        ('D', 'R') => EdgeType::DLC,
        ('D', 'D') => EdgeType::Vertical,
        ('L', 'U') => EdgeType::DLC,
        ('L', 'D') => EdgeType::ULC,
        ('L', 'L') => EdgeType::Horizontal,
        ('R', 'U') => EdgeType::DRC,
        ('R', 'D') => EdgeType::URC,
        ('R', 'R') => EdgeType::Horizontal,
        _ => panic!("Unknown edge"),
    }
}
