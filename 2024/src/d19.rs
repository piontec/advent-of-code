use crate::DayTask;

pub struct Task;

const TI: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        return 19;
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![6]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![16]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(340)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        let (towels, patterns) = parse_input(lines);

        let res = patterns
            .iter()
            .map(|pattern| check_pattern(pattern.clone(), towels.clone()).map_or(0, |_| 1))
            .sum();
        res
    }

    fn run_p2(&self, lines: &Vec<String>, _is_test: bool) -> i64 {
        let (towels, patterns) = parse_input(lines);

        let res = patterns
            .iter()
            .map(|pattern| count_pattern(pattern.clone(), towels.clone()) as i64)
            .sum();
        res
    }
}

fn parse_input(lines: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let towels: Vec<String> = lines[0].split(", ").map(|s| s.to_string()).collect();
    let patterns: Vec<String> = lines[2..].to_vec();
    (towels, patterns)
}

#[memoize::memoize]
fn check_pattern(pattern: String, towels: Vec<String>) -> Option<String> {
    // Base case: if pattern is empty, we've successfully matched everything
    if pattern.is_empty() {
        return Some(String::new());
    }

    // Try each towel as a potential prefix
    for towel in &towels {
        if pattern.starts_with(towel) {
            // Remove the matched prefix and recursively check the remainder
            let remaining = pattern[towel.len()..].to_string();
            if let Some(rest_result) = check_pattern(remaining, towels.clone()) {
                // If recursive call succeeded, prepend current towel and return
                if rest_result.is_empty() {
                    return Some(towel.clone());
                } else {
                    return Some(format!("{}, {}", towel, rest_result));
                }
            }
        }
    }

    // No towel matched as a prefix, or all recursive calls failed
    None
}

#[memoize::memoize]
fn count_pattern(pattern: String, towels: Vec<String>) -> usize {
    // Base case: if pattern is empty, we've successfully matched everything
    if pattern.is_empty() {
        return 1;
    }

    let mut res = 0;
    // Try each towel as a potential prefix
    for towel in &towels {
        if pattern.starts_with(towel) {
            // Remove the matched prefix and recursively check the remainder
            let remaining = pattern[towel.len()..].to_string();
            res += count_pattern(remaining, towels.clone());
        }
    }

    // No towel matched as a prefix, or all recursive calls failed
    res
}
