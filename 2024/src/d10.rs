use crate::{
    common::{MapVector, Point2D, Path},
    DayTask,
};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Task;

const TI: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        10
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![36]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![81]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(744)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(1651)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, true)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, false)
    }

}

fn run(lines: &Vec<String>, unique: bool) -> i64 {
    let map: MapVector<i8> = MapVector::new(lines, |c| (c.to_digit(10).unwrap()) as i8);
    let starts = map.find(0);
    starts.iter().map(|start| get_quality(&map, start, unique)).sum()
}

fn get_quality(map: &MapVector<i8>, start: &Point2D<isize>, unique: bool) -> i64 {
    let pos = *start;
    let mut res: Vec<Point2D<isize>> = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    while let Some(pos) = queue.pop_front() {
        if map[pos] == 9 {
            res.push(pos);
            continue;
        }
        let neighbors = map.get_neighbors_pos(&pos, |p| map[*p] == map[pos] + 1);
        queue.extend(neighbors);
    }
    if unique {
    let mut unique_res: HashSet<Point2D<isize>> = HashSet::new();
    unique_res.extend(res.iter().collect::<HashSet<&Point2D<isize>>>());
    unique_res.len() as i64
    } else {
        res.len() as i64
    }
}
