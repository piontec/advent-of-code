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
        let mut map = MapVector::new(lines);
        let starts = map.find('^');
        assert!(starts.len() == 1);
        map[starts[0]] = '.';
        let start = Point2D::<isize>::new(starts[0].x as isize, starts[0].y as isize);
        let path = find_path(&mut map, &start, &Direction::North);
        let visited = match path {
            Result::NoLoop(visited) => visited,
            _ => panic!("Unexpected"),
        };
        let res = visited.iter().map(|(point, _)| point).unique().count();
        res as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut map = MapVector::new(lines);
        let starts = map.find('^');
        assert!(starts.len() == 1);
        map[starts[0]] = '.';
        let mut current = Point2D::<isize>::new(starts[0].x as isize, starts[0].y as isize);
        let mut dir = Direction::North;
        let mut loops: Vec<Path> = vec![];
        loop {
            let next = current.move_dir(dir, 1);
            // if we're outside map boundaries - end the loop;
            if next.x < 0
                || next.y < 0
                || next.y >= map.map.len() as isize
                || next.x >= map.map[0].len() as isize
            {
                return loops.len() as i64;
            }
            if map[next] == '#' {
                dir = dir.turn_cw();
                continue;
            }
            let maybe_dir = dir.turn_cw();
            if let Some(_) = map.find_in_front(current, maybe_dir, '#') {
                map[next] = '#';
                if let Result::Loop(l) = find_path(&mut map, &current, &dir) {
                    if !loops.contains(&l) {
                        loops.push(l);
                    }
                }
                map[next] = '.';
            }
            current = next;
        }
    }
}

type Path = HashSet<(Point2D<isize>, Direction)>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Result {
    Loop(Path),
    NoLoop(Path),
}

fn find_path(map: &mut MapVector<char>, start: &Point2D<isize>, direction: &Direction) -> Result {
    let mut visited: Path = HashSet::new();
    let mut current = Point2D::<isize>::new(start.x, start.y);
    let mut dir = *direction;
    loop {
        visited.insert((current, dir));
        let next = current.move_dir(dir, 1);
        // if we're outside map boundaries - end the loop;
        if next.x < 0
            || next.y < 0
            || next.y >= map.map.len() as isize
            || next.x >= map.map[0].len() as isize
        {
            return Result::NoLoop(visited);
        }
        if visited.contains(&(next, dir)) {
            return Result::Loop(visited);
        }
        if map[next] == '#' {
            dir = dir.turn_cw();
            continue;
        }
        current = next;
    }
}
