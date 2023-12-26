use std::collections::HashMap;

use crate::DayTask;

pub struct Task;

const TI: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

impl DayTask<usize> for Task {
    fn day_no(&self) -> u8 {
        3
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> usize {
        4361
    }

    fn get_part2_test_result(&self) -> usize {
        467835
    }

    fn run_p1(&self, lines: &Vec<String>) -> usize {
        // answer: 537732
        let mut result = 0;
        for numpos in get_numpos(&lines) {
            if numpos.symbol.is_none() {
                continue;
            }
            result += numpos.num;
        }
        result
    }

    fn run_p2(&self, lines: &Vec<String>) -> usize {
        // 84883664
        let mut result = 0;
        let mut gears: HashMap<Position, Vec<SymbolNum>> = HashMap::new();
        for numpos in get_numpos(&lines) {
            if numpos.symbol.is_some_and(|s| s == '*') {
                if !gears.contains_key(&numpos.position) {
                    gears.insert(numpos.position, Vec::new());
                }
                gears.get_mut(&numpos.position).unwrap().push(numpos);
            }
        }
        for (_, v) in gears.iter() {
            if v.len() != 2 {
                continue;
            }
            result += v[0].num * v[1].num;
        }
        result
    }

    fn get_part1_result(&self) -> Option<usize> {
        None
    }

    fn get_part2_result(&self) -> Option<usize> {
        None
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    line_index: usize,
    char_index: usize,
}

struct SymbolNum {
    position: Position,
    symbol: Option<char>,
    num: usize,
}

fn get_numpos(lines: &Vec<String>) -> Vec<SymbolNum> {
    let mut result: Vec<SymbolNum> = Vec::new();

    for (li, line) in lines.iter().enumerate() {
        let mut digit_start_index: Option<usize> = Option::None;
        let mut digit_end_index;
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

                let s = get_num(li, &lines, digit_start_index.unwrap(), digit_end_index);
                result.push(s);
                digit_start_index = Option::None;
            }
        }
        if digit_start_index.is_some() {
            digit_end_index = line.len() - 1;
            let s = get_num(li, &lines, digit_start_index.unwrap(), digit_end_index);
            result.push(s);
        }
    }
    result
}

fn get_num(
    line_index: usize,
    lines: &Vec<String>,
    digit_start_index: usize,
    digit_end_index: usize,
) -> SymbolNum {
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
    let mut symbol_pos = Position {
        line_index: 0,
        char_index: 0,
    };
    let mut symbol: Option<char> = Option::None;

    'of: for line_index in lines_to_check_indexes {
        for char_index in min_i..=max_i {
            let cur = lines[line_index].chars().nth(char_index).unwrap();
            if cur == '.' || cur.is_digit(10) {
                continue;
            }
            has_symbol = true;
            symbol_pos.char_index = char_index;
            symbol_pos.line_index = line_index;
            symbol = Option::Some(cur);
            break 'of;
        }
    }
    if !has_symbol {
        if min_i < digit_start_index && line.chars().nth(min_i).unwrap() != '.' {
            symbol_pos.char_index = min_i;
            symbol_pos.line_index = line_index;
            symbol = Option::Some(line.chars().nth(min_i).unwrap());
        }
        if max_i > digit_end_index && line.chars().nth(max_i).unwrap() != '.' {
            symbol_pos.char_index = max_i;
            symbol_pos.line_index = line_index;
            symbol = Option::Some(line.chars().nth(max_i).unwrap());
        }
    }
    let num = str::parse::<usize>(&line[digit_start_index..=digit_end_index]).unwrap();
    SymbolNum {
        position: symbol_pos,
        symbol,
        num,
    }
}
