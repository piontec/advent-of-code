use crate::{
    common::{MapVector, Point2D},
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
        vec![]
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let map: MapVector<i8> = MapVector::new(lines, |c| (c.to_digit(10).unwrap()) as i8);
        let starts = map.find(0);
        starts.iter().map(|start| get_quality(&map, start)).sum()
    }

    fn run_p2(&self, _lines: &Vec<String>, _: bool) -> i64 {
        0
    }
}

fn get_quality(map: &MapVector<i8>, start: &Point2D<isize>) -> i64 {
    let pos = *start;
    let mut res: HashSet<Point2D<isize>> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    while let Some(pos) = queue.pop_front() {
        if map[pos] == 9i8 {
            res.insert(pos);
            continue;
        }
        let neighbors = map.get_neighbors_pos(&pos, |p| map[*p] == map[pos] + 1i8);
        queue.extend(neighbors);
    }
    res.len() as i64
}
