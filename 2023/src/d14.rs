use std::collections::HashMap;

use num::range;

use crate::DayTask;

pub struct Task;

const TI: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        14
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        136
    }

    fn get_part2_test_result(&self) -> i64 {
        64
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut map = lines
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        roll_north(&mut map);
        count_weight(&map)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut rounds = 1000000000;
        // let mut rounds = 20;
        let mut cache: HashMap<String, usize> = HashMap::new();
        let mut map = lines
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let cycle_start: usize;
        let cycle_end: usize;
        loop {
            roll_all(&mut map);
            rounds -= 1;
            let state = format_map(&map);
            if cache.contains_key(&state) {
                cycle_start = cache[&state];
                cycle_end = rounds;
                break;
            }
            cache.insert(state, rounds);
        }
        let cycle_len = cycle_start - cycle_end;
        rounds = rounds % cycle_len;
        for _ in 0..rounds {
            roll_all(&mut map);
        }
        count_weight(&map)
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(105003)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(93742)
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    println!("{}", format_map(map));
}

fn format_map(map: &Vec<Vec<char>>) -> String {
    let mut map_string = String::with_capacity(map.len() * (map[0].len() + 1));
    for row in map {
        map_string.push_str(&row.iter().collect::<String>());
        map_string.push('\n');
    }
    map_string
}

fn roll_all(map: &mut Vec<Vec<char>>) {
    roll_north(map);
    roll_west(map);
    roll_south(map);
    roll_east(map);
}

fn roll_north(map: &mut Vec<Vec<char>>) {
    let rng = range(0, map.len());
    roll_them_vertical(map, rng, 0, |y| y + 1);
}

fn roll_south(map: &mut Vec<Vec<char>>) {
    let rng = range(0, map.len()).rev();
    roll_them_vertical(map, rng, map.len() - 1, |y| if y == 0 {0} else {y - 1});
}

fn roll_west(map: &mut Vec<Vec<char>>) {
    let rng = range(0, map[0].len());
    roll_them_horizontal(map, rng, 0, |x| x + 1);
}

fn roll_east(map: &mut Vec<Vec<char>>) {
    let rng = range(0, map[0].len()).rev();
    roll_them_horizontal(map, rng, map[0].len() - 1, |x| if x == 0 {0} else {x - 1});
}

fn roll_them_vertical<T: Iterator<Item = usize>>(
    map: &mut Vec<Vec<char>>,
    range: T,
    start: usize,
    update_fun: impl Fn(usize) -> usize,
) where
    T: Clone,
{
    for x in 0..map[0].len() {
        let mut first_free_y = start;
        for y in range.clone() {
            if map[y][x] == '#' {
                // if we are at the top going from the bottom, we need to stop to avoid first_free_y to be -1
                // if start > 0 && y == 0 {
                    // break;
                // }
                first_free_y = update_fun(y);
                continue;
            }
            if map[y][x] == 'O' {
                if y == first_free_y {
                    first_free_y = update_fun(first_free_y);
                    continue;
                } else {
                    map[y][x] = '.';
                    map[first_free_y][x] = 'O';
                    first_free_y = update_fun(first_free_y);
                }
            }
        }
    }
}

fn roll_them_horizontal<T: Iterator<Item = usize>>(
    map: &mut Vec<Vec<char>>,
    range: T,
    start: usize,
    update_fun: impl Fn(usize) -> usize,
) where
    T: Clone,
{
    for y in 0..map.len() {
        let mut first_free_x = start;
        for x in range.clone() {
            if map[y][x] == '#' {
                // if we are at the top going from the bottom, we need to stop to avoid first_free_x to be -1
                if start > 0 && x == 0 {
                    break;
                }
                first_free_x = update_fun(x);
                continue;
            }
            if map[y][x] == 'O' {
                if x == first_free_x {
                    first_free_x = update_fun(first_free_x);
                    continue;
                } else {
                    map[y][x] = '.';
                    map[y][first_free_x] = 'O';
                    first_free_x = update_fun(first_free_x);
                }
            }
        }
    }
}

fn count_weight(map: &Vec<Vec<char>>) -> i64 {
    let res = map
        .iter()
        .enumerate()
        .map(|(y, val)| {
            let weight = map.len() - y;
            val.iter().filter(|c| **c == 'O').count() as i64 * weight as i64
        })
        .sum();
    res
}
