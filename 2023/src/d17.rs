use crate::{
    common::Point2D,
    DayTask,
};
use std::collections::HashMap;

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
    fn new(
        pos: Point2D<i32>,
        dx: i8,
        dy: i8,
        straight_line_steps: u8,
    ) -> Self {
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
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        let map = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
                    .collect()
            })
            .collect::<Vec<Vec<u8>>>();

        let destination = Point2D::new(map[0].len() as i32 - 1, map.len() as i32 - 1);
        let mut visited_states = Vec::<State>::new();
        let mut to_visit_states = HashMap::from([
            (0 as usize, vec![State::starting(0, 0, 1, 0)]),
            (0 as usize, vec![State::starting(0, 0, 0, 1)]),
        ]);

        // upper limit on cost
        assert!(map.len() == map[0].len());
        let mut best_solution = 0;
        for i in 1..map.len() {
            best_solution += map[i-1][i] as usize;
            best_solution += map[i][i] as usize;
        }
        while let Some(current_state) = pop_next_to_visit_state(&mut to_visit_states) {
            // check if we're done
            if current_state.state.pos == destination {
                if current_state.cost < best_solution {
                    best_solution = current_state.cost;
                }
                continue;
            }

            // check if we've visited this state before or if it's too expensive
            if visited_states.contains(&current_state.state) || current_state.cost >= best_solution {
                continue;
            }
            visited_states.push(current_state.state.clone());

            // add next states to visit
            let next_states = get_next_states(&map, &current_state);
            for next_state in next_states {
                if next_state.cost < best_solution {
                    add_to_visit_state(&mut to_visit_states, next_state);
                }
            }
            prune_to_visit_states(&mut to_visit_states, best_solution);
        }

        best_solution as i64
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn add_to_visit_state(to_visit_states: &mut HashMap<usize, Vec<State>>, state: StateCost) {
    if to_visit_states.contains_key(&state.cost) {
        to_visit_states.get_mut(&state.cost).unwrap().push(state.state);
    } else {
        to_visit_states.insert(state.cost, vec![state.state]);
    }
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

    let mut to_add = vec![
        (ccw_pos, ccw, 1),
        (cw_pos, cw, 1),
    ];
    if current_state.straight_line_steps < 3 {
        to_add.push((Point2D::new(
                current_state.pos.x + current_state.dx as i32,
                current_state.pos.y + current_state.dy as i32,
            ), 
            (current_state.dx, current_state.dy), 
            current_state.straight_line_steps + 1
        ));
    }

    let mut res = Vec::<StateCost>::new();
    for (new_pos, new_dpl, new_straight) in to_add
    {
        if new_pos.in_range(map[0].len() as i32, map.len() as i32) {
            res.push(
                StateCost { 
                    state: State::new(
                        new_pos,
                        new_dpl.0,
                        new_dpl.1,
                        new_straight,),
                    cost: current.cost + map[new_pos.y as usize][new_pos.x as usize] as usize,
                }
            )
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

fn prune_to_visit_states(to_visit_states: &mut HashMap<usize, Vec<State>>, best_solution: usize) {
    to_visit_states.retain(|k, _| *k < best_solution);
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
