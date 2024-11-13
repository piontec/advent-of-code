use crate::{common::Point2D, DayTask};
use std::{
    collections::{HashMap, HashSet},
    isize,
};

pub struct Task;

const TI: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        23
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        94
    }

    fn get_part2_test_result(&self) -> i64 {
        154
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (map, start, end) = parse(lines);
        let edges = find_edges(start, map, true);

        find_directed_longest_path(edges, start, end)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let (map, start, end) = parse(lines);
        let edges = find_edges(start, map, false);

        find_undirected_longest_path(edges, start, end)
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(1966)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn find_edges(start: Point2D<isize>, map: Vec<Vec<char>>, respect_slopes: bool) -> HashMap<(Point2D<isize>, Point2D<isize>), usize> {
    let mut visited: HashSet<Point2D<isize>> = HashSet::new();
    let mut to_check = vec![(start, Point2D::new(start.x, 1))];
    let mut edges: HashMap<(Point2D<isize>, Point2D<isize>), usize> = HashMap::new();
    while to_check.len() > 0 {
        let start_nodes = to_check.remove(0);
        // we need to skip the start node, as otherwise each node will be counted twice
        let mut edge_length = 0;
        let mut y: isize;
        let mut x: isize;
        let mut prev_y: isize;
        let mut prev_x: isize;
        (prev_y, prev_x) = (start_nodes.0.y, start_nodes.0.x);
        (y, x) = (start_nodes.1.y, start_nodes.1.x);
        let mut next: Vec<Point2D<isize>> = vec![];
        let mut neighbors: Vec<Point2D<isize>> = vec![];
        let mut correct_path = true;
        loop {
            visited.insert(Point2D::new(prev_x, prev_y));
            edge_length += 1;
            if respect_slopes && map[y as usize][x as usize] != '.' {
                if (prev_x < x && map[y as usize][x as usize] != '>')
                    || (prev_x > x && map[y as usize][x as usize] != '<')
                    || (prev_y < y && map[y as usize][x as usize] != 'v')
                    || (prev_y > y && map[y as usize][x as usize] != '^')
                {
                    correct_path = false;
                    break;
                }
                next = match map[y as usize][x as usize] {
                    '>' => vec![Point2D::new(x + 1, y)],
                    '<' => vec![Point2D::new(x - 1, y)],
                    '^' => vec![Point2D::new(x, y - 1)],
                    'v' => vec![Point2D::new(x, y + 1)],
                    _ => panic!("Invalid char"),
                }
            }
            else {
                neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .filter(|(nx, ny)| {
                        *ny >= 0
                            && *ny < map.len() as isize
                            && *nx >= 0
                            && *nx < map[0].len() as isize
                            && map[*ny as usize][*nx as usize] != '#'
                            && !(nx == &start_nodes.0.x && ny == &start_nodes.0.y)
                    })
                    .map(|(nx, ny)| Point2D::new(*nx, *ny))
                    .collect();
                next = neighbors.iter()
                    .filter(|p| !visited.contains(p))
                    .map(|p| *p)
                    .collect();
            }
            // if we can choose multiple paths or there's no next (we're at dest)
            if neighbors.len() >= 3 || next.len() == 0 {
                break;
            }
            prev_y = y;
            prev_x = x;
            y = next[0].y as isize;
            x = next[0].x as isize;
        }
        if !correct_path || edge_length <= 1 {
            continue;
        }
        // if !respect_slopes {
            // visited.insert(Point2D::new(x, y));
        // }
        let end_node = Point2D::new(x, y);
        if !respect_slopes { 
            if !edges.contains_key(&(start_nodes.0, end_node)) && !edges.contains_key(&(end_node, start_nodes.0)) {
                edges.insert((start_nodes.0, end_node), edge_length);
            }
        }
        else {
            edges.insert((start_nodes.0, end_node), edge_length);
        }
        for n in next {
            let new_start = (Point2D::new(x, y), n);
            if !to_check.contains(&new_start) {
                to_check.push(new_start);
            }
        }
    }
    edges
}

fn parse(lines: &Vec<String>) -> (Vec<Vec<char>>, Point2D<isize>, Point2D<isize>) {
    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let start = Point2D::new(map[0].iter().position(|c| *c == '.').unwrap() as isize, 0);
    let end = Point2D::new(
        map[map.len() - 1].iter().position(|c| *c == '.').unwrap() as isize,
        map.len() as isize - 1,
    );
    (map, start, end)
}

fn find_directed_longest_path(edges: HashMap<(Point2D<isize>, Point2D<isize>), usize>, start: Point2D<isize>, end: Point2D<isize>) -> i64 {
    let nodes = edges
        .keys()
        .map(|(a, b)| vec![a, b])
        .flatten()
        .collect::<HashSet<_>>();
    let mut costs: HashMap<Point2D<isize>, usize> = HashMap::new();
    costs.insert(start, 0);
    while !costs.contains_key(&end) {
        // find nodes with no incoming edges from unknown cost nodes
        let next_nodes: HashSet<&Point2D<isize>> = nodes
            .iter()
            .filter(|n| 
                edges
                    .keys()
                    .filter(|(a, b)| b == **n && !costs.contains_key(a))
                    .count() == 0
                && !costs.contains_key(*n))
            .map(|n| *n)
            .collect();
        for n in next_nodes {
            let max_path = edges
                .keys()
                .filter(|(_, b)| b == n)
                .map(|(a, b)| costs[a] + edges[&(*a, *b)])
                .max()
                .unwrap();
            costs.insert(*n, max_path);
        }
    }

    costs[&end] as i64
}

fn find_undirected_longest_path(edges: HashMap<(Point2D<isize>, Point2D<isize>), usize>, start: Point2D<isize>, end: Point2D<isize>) -> i64 {
    let mut to_check   = vec![(start, HashSet::<Point2D<isize>>::new(), 0)];
    let mut max_path = 0;
    while to_check.len() > 0 {
        let (node, path, length) = to_check.remove(0);
        if node == end {
            if length > max_path {
                max_path = length;
            }
            continue;
        }
        let next_edges = edges
            .keys()
            .filter(|(a, b)| a == &node || b == &node)
            .collect::<Vec<_>>();
        for e in next_edges {
            let next_node = if e.0 == node { e.1 } else { e.0 };
            if path.contains(&next_node) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.insert(next_node);
            let next_node = if e.0 == node { e.1 } else { e.0 };
            let new_length = length + edges[e];
            to_check.push((next_node, new_path, new_length));
        }
    }
    max_path as i64
}