use crate::{
    common::{Direction, MapVector, Path, Point2D},
    DayTask,
};
use std::collections::{HashMap, VecDeque};

pub struct Task;

const TI: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        return 20;
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![5] // Number of shortcuts saving at least 20 steps
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        let mut map = MapVector::new(lines, |c| c);

        // Find start and end positions
        let start = map.find('S')[0];
        let end = map.find('E')[0];
        map[start] = '.';
        map[end] = '.';

        // BFS to find shortest path
        let path = bfs_shortest_path(&map, start, end);
        let original_length = (path.len() - 1) as i64;

        // Analyze shortcuts with appropriate threshold
        let at_least_saves = if is_test { 20 } else { 100 };
        analyze_shortcuts(&map, &path, original_length, at_least_saves)
    }

    fn run_p2(&self, _lines: &Vec<String>, _is_test: bool) -> i64 {
        todo!()
    }
}

fn analyze_shortcuts(
    map: &MapVector<char>,
    path: &Path<isize>,
    original_length: i64,
    at_least_saves: i64,
) -> i64 {
    let start = path[0];
    let end = path[path.len() - 1];

    // Analyze path for wall-breaking shortcuts
    let mut path_length_savings = HashMap::new();
    let mut tested_walls = HashMap::new();

    // For each step in the path, analyze neighboring walls
    for step_pos in path {
        // Check all 4 directions for walls
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let wall_pos = step_pos.move_dir(direction, 1);

            if !(map.is_in_map(wall_pos) && map[wall_pos] == '#') {
                continue;
            }

            if tested_walls.contains_key(&wall_pos) {
                continue;
            }

            // Check if behind the wall (2 steps from current position) is passable
            let behind_wall_pos = step_pos.move_dir(direction, 2);
            if !(map.is_in_map(behind_wall_pos) && map[behind_wall_pos] == '.') {
                continue;
            }

            // Mark this wall as tested
            tested_walls.insert(wall_pos, true);

            // Create a modified map with this wall removed
            let mut modified_map = map.clone();
            modified_map[wall_pos] = '.';

            // Run BFS on modified map
            let modified_path = bfs_shortest_path(&modified_map, start, end);
            if !modified_path.is_empty() {
                let modified_savings = original_length - (modified_path.len() - 1) as i64;
                *path_length_savings.entry(modified_savings).or_insert(0) += 1;
            }
        }
    }

    // Return count of shortcuts that save at least the specified number of steps
    path_length_savings
        .iter()
        .filter(|&(&savings, _)| savings >= at_least_saves)
        .map(|(_, &count)| count)
        .sum()
}

fn bfs_shortest_path(
    map: &MapVector<char>,
    start: Point2D<isize>,
    end: Point2D<isize>,
) -> Path<isize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut parent = HashMap::new();

    queue.push_back(start);
    visited.insert(start, true);

    while let Some(current) = queue.pop_front() {
        if current == end {
            // Reconstruct path
            let mut path = Vec::new();
            let mut pos = current;
            path.push(pos);

            while let Some(&prev) = parent.get(&pos) {
                path.push(prev);
                pos = prev;
            }

            path.reverse();
            return path;
        }

        // Check all 4 directions
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let next_pos = current.move_dir(direction, 1);

            // Check if position is valid and not visited
            if map.is_in_map(next_pos) && !visited.contains_key(&next_pos) {
                let cell = map[next_pos];

                if cell == '.' {
                    visited.insert(next_pos, true);
                    parent.insert(next_pos, current);
                    queue.push_back(next_pos);
                }
            }
        }
    }

    // No path found
    Vec::new()
}
