use crate::{
    common::{MapHashMap, Point2D},
    DayTask,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Iter,
};

pub struct Task;

const TI: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        21
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        16
    }

    fn get_part2_test_result(&self) -> i64 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        let map = MapHashMap::parse_map(lines);
        let start = map.find('S')[0];
        let steps = if is_test { 6 } else { 64 };
        let shortest_paths = bfs_to_all(&map, *start);
        let gardens = shortest_paths
            .map
            .iter()
            .filter(|(_, &s)| s <= steps && s % 2 == 0)
            .count();
        gardens as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(3746)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn bfs_to_all(map: &MapHashMap<i32, char>, start: Point2D<i32>) -> MapHashMap<i32, i32> {
    let mut to_visit = VecDeque::from([(start, 0)]);
    let mut visited: MapHashMap<i32, i32> = MapHashMap::new();
    while let Some((pos, steps)) = to_visit.pop_front() {
        if visited.map.contains_key(&pos) {
            continue;
        }
        visited.map.insert(pos, steps);
        let (y, x) = (pos.y, pos.x);
        for next in [
            Point2D::new(x, y - 1),
            Point2D::new(x, y + 1),
            Point2D::new(x - 1, y),
            Point2D::new(x + 1, y),
        ]
        .iter()
        {
            if map.map.contains_key(&next)
                && map.map[next] != '#'
                && !visited.map.contains_key(&next)
            {
                to_visit.push_back((*next, steps + 1));
            }
        }
    }
    visited
}
