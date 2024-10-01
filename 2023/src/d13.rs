use crate::common::transpose;
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

#[derive(Clone,Debug)]
struct IgnoreInfo {
    index: usize,
    transposed: bool,
}

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
        let res: usize = maps
            .iter()
            .map(|map| check_both(map, None, false).unwrap())
            .map(|ii| if ii.transposed { ii.index * 100 } else { ii.index })
            .sum();
        res as i64
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        let maps = split_maps(lines);
        let res: usize = maps
            .iter()
            .map(|map| check_smudge(map).unwrap())
            .map(|ii| if ii.transposed { ii.index * 100 } else { ii.index })
            .sum();
        res as i64
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(37975)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(32497)
    }
}

fn check_smudge(map: &[String]) -> Option<IgnoreInfo> {
    let first = check_both(map, None, false);
    if first.is_none() {
        panic!("No solution found for original map");
    }
    let res = check_both(map, first, true);
    res
}

fn check_both(map: &[String], to_ignore: Option<IgnoreInfo>, allow_one_off: bool) -> Option<IgnoreInfo> {
    let ignore = if to_ignore.is_some() {
        let ti = to_ignore.clone().unwrap();
        if ti.transposed == false {
            Some(ti.index)
        } else {
            None
        }
    } else {
        None
    };
    let mut res = find_reflection(map, ignore, allow_one_off);
    if res.is_some() {
        return Some(IgnoreInfo {
            index: res.unwrap(),
            transposed: false,
        });
    }
    let char_map = &map
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let transposed = transpose(char_map);
    let ts = transposed
        .iter()
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<String>>();
    let ignore = if to_ignore.is_some() {
        let ti = to_ignore.clone().unwrap();
        if ti.transposed == true {
            Some(ti.index)
        } else {
            None
        }
    } else {
        None
    };
    res = find_reflection(&ts, ignore, allow_one_off);
    if res.is_none() {
        return None;
    }
    Some(IgnoreInfo {
        index: res.unwrap(),
        transposed: true,
    })
}

fn find_reflection(map: &[String], to_ignore: Option<usize>, allow_one_off: bool) -> Option<usize> {
    let max_x = map[0].len();
    let max_y = map.len();

    for x in 0..max_x - 1 {
        let mut offset: isize = 0;
        let mut diff = usize::MAX;
        let mut left = x as isize - offset;
        let mut right = x as isize + 1 + offset;
        let mut used_one_diff = false;
        while left >= 0 && (right as usize) < max_x {
            diff = (0..max_y)
                .map(|y| {
                    if map[y].chars().nth(left as usize) == map[y].chars().nth(right as usize) {
                        0
                    } else {
                        1
                    }
                })
                .sum();
            if diff > 0 && (used_one_diff || !allow_one_off) {
                break;
            }
            if diff == 1 && allow_one_off && !used_one_diff {
                used_one_diff = true;
                diff = 0;
            } else if diff > 0 {
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
