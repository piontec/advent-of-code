use crate::{common::Point2D, DayTask};
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub struct Task;

const TI: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    pos: Point2D<i32>,
    dx: i8,
    dy: i8,
    straight_line_steps: u8,
}

impl State {
    fn new(pos: Point2D<i32>, dx: i8, dy: i8, straight_line_steps: u8) -> Self {
        Self {
            pos,
            dx,
            dy,
            straight_line_steps,
        }
    }

    fn starting(x: i32, y: i32, dx: i8, dy: i8) -> Self {
        Self {
            pos: Point2D::new(x, y),
            dx,
            dy,
            straight_line_steps: 1,
        }
    }
}

struct StateCost {
    state: State,
    cost: usize,
}

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        17
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        102
    }

    fn get_part2_test_result(&self) -> i64 {
        94
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        let map = parse_map(lines);

        let destination = Point2D::new(map[0].len() as i32 - 1, map.len() as i32 - 1);
        let mut visited_states = HashSet::new();
        let mut to_visit_states = HashMap::from([
            (map[0][1] as usize, vec![State::starting(1, 0, 1, 0)]),
            (map[1][0] as usize, vec![State::starting(0, 1, 0, 1)]),
        ]);

        // upper limit on cost
        assert!(map.len() == map[0].len());
        while let Some(current_state) = pop_next_to_visit_state(&mut to_visit_states) {
            // check if we're done
            if current_state.state.pos == destination {
                return current_state.cost as i64;
            }

            if !visited_states.insert(current_state.state.clone()) {
                continue;
            }

            // add next states to visit
            let next_states = get_next_states(&map, &current_state);
            for next_state in next_states {
                if !visited_states.contains(&next_state.state) {
                    add_to_visit_state(&mut to_visit_states, next_state);
                }
            }
        }

        panic!("No solution found")
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        let map = parse_map(lines);

        let destination = Point2D::new(map[0].len() as i32 - 1, map.len() as i32 - 1);
        let mut visited_states = HashSet::new();
        let mut to_visit_states = HashMap::from([
            (12, vec![State::new(Point2D { x: 4, y: 0 }, 1, 0, 4)]),
            (13, vec![State::new(Point2D { x: 0, y: 4 }, 0, 1, 4)]),
        ]);

        // upper limit on cost
        assert!(map.len() == map[0].len());
        while let Some(current_state) = pop_next_to_visit_state(&mut to_visit_states) {
            // check if we're done
            if current_state.state.pos == destination {
                return current_state.cost as i64;
            }

            if !visited_states.insert(current_state.state.clone()) {
                continue;
            }

            // add next states to visit
            let next_states = get_next_states_ultra(&map, &current_state);
            for next_state in next_states {
                if !visited_states.contains(&next_state.state) {
                    add_to_visit_state(&mut to_visit_states, next_state);
                }
            }
        }

        panic!("No solution found")
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(1244)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn parse_map(lines: &Vec<String>) -> Vec<Vec<u8>> {
    let map = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();
    map
}

fn add_to_visit_state(to_visit_states: &mut HashMap<usize, Vec<State>>, state: StateCost) {
    if to_visit_states.contains_key(&state.cost) {
        to_visit_states
            .get_mut(&state.cost)
            .unwrap()
            .push(state.state);
    } else {
        to_visit_states.insert(state.cost, vec![state.state]);
    }
}

fn get_next_states_ultra(map: &Vec<Vec<u8>>, current: &StateCost) -> Vec<StateCost> {
    if current.state.straight_line_steps < 4 {
        panic!("I should never have steps < 4")
    }
    let current_state = current.state;
    let mut res = Vec::<StateCost>::new();

    let ccw = rotate(current_state.dx, current_state.dy, false);
    let ccw_pos = Point2D::new(
        current_state.pos.x + ccw.0 as i32 * 4,
        current_state.pos.y + ccw.1 as i32 * 4,
    );
    if ccw_pos.in_range(map[0].len() as i32, map.len() as i32) {
        let ccw_cost = current.cost 
            + (1..=4)
            .map(|i| {
                map[(current_state.pos.y + ccw.1 as i32 * i) as usize]
                    [(current_state.pos.x + ccw.0 as i32 * i) as usize] as usize
            })
            .sum::<usize>();
            res.push(StateCost {
                state: State::new(ccw_pos, ccw.0, ccw.1, 4),
                cost: ccw_cost,
            })
    }

    let cw = rotate(current_state.dx, current_state.dy, true);
    let cw_pos = Point2D::new(
        current_state.pos.x + cw.0 as i32 * 4,
        current_state.pos.y + cw.1 as i32 * 4,
    );
    if cw_pos.in_range(map[0].len() as i32, map.len() as i32) {
        let cw_cost = current.cost 
            + (1..=4)
            .map(|i| {
                map[(current_state.pos.y + cw.1 as i32 * i) as usize]
                    [(current_state.pos.x + cw.0 as i32 * i) as usize] as usize
            })
            .sum::<usize>();
            res.push(StateCost {
                state: State::new(cw_pos, cw.0, cw.1, 4),
                cost: cw_cost,
            })
    }

    if current_state.straight_line_steps < 10 {
        let pos = Point2D::new(
            current_state.pos.x + current_state.dx as i32,
            current_state.pos.y + current_state.dy as i32,
        );
        if pos.in_range(map[0].len() as i32, map.len() as i32) {
            res.push(StateCost {
                state: State::new(
                    pos,
                    current_state.dx,
                    current_state.dy,
                    current_state.straight_line_steps + 1,),
                cost: current.cost
                    + map[(current_state.pos.x + current_state.dx as i32) as usize]
                        [(current_state.pos.y + current_state.dy as i32) as usize] as usize,
                }
            );
        }
    }

    res
}

fn get_next_states(map: &Vec<Vec<u8>>, current: &StateCost) -> Vec<StateCost> {
    let current_state = current.state;
    let ccw = rotate(current_state.dx, current_state.dy, false);
    let ccw_pos = Point2D::new(
        current_state.pos.x + ccw.0 as i32,
        current_state.pos.y + ccw.1 as i32,
    );
    let cw = rotate(current_state.dx, current_state.dy, true);
    let cw_pos = Point2D::new(
        current_state.pos.x + cw.0 as i32,
        current_state.pos.y + cw.1 as i32,
    );

    let mut to_add = vec![(ccw_pos, ccw, 1), (cw_pos, cw, 1)];
    if current_state.straight_line_steps < 3 {
        to_add.push((
            Point2D::new(
                current_state.pos.x + current_state.dx as i32,
                current_state.pos.y + current_state.dy as i32,
            ),
            (current_state.dx, current_state.dy),
            current_state.straight_line_steps + 1,
        ));
    }

    let mut res = Vec::<StateCost>::new();
    for (new_pos, new_dpl, new_straight) in to_add {
        if new_pos.in_range(map[0].len() as i32, map.len() as i32) {
            res.push(StateCost {
                state: State::new(new_pos, new_dpl.0, new_dpl.1, new_straight),
                cost: current.cost + map[new_pos.y as usize][new_pos.x as usize] as usize,
            })
        }
    }
    res
}

fn rotate(x: i8, y: i8, clockwise: bool) -> (i8, i8) {
    match clockwise {
        true => (-y, x),
        false => (y, -x),
    }
}

fn pop_next_to_visit_state(to_visit_states: &mut HashMap<usize, Vec<State>>) -> Option<StateCost> {
    if to_visit_states.is_empty() {
        return None;
    }
    let min_key = to_visit_states.keys().min().unwrap().clone();
    let res = to_visit_states.get_mut(&min_key).unwrap().pop();
    if to_visit_states.get_mut(&min_key).unwrap().is_empty() {
        to_visit_states.remove(&min_key);
    }
    Some(StateCost {
        state: res.unwrap(),
        cost: min_key,
    })
}

pub trait StateStore {
    fn pop_next_to_visit_state(
        to_visit_states: &mut HashMap<usize, Vec<State>>,
    ) -> Option<StateCost>;
    fn get_next_states(map: &Vec<Vec<u8>>, current: &StateCost) -> Vec<StateCost>;
}

struct AStar {}

impl AStar {
    fn new() {}
}
