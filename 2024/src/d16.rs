use crate::{common::MapVector, DayTask};
use std::collections::HashMap;

pub struct Task;

const TI: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const TI2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        16
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI, TI2]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![7036, 11048]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![45, 64]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(95444)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        let map = parse(lines);
        let start = map.find('S').into_iter().next().expect("No S");
        let end = map.find('E').into_iter().next().expect("No E");
        find_shortest_path(&map, start, end, false)
    }

    fn run_p2(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        let map = parse(lines);
        let start = map.find('S').into_iter().next().expect("No S");
        let end = map.find('E').into_iter().next().expect("No E");
        find_shortest_path(&map, start, end, true)
    }
}

fn parse(lines: &Vec<String>) -> MapVector<char> {
    MapVector::new(lines, |c| c)
}

fn find_shortest_path(
    map: &MapVector<char>,
    start: crate::common::Point2D<isize>,
    end: crate::common::Point2D<isize>,
    find_all: bool,
) -> i64 {
    use crate::common::{Direction, Point2D};
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashSet};

    #[derive(Eq, PartialEq, Clone, Debug, Hash)]
    struct State {
        cost: i64,
        pos: Point2D<isize>,
        dir: Direction,
        path: Vec<Point2D<isize>>,
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut to_check = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut shortest_cost = None;
    let mut all_paths = Vec::new();
    // Start at 'S', facing east
    to_check.push(State {
        cost: 0,
        pos: start,
        dir: Direction::East,
        path: vec![start],
    });

    while let Some(State {
        cost,
        pos,
        dir,
        path,
    }) = to_check.pop()
    {
        if let Some(sc) = shortest_cost {
            if find_all && cost > sc {
                // Prune: only consider paths with cost <= shortest found
                continue;
            }
        }
        if pos == end {
            if !find_all {
                return cost;
            }
            if shortest_cost.is_none() {
                shortest_cost = Some(cost);
            } else if cost == shortest_cost.unwrap() {
                all_paths.push(path.clone());
            }
            continue;
        }
        if !visited.insert((pos, dir)) {
            continue;
        }

        // Try to go straight
        let next_pos = pos.move_dir(dir, 1);
        if map.is_in_map(next_pos) && map[&next_pos] != '#' {
            let mut new_path = path.clone();
            new_path.push(next_pos);
            to_check.push(State {
                cost: cost + 1,
                pos: next_pos,
                dir,
                path: new_path,
            });
        }

        // Try to turn left and right
        for turn in [Direction::turn_ccw, Direction::turn_cw] {
            let new_dir = turn(&dir);
            let next_pos = pos.move_dir(new_dir, 1);
            if map.is_in_map(next_pos) && map[&next_pos] != '#' {
                let mut new_path = path.clone();
                new_path.push(next_pos);
                to_check.push(State {
                    cost: cost + 1001,
                    pos: next_pos,
                    dir: new_dir,
                    path: new_path,
                });
            }
        }
    }
    if find_all && shortest_cost.is_some() {
        // Collect all unique tiles covered by all shortest paths
        let mut unique_tiles = HashSet::new();
        for path in all_paths {
            for tile in path {
                unique_tiles.insert(tile);
            }
        }
        unique_tiles.len() as i64
    } else {
        panic!("No path found");
    }
}
