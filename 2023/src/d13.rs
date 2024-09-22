use crate::DayTask;
use crate::common::transpose;

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
        let res: usize = maps.iter().map(|map| check_both(map, None).unwrap()).sum();
        res as i64
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        let maps = split_maps(lines);
        let res: usize = maps.iter().map(|map| check_smudge(map)).sum();
        res as i64
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(37975)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn check_smudge(map: &[String]) -> usize {
    let first = check_both(map, None);
    if first.is_none() {
        panic!("No solution found for original map");
    }
    let res = check_both(map, first);
    return res.unwrap();
}

fn check_both(map: &[String], to_ignore: Option<usize>) -> Option<usize> {
    let mut res = find_reflection(map, to_ignore);
    if res.is_some() {
        return res;
    }
    let char_map = &map.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let transposed = transpose(char_map);
    let ts = transposed.iter()
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<String>>();
    res = find_reflection(&ts, to_ignore);
    if res.is_none() {
        return res;
    }
    Some(res.unwrap() * 100)
}

fn find_reflection(map: &[String], to_ignore: Option<usize>) -> Option<usize> {
    let max_x = map[0].len();
    let max_y = map.len();

    for x in 0..max_x - 1{
        let mut offset: isize = 0;
        let mut diff = usize::MAX;
        let mut left = x as isize - offset;
        let mut right = x as isize + 1 + offset;
        let mut used_one_diff = false;
        while left >= 0 && (right as usize) < max_x {
            diff = (0..max_y)
                .map(|y| if map[y].chars().nth(left as usize) == map[y].chars().nth(right as usize) {0} else {1})
                .sum();
            if diff > 0 && (used_one_diff || to_ignore.is_none()) {
                break;
            }
            if diff == 1 && to_ignore.is_some() && !used_one_diff {
                used_one_diff = true;
                diff = 0;
            }
            else if diff > 0 {
                break;
            }
            offset += 1;
            left = x as isize - offset;
            right = x as isize + 1 + offset;
        }
        if diff == 0 && !(to_ignore.is_some() && to_ignore.unwrap() == x + 1) {
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
