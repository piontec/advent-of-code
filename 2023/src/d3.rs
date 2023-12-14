use crate::DayTask;

pub struct Task;

impl DayTask<i32> for Task {
    fn day_no(&self) -> u8 {
        3
    }

    fn get_part1_test_input(&self) -> &'static str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    }

    fn get_part2_test_input(&self) -> &'static str {
        todo!()
    }

    fn get_part1_test_result(&self) -> i32 {
        4361
    }

    fn get_part2_test_result(&self) -> i32 {
        todo!()
    }

    fn run_p1(&self, lines: Vec<String>) -> i32 {
        let mut result = 0;
        for (li, line) in lines.iter().enumerate() {
            let mut digit_start_index: Option<usize> = Option::None;
            let mut digit_end_index = 0;
            for (i, c) in line.chars().enumerate() {
                if c.is_digit(10) && digit_start_index.is_none() {
                    digit_start_index = Option::Some(i);
                    continue;
                }
                if c.is_digit(10) && digit_start_index.is_some() {
                    continue;
                }
                if !c.is_digit(10) && digit_start_index.is_some() {
                    digit_end_index = i - 1;

                    let (num, has_symbol) = get_num(
                        li,
                        &lines,
                        digit_start_index.unwrap(),
                        digit_end_index,
                    );
                    if has_symbol {
                        result += num;
                    }
                    digit_start_index = Option::None;
                }
            }
            if digit_start_index.is_some() {
                digit_end_index = line.len() - 1;
                let (num, has_symbol) = get_num(
                    li,
                    &lines,
                    digit_start_index.unwrap(),
                    digit_end_index,
                );
                if has_symbol {
                    result += num;
                }
            }
        }
        result
    }

    fn run_p2(&self, lines: Vec<String>) -> i32 {
        todo!()
    }
}

fn get_num(
    line_index: usize,
    lines: &Vec<String>,
    digit_start_index: usize,
    digit_end_index: usize,
) -> (i32, bool) {
    let line = &lines[line_index];
    let mut lines_to_check_indexes: Vec<usize> = Vec::new();
    if line_index > 0 {
        lines_to_check_indexes.push(line_index - 1);
    }
    if line_index < lines.len() - 1 {
        lines_to_check_indexes.push(line_index + 1);
    }
    let min_i = if digit_start_index > 0 {
        digit_start_index - 1
    } else {
        digit_start_index
    };
    let max_i = if digit_end_index < lines[line_index].len() - 1 {
        digit_end_index + 1
    } else {
        digit_end_index
    };

    let mut has_symbol = false;
    'of: for line_index in lines_to_check_indexes {
        for char_index in min_i..=max_i {
            let cur = lines[line_index].chars().nth(char_index).unwrap();
            if cur == '.' || cur.is_digit(10) {
                continue;
            }
            has_symbol = true;
            break 'of;
        }
    }
    if !has_symbol {
        if min_i < digit_start_index && line.chars().nth(min_i).unwrap() != '.' {
            has_symbol = true;
        }
        if max_i > digit_end_index && line.chars().nth(max_i).unwrap() != '.' {
            has_symbol = true;
        }
    }
    let num = str::parse::<i32>(&line[digit_start_index..=digit_end_index]).unwrap();
    (num, has_symbol)
}
