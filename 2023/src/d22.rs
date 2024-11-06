use crate::common::{Cube, Point3D};
use crate::DayTask;
use crossbeam_channel::unbounded;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Task;

const TI: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        22
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        5
    }

    fn get_part2_test_result(&self) -> i64 {
        7
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut bricks = parse(lines);
        move_down_all(&mut bricks);

        // check what can be disintegrated
        // we can disintegrate a brick if for every brick that is on top of it, there's at least one more different
        // brick supporting it below
        let bricks_by_z: HashMap<u16, Vec<i64>> =
            bricks
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (id, b)| {
                    for z in b.low_corner.z..=b.high_corner.z {
                        if acc.get(&(z as u16)).is_none() {
                            acc.insert(z as u16, Vec::new());
                        }
                        acc.get_mut(&(z as u16)).unwrap().push(id as i64);
                    }
                    acc
                });
        let mut disintegrate_count = 0;
        for bid in 0..bricks.len() {
            disintegrate_count +=
                get_bricks_on_top(bid, &bricks, &bricks_by_z)
                    .iter()
                    .all(|on_top_id| {
                        let mut supporting_bricks =
                            get_bricks_supporting(*on_top_id, &bricks, &bricks_by_z);
                        supporting_bricks.remove(&(bid as i64));
                        supporting_bricks.len() > 0
                    }) as i64;
        }

        disintegrate_count
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let mut bricks = parse(lines);
        move_down_all(&mut bricks);

        let mut total_moved = 0;
        let (in_tx, in_rx) = unbounded();
        let (out_tx, out_rx) = unbounded();
        let thread_count = 24;
        for _ in 0..thread_count {
            let my_in_rx = in_rx.clone();
            let my_out_tx = out_tx.clone();
            std::thread::spawn(move || {
                let mut moved = 0;
                while let Ok(mut new_bricks) = my_in_rx.recv() {
                    moved += move_down_all(&mut new_bricks).len() as i64;
                }
                my_out_tx.send(moved).unwrap();
            });
        }
        for bid in 0..bricks.len() {
            let new_bricks: Vec<Cube<i64>> = (&bricks)
                .iter()
                .enumerate()
                .filter_map(|(id, b)| {
                    if id == bid {
                        return None;
                    }
                    Some(b.clone())
                })
                .collect();
            in_tx.send(new_bricks).unwrap();
        }
        drop(in_tx);
        for _ in 0..thread_count {
            total_moved += out_rx.recv().unwrap();
        }
        total_moved
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(503)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(98431)
    }
}

fn move_down_all(bricks: &mut Vec<Cube<i64>>) -> Vec<i64> {
    // move bricks down starting from the bottom
    // floor is at 0, so bricks with z=1 are already on the floor
    let mut moved_bricks_ids: Vec<i64> = Vec::new();
    for brick_id in 0..bricks.len() {
        let brick = bricks.get(brick_id).unwrap();
        // does it overlap with any cube of any other brick below its original z?
        let mut crosses = false;
        let mut moved_down = brick.clone();
        let mut pre_moved_down = brick.clone();
        while !crosses && moved_down.low_corner.z > 0 {
            pre_moved_down = moved_down.clone();
            moved_down.move_down();
            // TODO: check this assumption that we can cross only with a brick with lower ID
            crosses = bricks
                .iter()
                .enumerate()
                .any(|(id, b)| id < brick_id as usize && b.crosses(&moved_down));
        }
        if bricks[brick_id] != pre_moved_down {
            moved_bricks_ids.push(brick_id as i64);
        }
        bricks[brick_id] = pre_moved_down;
    }
    moved_bricks_ids
}

fn parse(lines: &Vec<String>) -> Vec<Cube<i64>> {
    let bricks: Vec<Cube<i64>> = lines
        .iter()
        .map(|l| {
            let mut parts = l.split('~');
            let p1 = parts.next().unwrap();
            let p2 = parts.next().unwrap();
            let mut p1 = p1.split(',').map(|s| s.parse::<i64>().unwrap());
            let mut p2 = p2.split(',').map(|s| s.parse::<i64>().unwrap());
            Cube::new(
                Point3D::new(p1.next().unwrap(), p1.next().unwrap(), p1.next().unwrap()),
                Point3D::new(p2.next().unwrap(), p2.next().unwrap(), p2.next().unwrap()),
            )
        })
        .sorted_by_key(|c| c.low_corner.z)
        .collect();
    bricks
}

fn get_bricks_supporting(
    bid: i64,
    bricks: &[Cube<i64>],
    bricks_by_z: &HashMap<u16, Vec<i64>>,
) -> HashSet<i64> {
    get_bricks_in_z_dist(bid as usize, bricks, bricks_by_z, -1)
}

fn get_bricks_on_top(
    bid: usize,
    bricks: &[Cube<i64>],
    bricks_by_z: &HashMap<u16, Vec<i64>>,
) -> HashSet<i64> {
    get_bricks_in_z_dist(bid as usize, bricks, bricks_by_z, 1)
}

fn get_bricks_in_z_dist(
    bid: usize,
    bricks: &[Cube<i64>],
    bricks_by_z: &HashMap<u16, Vec<i64>>,
    z_dist: i64,
) -> HashSet<i64> {
    let mut res: HashSet<i64> = HashSet::new();
    let border_z = match z_dist {
        1 => bricks[bid].high_corner.z,
        -1 => bricks[bid].low_corner.z,
        _ => panic!("Invalid z_dist"),
    };
    let neighbor_brick_ids = bricks_by_z.get(&((border_z + z_dist) as u16));
    if neighbor_brick_ids.is_none() {
        return res;
    }
    let mut moved_brick = bricks[bid].clone();
    match z_dist {
        1 => moved_brick.move_up(),
        -1 => moved_brick.move_down(),
        _ => panic!("Invalid z_dist"),
    };

    for neighbor_brick_id in neighbor_brick_ids.unwrap() {
        let neighbor_brick = &bricks[*neighbor_brick_id as usize];
        if neighbor_brick.crosses(&moved_brick) {
            res.insert(*neighbor_brick_id);
        }
    }
    res
}
