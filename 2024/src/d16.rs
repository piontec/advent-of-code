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
        vec![TI, TI2]
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
    use std::collections::{BinaryHeap, HashMap, HashSet};

    #[derive(Eq, PartialEq, Clone, Debug, Hash)]
    struct State {
        cost: i64,
        pos: Point2D<isize>,
        dir: Direction,
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

    let mut heap = BinaryHeap::new();
    let mut visited: HashMap<(Point2D<isize>, Direction), i64> = HashMap::new();
    let mut predecessors: HashMap<(Point2D<isize>, Direction), Vec<(Point2D<isize>, Direction)>> =
        HashMap::new();
    let mut min_cost = None;
    // Start at 'S', facing east
    heap.push(State {
        cost: 0,
        pos: start,
        dir: Direction::East,
    });

    while let Some(State { cost, pos, dir }) = heap.pop() {
        if pos == end {
            if min_cost.is_none() {
                min_cost = Some(cost);
                if !find_all {
                    return min_cost.unwrap();
                }
            }
        }

        if let Some(mc) = min_cost {
            if cost > mc {
                continue;
            }
        }
        let key = (pos, dir);
        if let Some(&prev_cost) = visited.get(&key) {
            // we got to a place we know but with higher cost - this can't be the best path
            if cost > prev_cost {
                continue;
            }
        }
        visited.insert(key, cost);

        // Try to go straight
        let next_pos = pos.move_dir(dir, 1);
        if map.is_in_map(next_pos) && map[&next_pos] != '#' {
            let next_key = (next_pos, dir);
            let next_cost = cost + 1;
            let should_add = match visited.get(&next_key) {
                None => true,
                Some(&prev_cost) => next_cost <= prev_cost,
            };
            if should_add {
                heap.push(State {
                    cost: next_cost,
                    pos: next_pos,
                    dir,
                });
                // Only track predecessors for optimal paths
                match visited.get(&next_key) {
                    None => {
                        predecessors.insert(next_key, vec![key]);
                    }
                    Some(&prev_cost) => {
                        if next_cost < prev_cost {
                            predecessors.insert(next_key, vec![key]);
                        } else if next_cost == prev_cost {
                            predecessors.entry(next_key).or_default().push(key);
                        }
                    }
                }
            }
        }

        // Try to turn left and right
        for turn in [Direction::turn_ccw, Direction::turn_cw] {
            let new_dir = turn(&dir);
            let next_pos = pos.move_dir(new_dir, 1);
            if map.is_in_map(next_pos) && map[&next_pos] != '#' {
                let next_key = (next_pos, new_dir);
                let next_cost = cost + 1001;
                let should_add = match visited.get(&next_key) {
                    None => true,
                    Some(&prev_cost) => next_cost <= prev_cost,
                };
                if should_add {
                    heap.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        dir: new_dir,
                    });
                    // Only track predecessors for optimal paths
                    match visited.get(&next_key) {
                        None => {
                            predecessors.insert(next_key, vec![key]);
                        }
                        Some(&prev_cost) => {
                            if next_cost < prev_cost {
                                predecessors.insert(next_key, vec![key]);
                            } else if next_cost == prev_cost {
                                predecessors.entry(next_key).or_default().push(key);
                            }
                        }
                    }
                }
            }
        }
    }

    let end_states: Vec<(Point2D<isize>, Direction)> = visited
        .iter()
        .filter(|((p, _), &c)| *p == end && c == min_cost.unwrap())
        .map(|(k, _)| *k)
        .collect();

    // Backtrack all shortest paths
    let mut unique_tiles = HashSet::new();
    for end_state in end_states {
        // iterate through predecessors backwards until 'S' is found
        // add all positions to unique_tiles
        let mut stack = vec![end_state];
        let mut seen = HashSet::new();
        while let Some(state) = stack.pop() {
            if !seen.insert(state) {
                continue;
            }
            unique_tiles.insert(state.0);
            if state.0 == start {
                continue;
            }
            if let Some(preds) = predecessors.get(&state) {
                for &pred in preds {
                    stack.push(pred);
                }
            }
        }
    }
    unique_tiles.len() as i64
}
