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

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        15
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI, TI2]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI2]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![2028, 10092]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![9021]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(1318523)
    }

    fn get_part2_result(&self) -> Option<i64> {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (start_pos, mut map, moves) = parse(lines);
        do_moves(&mut map, &start_pos, &moves, false);
        calc_res(&map, 'O')
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (start_pos, orig_map, moves) = parse(lines);
        let mut map = expand_map(&orig_map);
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
        if (big_boxes == false && try_move(map, robot_pos, dir))
            || (big_boxes && try_move_2(map, robot_pos, dir))
        {
            robot_pos = robot_pos.move_dir(dir, 1);
        }
    }
}

fn expand_map(orig_map: &MapVector<char>) -> MapVector<char> {
    todo!()
}

fn try_move_2(map: &mut MapVector<char>, pos: Point2D<isize>, dir: Direction) -> bool {
    todo!()
}

fn try_move(map: &mut MapVector<char>, pos: Point2D<isize>, dir: Direction) -> bool {
    let new_pos = pos.move_dir(dir, 1);
    if map[new_pos] == '#' {
        return false;
    }
    if map[new_pos] == '.' || (map[new_pos] == 'O' && try_move(map, new_pos, dir)) {
        map[new_pos] = map[pos];
        map[pos] = '.';
        return true;
    } else {
        // O encountered, but cannot move further
        return false;
    }
}

fn parse(lines: &Vec<String>) -> (Point2D<isize>, MapVector<char>, Vec<char>) {
    let empty_ind = lines.iter().position(|s| s.trim().is_empty()).unwrap();
    let map_lines = Vec::from(&lines[..empty_ind]);
    let mut map = MapVector::new(&map_lines, |c| c);
    let moves: Vec<char> = lines[empty_ind + 1..].join("").chars().collect();
    let start_pos = map.find('@').first().unwrap().clone();
    map[start_pos] = '.';
    (start_pos, map, moves)
}
