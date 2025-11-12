use crate::{
    common::{Direction, MapVector, Point2D},
    DayTask,
};

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
        Some(1337648)
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (start_pos, mut map, moves) = parse(lines, false);
        do_moves(&mut map, &start_pos, &moves, false);
        calc_res(&map, 'O')
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (start_pos, mut map, moves) = parse(lines, true);
        do_moves(&mut map, &start_pos, &moves, true);
        calc_res(&map, '[')
    }
}

fn calc_res(map: &MapVector<char>, shape: char) -> i64 {
    map.find(shape)
        .iter()
        .map(|p| (p.x as i64) + (p.y as i64) * 100)
        .sum()
}

fn do_moves(
    map: &mut MapVector<char>,
    start_pos: &Point2D<isize>,
    moves: &[char],
    big_boxes: bool,
) {
    let mut robot_pos = start_pos.clone();
    for &m in moves {
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
            if moved {
                robot_pos = next_pos;
            }
        }
    }
}

fn try_move_big_box(map: &mut MapVector<char>, pos: Point2D<isize>, dir: Direction) -> bool {
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

    if !can_move_big_box_check(map, left_pos, right_pos, dir) {
        return false;
    }

    do_move_big_box(map, left_pos, right_pos, dir);
    return true;
}

fn can_move_big_box_check(
    map: &MapVector<char>,
    left_pos: Point2D<isize>,
    right_pos: Point2D<isize>,
    dir: Direction,
) -> bool {
    let new_left = left_pos.move_dir(dir, 1);
    let new_right = right_pos.move_dir(dir, 1);

    if dir == Direction::North || dir == Direction::South {
        if map[new_left] == '#' || map[new_right] == '#' {
            return false;
        }
        if map[new_left] == '.' && map[new_right] == '.' {
            return true;
        }

        if map[new_left] == '[' && map[new_right] == ']' {
            return can_move_big_box_check(map, new_left, new_right, dir);
        }
        if map[new_left] == ']' && map[new_right] != '[' {
            let actual_left = new_left.move_dir(Direction::West, 1);
            return can_move_big_box_check(map, actual_left, new_left, dir);
        }
        if map[new_left] != ']' && map[new_right] == '[' {
            let actual_right = new_right.move_dir(Direction::East, 1);
            return can_move_big_box_check(map, new_right, actual_right, dir);
        }
        // last case: two different boxes blocking
        let left_can_move = if map[new_left] == ']' {
            let actual_left = new_left.move_dir(Direction::West, 1);
            can_move_big_box_check(map, actual_left, new_left, dir)
        } else {
            true
        };
        let right_can_move = if map[new_right] == '[' {
            let actual_right = new_right.move_dir(Direction::East, 1);
            can_move_big_box_check(map, new_right, actual_right, dir)
        } else {
            true
        };
        return left_can_move && right_can_move;
    } else {
        // moving left/right
        let next_to_side = if dir == Direction::West {
            new_left
        } else {
            new_right
        };
        if map[next_to_side] == '#' {
            return false;
        }
        if map[next_to_side] == '.' {
            return true;
        }
        return can_move_big_box_check(
            map,
            if map[next_to_side] == '[' {
                next_to_side
            } else {
                next_to_side.move_dir(Direction::West, 1)
            },
            if map[next_to_side] == ']' {
                next_to_side
            } else {
                next_to_side.move_dir(Direction::East, 1)
            },
            dir,
        );
    }
}

fn do_move_big_box(
    map: &mut MapVector<char>,
    left_pos: Point2D<isize>,
    right_pos: Point2D<isize>,
    dir: Direction,
) {
    let new_left = left_pos.move_dir(dir, 1);
    let new_right = right_pos.move_dir(dir, 1);

    if dir == Direction::North || dir == Direction::South {
        if map[new_left] == '[' && map[new_right] == ']' {
            do_move_big_box(map, new_left, new_right, dir);
        }
        if map[new_left] == ']' && map[new_right] != '[' {
            let actual_left = new_left.move_dir(Direction::West, 1);
            do_move_big_box(map, actual_left, new_left, dir);
        }
        if map[new_left] != ']' && map[new_right] == '[' {
            let actual_right = new_right.move_dir(Direction::East, 1);
            do_move_big_box(map, new_right, actual_right, dir);
        }
        // last case: two different boxes blocking
        if map[new_left] == ']' {
            let actual_left = new_left.move_dir(Direction::West, 1);
            do_move_big_box(map, actual_left, new_left, dir);
        }
        if map[new_right] == '[' {
            let actual_right = new_right.move_dir(Direction::East, 1);
            do_move_big_box(map, new_right, actual_right, dir);
        }
    } else {
        // moving left/right
        let next_to_side = if dir == Direction::West {
            new_left
        } else {
            new_right
        };
        if map[next_to_side] != '.' {
            do_move_big_box(
                map,
                if map[next_to_side] == '[' {
                    next_to_side
                } else {
                    next_to_side.move_dir(Direction::West, 1)
                },
                if map[next_to_side] == ']' {
                    next_to_side
                } else {
                    next_to_side.move_dir(Direction::East, 1)
                },
                dir,
            );
        }
    }

    // Move current box
    map[left_pos] = '.';
    map[right_pos] = '.';
    map[new_left] = '[';
    map[new_right] = ']';
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
