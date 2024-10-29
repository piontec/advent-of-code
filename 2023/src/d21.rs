use crate::DayTask;
use std::{collections::{HashMap, HashSet}, path::Iter};

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
        let map = Map::parse_map(lines);
        let start = map.find('S')[0];
        let steps = if is_test { 6 } else {64};
        let positions = do_steps(&map, start.clone(), steps);
        positions.len() as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn do_steps(map: &Map<i32, char>, start: (i32, i32), steps: i32) -> Vec<(i32, i32)> {
    let mut current_pos = vec![start];
    for _ in 0..steps {
        let mut new_pos = HashSet::new();
        for pos in current_pos {
            let (y, x) = pos;
            for next in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].iter() {
                if map.map.contains_key(&next) && map.map[&next] != '#' {
                    new_pos.insert(*next);
                }
            }
        }
        current_pos = new_pos.into_iter().collect();
    }
    current_pos
}

pub struct Map<K, V> {
    pub map: HashMap<(K, K), V>,
}

// TODO: make a library for this
impl Map<i32, char> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn parse_map(lines: &Vec<String>) -> Self{
        let mut map = Self::new();
        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                map.map.insert((y as i32, x as i32), c);
            });
        });
        map
    }

    fn find(&self, c: char) -> Vec<&(i32, i32)> {
        self.map
            .iter()
            .filter(|(_, &v)| v == c)
            .map(|(k, _)| k)
            .collect::<Vec<&(i32, i32)>>()
    }
}