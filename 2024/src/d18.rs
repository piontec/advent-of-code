use crate::{
    common::{Direction, MapVector, Point2D},
    DayTask,
};
use std::collections::VecDeque;

pub struct Task;

const TI: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        18
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![22]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![20]
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        let (size, bytes) = if _is_test {
            (Point2D { x: 7, y: 7 }, 12)
        } else {
            (Point2D { x: 71, y: 71 }, 1024)
        };
        let map = parse_input(lines, size, bytes);
        solve(map)
    }

    fn run_p2(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        let size = if _is_test {
            Point2D { x: 7, y: 7 }
        } else {
            Point2D { x: 71, y: 71 }
        };

        // Binary search to find the minimum bytes that blocks the path
        let mut left = 0;
        let mut right = lines.len();

        while left < right {
            let mid = (left + right) / 2;
            let map = parse_input(lines, size, mid);

            if solve(map) == -1 {
                // Path is blocked, try with fewer bytes
                right = mid;
            } else {
                // Path exists, try with more bytes
                left = mid + 1;
            }
        }

        // Return the index of the blocking byte (0-based, so subtract 1)
        println!("** {} **", lines[left - 1].as_str());
        (left - 1) as i64
    }
}

fn parse_input(lines: &Vec<String>, size: Point2D<u8>, bytes: usize) -> MapVector<u8> {
    let mut map = MapVector::<u8>::empty(
        Point2D {
            x: size.x as usize,
            y: size.y as usize,
        },
        0,
    );
    let mut parsed_bytes = 0;
    for line in lines {
        if parsed_bytes >= bytes {
            break;
        }
        let parts: Vec<&str> = line.split(',').collect();
        let x: usize = parts[0].parse().unwrap();
        let y: usize = parts[1].parse().unwrap();
        map[Point2D { x, y }] += 1;
        parsed_bytes += 1;
    }
    map
}

fn solve(map: MapVector<u8>) -> i64 {
    let start = Point2D {
        x: 0isize,
        y: 0isize,
    };
    let end = Point2D {
        x: map.get_size().x as isize - 1,
        y: map.get_size().y as isize - 1,
    };

    // BFS to find shortest path
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    queue.push_back((start, 0)); // (position, distance)
    visited.insert(start);

    // Directions: North, South, West, East
    let directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    while let Some((current, distance)) = queue.pop_front() {
        if current == end {
            return distance;
        }

        for &dir in &directions {
            let new_pos = current.move_dir(dir, 1);

            // Check bounds
            if new_pos.x < 0
                || new_pos.y < 0
                || new_pos.x >= map.get_size().x as isize
                || new_pos.y >= map.get_size().y as isize
            {
                continue;
            }

            // Check if not visited
            // Check if passable (value 0)
            if !visited.contains(&new_pos) && map[new_pos] == 0 {
                visited.insert(new_pos);
                queue.push_back((new_pos, distance + 1));
            }
        }
    }

    -1 // No path found
}
