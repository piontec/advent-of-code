use itertools::Iterate;

use crate::{
    common::{MapVector, Point2D},
    DayTask,
};
use std::{collections::HashSet, vec};

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

const TI4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const TI5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

const TI6: &str = "AABAA
AABAA
BBBBB
AABAA
AABAA";

struct Region {
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
        vec![TI, TI4, TI5, TI6]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![140, 772, 1930]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![80, 236, 368, 64 + 9 * 12]
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, true)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        run(lines, false)
    }
}

fn run(lines: &Vec<String>, perimeter_mode: bool) -> i64 {
    let map = MapVector::new(lines, |c| c);
    let map_size = map.get_size();
    let mut visited: HashSet<Point2D<isize>> = HashSet::new();
    let mut regions = vec![];
    for y in 0..map_size.y as isize {
        for x in 0..map_size.x as isize {
            let pos = Point2D::new(x, y);
            if !visited.contains(&pos) {
                let region = find_region(&map, &pos, perimeter_mode);
                visited.extend(region.coords.iter().copied());
                regions.push(region);
            }
        }
    }
    regions.iter().map(|r| (r.area * r.perimeter) as i64).sum()
}

fn find_region(map: &MapVector<char>, pos: &Point2D<isize>, perimeter_mode: bool) -> Region {
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
    let perimeter = if perimeter_mode {
        find_perimeter(map, &visited_local, terrain)
    } else {
        find_sides(map, &visited_local, terrain)
    };
    Region {
        area: visited_local.len(),
        perimeter: perimeter,
        coords: visited_local,
    }
}

fn find_sides(
    map: &MapVector<char>,
    visited_local: &HashSet<Point2D<isize>>,
    terrain: char,
) -> usize {
    // clever way to count perimeter sides: the number of sides is equal to the number of corners (external and internal)
    let peri: Vec<Point2D<isize>> = visited_local.iter().copied().collect();
    if peri.len() == 1 {
        return 4;
    }
    let dead_end_corners = filter_peri(map, terrain, &peri, |nc| nc == 1, |_, _| true) * 2;
    let ul_ext_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc == 2,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x + 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y + 1))
        },
    );
    let ur_ext_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc == 2,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x - 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y + 1))
        },
    );
    let dr_ext_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc == 2,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x - 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y - 1))
        },
    );
    let dl_ext_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc == 2,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x + 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y - 1))
        },
    );
    let ul_int_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc > 1,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x + 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y + 1))
                && map[Point2D::new(p.x + 1, p.y + 1)] != terrain
        },
    );
    let ur_int_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc > 1,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x - 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y + 1))
                && map[Point2D::new(p.x - 1, p.y + 1)] != terrain
        },
    );
    let dr_int_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc > 1,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x - 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y - 1))
                && map[Point2D::new(p.x - 1, p.y - 1)] != terrain
        },
    );
    let dl_int_count = filter_peri(
        map,
        terrain,
        &peri,
        |nc| nc > 1,
        |p, neighbors_pos| {
            neighbors_pos.contains(&Point2D::new(p.x + 1, p.y))
                && neighbors_pos.contains(&Point2D::new(p.x, p.y - 1))
                && map[Point2D::new(p.x + 1, p.y - 1)] != terrain
        },
    );
    dead_end_corners
        + ul_int_count
        + ur_int_count
        + dr_int_count
        + dl_int_count
        + ul_ext_count
        + ur_ext_count
        + dr_ext_count
        + dl_ext_count
}

fn filter_peri(
    map: &MapVector<char>,
    terrain: char,
    peri: &Vec<Point2D<isize>>,
    neigh_count_expr: impl Fn(u8) -> bool,
    filter: impl Fn(&Point2D<isize>, &Vec<Point2D<isize>>) -> bool,
) -> usize {
    peri.iter()
        .filter(|p| {
            let neighbors_pos = map.get_neighbors_pos(p, |&np| map[np] == terrain);
            if neigh_count_expr(neighbors_pos.len() as u8) == false {
                return false;
            }
            filter(p, &neighbors_pos)
        })
        .count()
}

fn find_perimeter(
    map: &MapVector<char>,
    visited_local: &HashSet<Point2D<isize>>,
    terrain: char,
) -> usize {
    let mut perimeter = 0;
    for p in visited_local.iter() {
        let sames = map.get_neighbors(p, |&np| map[np] == terrain);
        perimeter += 4 - sames.len();
    }
    perimeter
}
