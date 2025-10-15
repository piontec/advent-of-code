use itertools::Itertools;

use crate::{
    common::{Direction, MapVector, Point2D},
    DayTask,
};
use std::collections::HashSet;

pub struct Task;

const TI: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        6
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![41]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![6]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(5329)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut map = MapVector::new(lines, |c| c);
        let starts = map.find('^');
        assert!(starts.len() == 1);
        map[starts[0]] = '.';
        let start = Point2D::<isize>::new(starts[0].x as isize, starts[0].y as isize);
        let mut visited = HashSet::new();
        let path = find_path(&mut map, &mut visited, &start, &Direction::North);
        match path {
            PathResult::NoLoop => {
                // visited is already populated by find_path
            }
            _ => panic!("Unexpected"),
        };
        let res = visited.iter().map(|(point, _)| point).unique().count();
        res as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut map = MapVector::new(lines, |c| c);
        let starts = map.find('^');
        assert!(starts.len() == 1);
        map[starts[0]] = '.';
        let mut current = Point2D::<isize>::new(starts[0].x as isize, starts[0].y as isize);
        let mut dir = Direction::North;
        let mut visited = HashSet::new();
        let mut obstacles: HashSet<Point2D<isize>> = HashSet::new();
        loop {
            visited.insert((current, dir));
            let (next_pos, next_dir) = get_next_pos(&mut map, &current, &dir);
            // if we're outside map boundaries - end the loop;
            if !map.is_in_map(next_pos) {
                return obstacles.iter().unique().count() as i64;
            }
            map[next_pos] = '#';
            let res = find_path(&mut map, &mut visited.clone(), &current, &dir);
            if res == PathResult::Loop {
                obstacles.insert(next_pos);
            }
            map[next_pos] = '.';
            current = next_pos;
            dir = next_dir;
        }
    }
}

type Path = HashSet<(Point2D<isize>, Direction)>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum PathResult {
    Loop,
    NoLoop,
}

fn find_path(
    map: &mut MapVector<char>,
    visited: &mut Path,
    start: &Point2D<isize>,
    direction: &Direction,
) -> PathResult {
    let mut current = Point2D::<isize>::new(start.x, start.y);
    let mut dir = *direction;
    loop {
        visited.insert((current, dir));
        let (next, new_dir) = get_next_pos(map, &current, &dir);
        // if we're outside map boundaries - end the loop;
        if !map.is_in_map(next) {
            return PathResult::NoLoop;
        }
        if visited.contains(&(next, new_dir)) {
            return PathResult::Loop;
        }
        current = next;
        dir = new_dir;
    }
}

fn get_next_pos(
    map: &mut MapVector<char>,
    current: &Point2D<isize>,
    dir: &Direction,
) -> (Point2D<isize>, Direction) {
    let mut next = current.move_dir(*dir, 1);
    let mut new_dir = *dir;
    loop {
        if map.is_in_map(next) && map[next] == '#' {
            new_dir = new_dir.turn_cw();
            next = current.move_dir(new_dir, 1);
        } else {
            break;
        }
    }
    return (next, new_dir);
}
