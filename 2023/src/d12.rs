use memoize::memoize;

use crate::DayTask;

pub struct Task;

const TI: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        12
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    // 540 ms
    fn get_part1_test_result(&self) -> i64 {
        21
    }

    // 50300 ms
    fn get_part2_test_result(&self) -> i64 {
        525152
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        lines
            .iter()
            .map(|line| {
                let (chars_str, counts_str) = line.split_once(" ").unwrap();
                let expected_counts: Vec<u8> = counts_str
                    .split(",")
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                let chars: String = String::from(chars_str);
                count_recursive(chars, expected_counts)
            })
            .sum::<usize>() as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let result: Vec<String> = lines
            .iter()
            .map(|l| {
                let (p1, p2) = l.split_once(' ').unwrap();
                let p1c = p1.to_string() + "?";
                let p2c = p2.to_string() + ",";
                let p1cr = p1c.repeat(5);
                let res =
                    (p1cr[..p1cr.len() - 1]).to_owned() + " " + p2c.repeat(5).trim_end_matches(",");
                res
            })
            .collect();
        result
            .iter()
            .map(|line| {
                let (chars_str, counts_str) = line.split_once(" ").unwrap();
                let expected_counts: Vec<u8> = counts_str
                    .split(",")
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                let chars: String = String::from(chars_str);
                count_recursive(chars, expected_counts)
            })
            .sum::<usize>() as i64
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(7286)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

#[memoize]
fn count_recursive(line: String, groups: Vec<u8>) -> usize {
    // case: nothing left in "line", so it's OK only if "groups" is empty
    if line.len() == 0 {
        return if groups.len() == 0 { 1 } else { 0 };
    }
    // case: nothing left in "groups", so it's OK only if "line" has no "#"
    if groups.len() == 0 {
        return if line.contains("#") { 0 } else { 1 };
    }
    // check first character
    let c = line.chars().next().unwrap();
    let rest = line[1..].to_string();
    match c {
        '.' => count_recursive(rest.trim_start_matches(".").to_string(), groups),
        '?' => {
            count_recursive(".".to_string() + &rest, groups.clone())
                + count_recursive("#".to_string() + &rest, groups)
        }
        '#' => {
            let expected = groups[0];
            if line.len() >= expected as usize
                && line.chars().take(expected as usize).all(|c| c != '.')
                && (line.len() == expected as usize
                    || line.chars().nth(expected as usize).unwrap() != '#')
            {
                // if the whole line ends with a block of "#"s and that's the last block we expect, we have a match
                if line.len() == expected as usize {
                    return if groups.len() == 1 { 1 } else { 0 };
                }
                // we add "+ 1" to skip the next char after a block of "#"s, as it might be the '?'
                // we need to interpret as '.'
                let new_groups = groups[1..].to_vec();
                count_recursive(line[expected as usize + 1..].to_string(), new_groups)
            } else {
                0
            }
        }
        _ => panic!("This shall not happen!"),
    }
}
