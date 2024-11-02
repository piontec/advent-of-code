use crate::DayTask;
use itertools::Itertools;
use std::{borrow::BorrowMut, collections::{HashMap, HashSet}};

pub struct Task;

const TI: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

type Coord = (i64, i64, i64);

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
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let bricks_coords: Vec<(Coord, Coord)> = lines
            .iter()
            .map(|l| {
                let mut parts = l.split('~');
                let p1 = parts.next().unwrap();
                let p2 = parts.next().unwrap();
                let mut p1 = p1.split(',').map(|s| s.parse::<i64>().unwrap());
                let mut p2 = p2.split(',').map(|s| s.parse::<i64>().unwrap());
                (
                    (p1.next().unwrap(), p1.next().unwrap(), p1.next().unwrap()),
                    (p2.next().unwrap(), p2.next().unwrap(), p2.next().unwrap()),
                )
            })
            .collect();
        let mut bricks: Vec<Vec<(i64, i64, i64)>> = Vec::with_capacity(bricks_coords.len());
        let mut bricks_by_z: HashMap<u16, Vec<i64>> = HashMap::new();
        for (brick_start, brick_end) in bricks_coords {
            let cubes = vec![
                brick_start.0..=brick_end.0,
                brick_start.1..=brick_end.1,
                brick_start.2..=brick_end.2,
            ]
            .into_iter()
            .multi_cartesian_product()
            .map(|v| (v[0], v[1], v[2]))
            .collect();
            bricks.push(cubes);
            let brick_id = bricks.len() - 1;
            for z in brick_start.2..=brick_end.2 {
                bricks_by_z
                    .entry(z as u16)
                    .or_insert(Vec::new())
                    .push(brick_id as i64);
            }
        }
        // move bricks down starting from the bottom
        // floor is at 0, so bricks with z=1 are already on the floor
        let mut new_bricks_by_z: HashMap<u16, Vec<i64>> = HashMap::new();
        let mut moved_bricks_ids: Vec<i64> = Vec::new();
        for z in 1..=*bricks_by_z.keys().max().unwrap() {
            let bricks_in_this_z = bricks_by_z.get(&(z as u16)).unwrap();
            for brick_id in bricks_in_this_z.iter() {
                if moved_bricks_ids.contains(brick_id) {
                    continue;
                }
                let brick = bricks.get(*brick_id as usize).unwrap();
                // does it overlap with any cube of any other brick below its original z?
                let mut overlaps = false;
                let mut moved_down = brick.iter().map(|(x, y, z)| (*x, *y, *z)).collect::<Vec<(i64, i64, i64)>>();
                while !overlaps && moved_down.iter().all(|(_, _, z)| *z > 1) {
                    moved_down = moved_down.iter().map(|(x, y, z)| (*x, *y, *z - 1)).collect::<Vec<(i64, i64, i64)>>();
                    overlaps = bricks.iter().any(|b| {
                        b.iter().any(|(x, y, z)| {
                            moved_down.iter().any(|(mx, my, mz)| {
                                x == mx && y == my && z == mz
                            })
                        })
                    });
                }
                let moved_min_z = *moved_down.iter().map(|(_, _, z)| z).min().unwrap();
                let moved_max_z = *moved_down.iter().map(|(_, _, z)| z).max().unwrap();
                bricks[*brick_id as usize] = moved_down;
                moved_bricks_ids.push(*brick_id);
                for z in moved_min_z..=moved_max_z {
                    new_bricks_by_z
                        .entry(z as u16)
                        .or_insert(Vec::new())
                        .push(*brick_id);
                }
            }
        }
        bricks_by_z = new_bricks_by_z;

        // check what can be disintegrated
        // we can disintegrate a brick if for every brick that is on top of it, there's at least one more different
        // brick supporting it below
        let mut disintegrate_count = 0;
        for bid in 0..bricks.len() {
            let on_top = get_bricks_on_top(bid, &bricks, &bricks_by_z);
            for on_top_id in on_top.iter() {
                let mut supporting_bricks = get_bricks_supporting(*on_top_id, &bricks, &bricks_by_z);
                supporting_bricks.remove(&(bid as i64));
                if supporting_bricks.len() > 0 {
                    disintegrate_count += 1;
                }
            }
        }

        disintegrate_count
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn get_bricks_supporting(bid: i64, bricks: &[Vec<(i64, i64, i64)>], bricks_by_z: &HashMap<u16, Vec<i64>>) -> HashSet<i64> {
    get_bricks_in_z_dist(bid as usize, bricks, bricks_by_z, -1)
}

fn get_bricks_on_top(bid: usize, bricks: &[Vec<(i64, i64, i64)>], bricks_by_z: &HashMap<u16, Vec<i64>>) -> HashSet<i64> {
    get_bricks_in_z_dist(bid as usize, bricks, bricks_by_z, 1)
}

fn get_bricks_in_z_dist(bid: usize, bricks: &[Vec<(i64, i64, i64)>], bricks_by_z: &HashMap<u16, Vec<i64>>, z_dist: i64) -> HashSet<i64> {
    let mut res: HashSet<i64> = HashSet::new();
    let border_z = match z_dist {
        1 => bricks[bid].iter().map(|(_, _, z)| z).max().unwrap(),
        -1 => bricks[bid].iter().map(|(_, _, z)| z).min().unwrap(),
        _ => panic!("Invalid z_dist")
    };
    let border_z_cubes_moved = bricks[bid].iter()
        .filter(|(_, _, z)| *z == *border_z)
        .map(|(x, y, z)| (*x, *y, *z + z_dist))
        .collect::<Vec<(i64, i64, i64)>>();
    for neighbor_brick_id in bricks_by_z.get(&((*border_z + z_dist) as u16)).unwrap() {
        let neighbor_brick = &bricks[*neighbor_brick_id as usize];
        if neighbor_brick.iter().any(|(x, y, z)| {
            border_z_cubes_moved.iter().any(|(mx, my, mz)| {
                x == mx && y == my && z == mz
            })
        }) {
            res.insert(*neighbor_brick_id);
        }
    }
    res
}
