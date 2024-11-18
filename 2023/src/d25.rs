use crate::DayTask;
use std::collections::{HashMap, HashSet};

pub struct Task;

const TI: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        25
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        54
    }

    fn get_part2_test_result(&self) -> i64 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        let skip_edges = if is_test {
            vec![("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")]
        } else {
            // found using a python script to prepare dot input, then dot, inkscape and my own eyes
            vec![("jzj", "vkb"), ("vrx", "hhx"), ("grh", "nvh")]
        };
        let mut next_nodes = HashMap::<String, HashSet<String>>::new();
        for line in lines {
            let parts: Vec<&str> = line.split(":").collect();
            let source = parts[0].to_string();
            let destinations = parts[1].strip_prefix(" ").unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            for d in &destinations {
                next_nodes
                    .entry(d.to_string())
                    .or_insert(HashSet::new())
                    .insert(source.clone());
                next_nodes
                    .entry(source.clone())
                    .or_insert(HashSet::new())
                    .insert(d.to_string());
            }
        }
        for se in &skip_edges {
            next_nodes.get_mut(se.0).unwrap().retain(|n| *n != se.1);
            next_nodes.get_mut(se.1).unwrap().retain(|n| *n != se.0);
        }
        let res = count_nodes(&next_nodes, &skip_edges[0].0);
        res * count_nodes(&next_nodes, &skip_edges[0].1)
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(546804)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}

fn count_nodes(next_nodes: &HashMap<String, HashSet<String>>, node: &str) -> i64 {
    let mut visited = HashSet::<String>::new();
    let mut count = 0;
    let mut to_visit = vec![node.to_string()];
    while to_visit.len() > 0 {
        let current = to_visit.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        count += 1;
        to_visit.extend(next_nodes.get(&current).unwrap().iter().cloned());
    }
    count
}
