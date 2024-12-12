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
        find_path(&mut map, &start, &Direction::North);
        map.find('X').len() as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut map = MapVector::new(lines);
        let starts = map.find('^');
        assert!(starts.len() == 1);
        map[starts[0]] = '.';
        let mut current = Point2D::<isize>::new(starts[0].x as isize, starts[0].y as isize);
        let mut dir = Direction::North;
        let mut loop_counter = 0;
        loop {
            let next = current.move_dir(dir, 1);
            // if we're outside map boundaries - end the loop;
            if next.x < 0
                || next.y < 0
                || next.y >= map.map.len() as isize
                || next.x >= map.map[0].len() as isize
            {
                return loop_counter;
            }
            if map[next] == '#' {
                dir = dir.turn_cw();
                continue;
            }
            let maybe_dir = dir.turn_cw();
            if let Some(_) = map.find_in_front(current, maybe_dir, '#') {
                let mut maybe_map = map.clone();
                maybe_map[next] = '#';
                if find_path(&mut maybe_map, &current, &dir) == Result::Loop {
                    loop_counter += 1;
                }
            }
            current = next;
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Result {
    Loop,
    NoLoop,
}

fn find_path(map: &mut MapVector<char>, start: &Point2D<isize>, direction: &Direction) -> Result {
    let mut visited: HashSet<(Point2D<isize>, Direction)> = HashSet::new();
    let mut current = Point2D::<isize>::new(start.x, start.y);
    let mut dir = *direction;
    loop {
        map[current] = 'X';
        visited.insert((current, dir));
        let next = current.move_dir(dir, 1);
        // if we're outside map boundaries - end the loop;
        if next.x < 0
            || next.y < 0
            || next.y >= map.map.len() as isize
            || next.x >= map.map[0].len() as isize
        {
            return Result::NoLoop;
        }
        if visited.contains(&(next, dir)) {
            return Result::Loop;
        }
        if map[next] == '#' {
            dir = dir.turn_cw();
            continue;
        }
        current = next;
    }
}
