use crate::DayTask;

pub struct Task;

const TI: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        13
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        405
    }

    fn get_part2_test_result(&self) -> i64 {
        400
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        let maps = split_maps(lines);
        let res: usize = maps.iter().map(|map| check_both(map)).sum();
        res as i64
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        0
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(37975)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn check_both(map: &[String]) -> usize {
    let mut res = find_reflection(map);
    if res.is_some() {
        return res.unwrap();
    }
    let char_map = &map.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let transposed = transpose(char_map);
    let ts = transposed.iter()
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<String>>();
    res = find_reflection(&ts);
    if res.is_none() {
        panic!("No reflection found");
    }
    res.unwrap() * 100
}

fn transpose<T: Clone>(array2d: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::<Vec<T>>::new();
    for x in 0..array2d[0].len() {
        let mut row = Vec::<T>::new();
        for y in 0..array2d.len() {
            row.push(array2d[y][x].clone());
        }
        result.push(row);
    }
    result
}

fn find_reflection(map: &[String]) -> Option<usize> {
    let max_x = map[0].len();
    let max_y = map.len();

    for x in 0..max_x - 1{
        let mut offset: isize = 0;
        let mut symmetric = true;
        let mut left = x as isize - offset;
        let mut right = x as isize + 1 + offset;
        while left >= 0 && (right as usize) < max_x {
            symmetric = (0..max_y)
                .all(|y| map[y].chars().nth(left as usize) == map[y].chars().nth(right as usize));
            if !symmetric {
                break;
            }
            offset += 1;
            left = x as isize - offset;
            right = x as isize + 1 + offset;
        }
        if symmetric {
            return Some(x + 1);
        }
    }
    None
}

fn split_maps(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut maps: Vec<Vec<String>> = Vec::new();
    let mut map: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            maps.push(map);
            map = Vec::new();
        } else {
            map.push(line.to_string());
        }
    }
    maps.push(map);
    maps
}
