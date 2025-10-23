use crate::{
    common::{MapVector, Point2D},
    DayTask,
};
use std::collections::HashSet;

pub struct Task;

const TI: &str = "AAAA
BBCD
BBCC
EEEC";

const TI2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const TI3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

struct Region {
    terrain: char,
    area: usize,
    perimeter: usize,
    coords: HashSet<Point2D<isize>>,
}

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        12
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI, TI2, TI3]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI, TI2, TI3]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![140, 772, 1930]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let map = MapVector::new(lines, |c| c);
        let map_size = map.get_size();
        let mut visited: HashSet<Point2D<isize>> = HashSet::new();
        let mut regions = vec![];
        for y in 0..map_size.y as isize {
            for x in 0..map_size.x as isize {
                let pos = Point2D::new(x, y);
                if !visited.contains(&pos) {
                    let region = find_region(&map, &pos);
                    visited.extend(region.coords.iter().copied());
                    regions.push(region);
                }
            }
        }
        regions.iter().map(|r| (r.area * r.perimeter) as i64).sum()
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }
}

fn find_region(map: &MapVector<char>, pos: &Point2D<isize>) -> Region {
    let mut visited_local: HashSet<Point2D<isize>> = HashSet::new();
    let mut to_visit = vec![*pos];
    let terrain = map[*pos];
    while let Some(current) = to_visit.pop() {
        visited_local.insert(current);
        for neighbor_pos in map.get_neighbors_pos(&current, |&np| map[np] == terrain) {
            if !visited_local.contains(&neighbor_pos) {
                to_visit.push(neighbor_pos);
            }
        }
    }
    let mut perimeter = 0;
    for p in visited_local.iter() {
        let sames = map.get_neighbors(p, |&np| map[np] == terrain);
        perimeter += 4 - sames.len();
    }
    Region {
        terrain: terrain,
        area: visited_local.len(),
        perimeter: perimeter,
        coords: visited_local,
    }
}
