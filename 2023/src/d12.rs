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

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        lines.iter().map(|line| count_permutations(line)).sum()
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
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
        result.iter().map(|line| count_permutations(&line)).sum()
    }

    fn get_part1_result(&self) -> Option<i64> {
        // Some(7286)
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn count_permutations(line: &String) -> i64 {
    let (chars_str, counts_str) = line.split_once(" ").unwrap();
    let expected_counts: Vec<usize> = counts_str.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    let chars: String = String::from(chars_str);

    let mut combinations = 0;
    let mut to_check = Vec::new();
    to_check.push((chars, expected_counts));

    while to_check.len() > 0 {
        let (to_check_chars, mut to_check_counts) = to_check.pop().unwrap();

        match to_check_chars.chars().next().unwrap() {
            '?' => {
                let has = to_check_chars.char_indices().map(|(i, c)| {if i == 0 {'#'} else {c}}).collect();
                let has_not = to_check_chars.char_indices().map(|(i, c)| {if i == 0 {'.'} else {c}}).collect();
                let count_copy = to_check_counts.to_vec();
                to_check.push((has, to_check_counts));
                to_check.push((has_not, count_copy));
            }
            '#' => {
                // we found a '#', but there are no more sections to match - failed match
                if to_check_counts.len() == 0 {
                    continue;
                }
                let mut last_hash_index = 0;
                let mut last_hash_or_qm_index = 0;
                for c in to_check_chars.chars() {
                    if c == '#' { last_hash_index += 1; }
                    if c == '#' || c == '?' { last_hash_or_qm_index += 1;} else {break;}
                }
                // we found a '#-or-?' group, but does it match the expected length?
                if last_hash_index <= to_check_counts[0] && to_check_counts[0] <= last_hash_or_qm_index {
                    if to_check_counts[0] == to_check_chars.len() {
                        // exact match until the end of string
                        combinations += 1;
                        continue;
                    } 
                    // can be a match if a '.' can follow
                    let next_one = to_check_chars.as_bytes()[to_check_counts[0] as usize] as char;
                    if next_one == '.' || next_one == '?' {
                        // all good, we have a match
                        let len = to_check_counts.remove(0);
                        let new_string = to_check_chars.chars().skip(len as usize + 1).collect::<String>();
                        if to_check_counts.len() == 0 && (new_string.len() == 0 || new_string.chars().all(|c| c == '.')) {
                            combinations += 1;
                        }
                        else if new_string.len() > 0 {
                            to_check.push((new_string, to_check_counts));
                        }
                    }
                }
            }
            '.' => {
                let new_string = to_check_chars.trim_start_matches('.');
                if new_string.len() == 0 {
                    if to_check_counts.len() == 0 {
                        combinations += 1;
                    }
                }
                else {
                    to_check.push((new_string.to_string(), to_check_counts));
                }
            }
            _ => { panic!() }
        }
    }
    combinations
}
