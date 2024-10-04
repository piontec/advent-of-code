use crate::{common::Point2D, DayTask};

pub struct Task;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Beam {
    pos: Point2D<i8>,
    dir: Direction,
}

#[derive(Clone)]
struct MapPosition {
    char: char,
    beams: Vec<Direction>,
}

const TI: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        16
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        46
    }

    fn get_part2_test_result(&self) -> i64 {
        51
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        let map = create_map(lines);
        run_one(map, Beam {
            pos: Point2D::new(0, 0),
            dir: Direction::East,
        })
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        let map = create_map(lines);
        let mut best_score = 0;

        for y in 0..map.len() {
            for x in [0, map[0].len() - 1] {
                let score = run_one(map.clone(), Beam {
                    pos: Point2D::new(x as i8, y as i8),
                    dir: if x == 0 {Direction::East} else {Direction::West},
                });   
                if score > best_score {
                    best_score = score;
                }
            }
        }
        for x in 0..map[0].len() {
            for y in [0, map.len() - 1] {
                let score = run_one(map.clone(), Beam {
                    pos: Point2D::new(x as i8, y as i8),
                    dir: if y == 0 {Direction::South} else {Direction::North},
                });   
                if score > best_score {
                    best_score = score;
                }
            }
        }
        best_score
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(7927)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(8246)
    }
}

fn create_map(lines: &Vec<String>) -> Vec<Vec<MapPosition>> {
    let map = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| MapPosition {
                    char: c,
                    beams: Vec::new(),
                })
                .collect::<Vec<MapPosition>>()
        })
        .collect::<Vec<Vec<MapPosition>>>();
    map
}

fn pos_in_map(pos: &Point2D<i8>, map: &Vec<Vec<MapPosition>>) -> bool {
    pos.x >= 0
        && pos.y >= 0
        && pos.y < map.len() as i8
        && pos.x < map[0].len() as i8
}

fn run_one(mut map: Vec<Vec<MapPosition>>, start: Beam) -> i64 {
    let mut beams = vec![start];

    while let Some(beam) = beams.pop() {
        let pos = beam.pos;
        let dir = beam.dir;

        // check if we already entered that position with the same direction
        if map[pos.y as usize][pos.x as usize].beams.contains(&dir) {
            continue;
        }
        map[pos.y as usize][pos.x as usize].beams.push(dir);

        // let's process the tile
        let tile = map[pos.y as usize][pos.x as usize].char;
        // case: just continue (empty or split along its way)
        if tile == '.' 
            || (tile == '|' && (dir == Direction::South || dir == Direction::North)) 
            || (tile == '-' && (dir == Direction::West || dir == Direction::East)) {
            let next_pos = match dir {
                Direction::North => Point2D::new(pos.x, pos.y - 1),
                Direction::East => Point2D::new(pos.x + 1, pos.y),
                Direction::South => Point2D::new(pos.x, pos.y + 1),
                Direction::West => Point2D::new(pos.x - 1, pos.y),
            };
            if pos_in_map(&next_pos, &map) {
                beams.push(Beam {
                    pos: next_pos,
                    dir: dir,
                });
            }
            continue;
        }
        // case: | splitter from a side
        if tile == '|' && (dir == Direction::West || dir == Direction::East) {
            let up = Point2D::new(pos.x, pos.y - 1);
            if pos_in_map(&up, &map) {
                beams.push(Beam {
                    pos: up,
                    dir: Direction::North,
                });
            }
            let down = Point2D::new(pos.x, pos.y + 1);
            if pos_in_map(&down, &map) {
                beams.push(Beam {
                    pos: down,
                    dir: Direction::South,
                });
            }
            continue;
        }
        // case: - splitter from a side
        if tile == '-' && (dir == Direction::South || dir == Direction::North) {
            let left = Point2D::new(pos.x - 1, pos.y);
            if pos_in_map(&left, &map) {
                beams.push(Beam {
                    pos: left,
                    dir: Direction::West,
                });
            }
            let right = Point2D::new(pos.x + 1, pos.y);
            if pos_in_map(&right, &map) {
                beams.push(Beam {
                    pos: right,
                    dir: Direction::East,
                });
            }
            continue;
        }
        // the last case: reflection from a mirror
        let new_pos: Point2D<i8>;
        let new_dir = match tile {
            '/' => match dir {
                Direction::North => {new_pos = Point2D::new(pos.x + 1, pos.y); Direction::East},
                Direction::East => {new_pos = Point2D::new(pos.x, pos.y - 1); Direction::North},
                Direction::South => {new_pos = Point2D::new(pos.x - 1, pos.y); Direction::West},
                Direction::West => {new_pos = Point2D::new(pos.x, pos.y + 1); Direction::South},
            },
            '\\' => match dir {
                Direction::North => {new_pos = Point2D::new(pos.x - 1, pos.y); Direction::West},
                Direction::East => {new_pos = Point2D::new(pos.x, pos.y + 1); Direction::South},
                Direction::South => {new_pos = Point2D::new(pos.x + 1, pos.y); Direction::East},
                Direction::West => {new_pos = Point2D::new(pos.x, pos.y - 1); Direction::North},
            },
            _ => {
                panic!("Unexpected character: {}", tile);
            }
        };
        if pos_in_map(&new_pos, &map) {
            beams.push(Beam {
                pos: new_pos,
                dir: new_dir,
            });
        }
    }

    let res: usize = map
        .iter()
        .map(|row| row.iter().filter(|e| e.beams.len() > 0).count())
        .sum();
    res as i64
}