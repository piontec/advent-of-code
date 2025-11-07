use crate::{
    common::{Direction, MapVector, Point2D},
    DayTask,
};
use std::collections::HashMap;

pub struct Task;

const TI: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const TI2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

const TI3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        15
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI, TI2]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI2, TI3]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![2028, 10092]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![9021, 618]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(1318523)
    }

    fn get_part2_result(&self) -> Option<i64> {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (start_pos, mut map, moves) = parse(lines, false);
        do_moves(&mut map, &start_pos, &moves, false);
        calc_res(&map, 'O')
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (start_pos, mut map, moves) = parse(lines, true);
        do_moves(&mut map, &start_pos, &moves, true);
        print!("{:?}", map);
        calc_res(&map, '[')
    }
}

fn calc_res(map: &MapVector<char>, shape: char) -> i64 {
    map.find(shape)
        .iter()
        .map(|p| (p.x as i64) + (p.y as i64) * 100)
        .sum()
}

fn print_map(map: &mut MapVector<char>, robot_pos: &Point2D<isize>) {
    map[robot_pos] = '@';
    println!("{:?}", map);
    map[robot_pos] = '.';
}

fn do_moves(
    map: &mut MapVector<char>,
    start_pos: &Point2D<isize>,
    moves: &[char],
    big_boxes: bool,
) {
    let mut robot_pos = start_pos.clone();
    for &m in moves {
        print_map(map, &robot_pos);
        print!("move: {}\n", m);

        let dir = match m {
            '^' => Direction::North,
            'v' => Direction::South,
            '<' => Direction::West,
            '>' => Direction::East,
            _ => panic!("Invalid move"),
        };
        let next_pos = robot_pos.move_dir(dir, 1);
        if map[next_pos] == '#' {
            continue;
        }
        if map[next_pos] == '.' {
            robot_pos = next_pos;
            continue;
        }
        // else, there is a box
        if big_boxes == false && try_move_box(map, next_pos, dir) {
            robot_pos = next_pos;
        }
        if big_boxes {
            let moved = try_move_big_box(map, next_pos, dir);
            if moved > 0 && moved <= 3 {
                robot_pos = next_pos;
            }
        }
    }
}

fn try_move_big_box(map: &mut MapVector<char>, pos: Point2D<isize>, dir: Direction) -> u8 {
    let left_pos = if map[pos] == '[' {
        pos
    } else {
        pos.move_dir(Direction::West, 1)
    };
    let right_pos = if map[pos] == ']' {
        pos
    } else {
        pos.move_dir(Direction::East, 1)
    };
    let new_left = left_pos.move_dir(dir, 1);
    let new_right = right_pos.move_dir(dir, 1);
    if dir == Direction::North || dir == Direction::South {
        if map[new_left] == '#' || map[new_right] == '#' {
            return 0;
        }
        if map[new_left] == '.' && map[new_right] == '.' {
            map[new_left] = map[left_pos];
            map[new_right] = map[right_pos];
            map[left_pos] = '.';
            map[right_pos] = '.';
            return 1;
        }

        if map[new_left] == '[' && map[new_right] == ']' {
            let so_far = try_move_big_box(map, new_left, dir);
            if so_far == 0 || so_far >= 3 {
                return 0;
            }
            map[new_left] = map[left_pos];
            map[new_right] = map[right_pos];
            map[left_pos] = '.';
            map[right_pos] = '.';
            return so_far + 1;
        }
        if map[new_left] == ']' && map[new_right] != '[' {
            let left_so_far = try_move_big_box(map, new_left, dir);
            if left_so_far == 0 || left_so_far >= 3 {
                return 0;
            }
            map[new_left] = map[left_pos];
            map[new_right] = map[right_pos];
            map[left_pos] = '.';
            map[right_pos] = '.';
            return left_so_far + 1;
        }
        if map[new_left] != ']' && map[new_right] == '[' {
            let right_so_far = try_move_big_box(map, new_right, dir);
            if right_so_far == 0 || right_so_far >= 3 {
                return 0;
            }
            map[new_left] = map[left_pos];
            map[new_right] = map[right_pos];
            map[left_pos] = '.';
            map[right_pos] = '.';
            return right_so_far + 1;
        }
        // last case: two different boxes blocking
        let left_so_far = try_move_big_box(map, new_left, dir);
        let right_so_far = try_move_big_box(map, new_right, dir);
        if left_so_far > 0 && right_so_far > 0 {
            if left_so_far + right_so_far >= 3 {
                return 0;
            }
            map[new_left] = map[left_pos];
            map[new_right] = map[right_pos];
            map[left_pos] = '.';
            map[right_pos] = '.';
            return left_so_far + right_so_far + 1;
        }
        return 0;
    } else {
        // moving left/right
        let next_to_side = if dir == Direction::West {
            new_left
        } else {
            new_right
        };
        if map[next_to_side] == '#' {
            return 0;
        }
        if map[next_to_side] == '.' {
            map[left_pos] = '.';
            map[right_pos] = '.';
            map[new_left] = '[';
            map[new_right] = ']';
            return 1;
        }
        let so_far = try_move_big_box(map, next_to_side, dir);
        if so_far == 0 || so_far >= 3 {
            return 0;
        }
        map[left_pos] = '.';
        map[right_pos] = '.';
        map[new_left] = '[';
        map[new_right] = ']';
        return so_far + 1;
    }
}

fn try_move_box(map: &mut MapVector<char>, pos: Point2D<isize>, dir: Direction) -> bool {
    let new_pos = pos.move_dir(dir, 1);
    if map[new_pos] == '#' {
        return false;
    }
    if map[new_pos] == '.' || (map[new_pos] == 'O' && try_move_box(map, new_pos, dir)) {
        map[new_pos] = map[pos];
        map[pos] = '.';
        return true;
    } else {
        // O encountered, but cannot move further
        return false;
    }
}

fn parse(lines: &Vec<String>, expand: bool) -> (Point2D<isize>, MapVector<char>, Vec<char>) {
    let empty_ind = lines.iter().position(|s| s.trim().is_empty()).unwrap();
    let mut map_lines = Vec::from(&lines[..empty_ind]);
    if expand {
        map_lines = map_lines
            .iter()
            .map(|line| {
                line.replace('#', "##")
                    .replace('.', "..")
                    .replace('O', "[]")
                    .replace('@', "@.")
            })
            .collect();
    }
    let mut map = MapVector::new(&map_lines, |c| c);
    let moves: Vec<char> = lines[empty_ind + 1..].join("").chars().collect();
    let start_pos = map.find('@').first().unwrap().clone();
    map[start_pos] = '.';
    (start_pos, map, moves)
}
