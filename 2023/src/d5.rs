use crate::DayTask;
use crossbeam_channel::unbounded;
use std::thread;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub struct Task;

const TI: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        5
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        35
    }

    fn get_part2_test_result(&self) -> i64 {
        46
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i64 {
        let maps = parse_maps(&lines[1..]);

        lines[0]
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| find_loc(s.parse::<u64>().unwrap(), &maps))
            .min()
            .unwrap() as i64
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i64 {
        let maps = parse_maps(&lines[1..]);
        let ranges: Vec<Range> = lines[0]
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .chunks(2)
            .map(|c| Range {
                min: c[0],
                max: c[0] + c[1],
            })
            .collect();

        let mut min = u64::MAX;
        let range_len = ranges.len();
        let (tx_req, rx_req) = unbounded();
        let (tx_resp, rx_resp) = unbounded();
        let maps_ref = &maps;
        thread::scope(|scope| {
            for _ in 0..4 {
                let rx_req = rx_req.clone();
                let tx_resp = tx_resp.clone();
                scope.spawn(move || {
                    loop {
                        match rx_req.recv() {
                            Ok(range) => {
                                let res = find_loc_range(range, maps_ref);
                                tx_resp.send(res).unwrap();
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
            drop(tx_resp);

            for r in ranges {
                tx_req.send(r).unwrap()
            }
            drop(tx_req);

            for _ in 0..range_len {
                let res = rx_resp.recv().unwrap();
                if res < min {
                    min = res;
                }
            }
        });
        min as i64
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(510109797)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(9622622)
    }
}

fn find_loc_range(orig_range: Range, maps: &HashMap<&str, HashMap<Range, i64>>) -> u64 {
    let mut ranges_to_check = HashSet::new();
    ranges_to_check.insert(orig_range);
    for step in vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ] {
        let map = maps.get(step).unwrap();
        let mut new_ranges = HashSet::new();
        while !ranges_to_check.is_empty() {
            let range = ranges_to_check.iter().next().cloned().unwrap();
            ranges_to_check.remove(&range);
            for (map_range, offset) in map.iter() {
                // we have to check 5 cases:
                // - range entirely within map_range,
                if range.min >= map_range.min && range.max <= map_range.max {
                    new_ranges.insert(Range {
                        min: (range.min as i64 + offset) as u64,
                        max: (range.max as i64 + offset) as u64,
                    });
                }
                // - map_range entirely within range,
                else if range.min < map_range.min && range.max > map_range.max {
                    ranges_to_check.insert(Range {
                        min: range.min,
                        max: map_range.min,
                    });
                    new_ranges.insert(Range {
                        min: (map_range.min as i64 + offset) as u64,
                        max: (map_range.max as i64 + offset) as u64,
                    });
                    ranges_to_check.insert(Range {
                        min: map_range.max,
                        max: range.max,
                    });
                }
                // - right side of range overlaps with map_range,
                else if range.min < map_range.min && range.max > map_range.min {
                    ranges_to_check.insert(Range {
                        min: range.min,
                        max: map_range.min,
                    });
                    new_ranges.insert(Range {
                        min: (map_range.min as i64 + offset) as u64,
                        max: (range.max as i64 + offset) as u64,
                    });
                }
                // - left side of range overlaps with map_range,
                else if range.min < map_range.max && range.max > map_range.max {
                    new_ranges.insert(Range {
                        min: (range.min as i64 + offset) as u64,
                        max: (map_range.max as i64 + offset) as u64,
                    });
                    ranges_to_check.insert(Range {
                        min: map_range.max,
                        max: range.max,
                    });
                }
            }
            // range is outside of map_range
            if new_ranges.is_empty() {
                new_ranges.insert(Range { ..range });
            }
        }
        ranges_to_check = new_ranges;
    }
    ranges_to_check.iter().map(|r| r.min).min().unwrap()
}

fn find_loc(seed: u64, maps: &HashMap<&str, HashMap<Range, i64>>) -> u64 {
    let mut current_value = seed;
    'step: for step in vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ] {
        let map = maps.get(step).unwrap();
        for (range, offset) in map.iter() {
            if current_value >= range.min && current_value <= range.max {
                current_value = (current_value as i64 + offset) as u64;
                continue 'step;
            }
        }
    }
    current_value
}

fn parse_maps(lines: &[String]) -> HashMap<&str, SingleMap> {
    let mut maps = HashMap::new();
    let mut header = None;
    for line in lines {
        if line.is_empty() {
            header = None;
            continue;
        }
        if header.is_none() {
            header = Some(line.split(" ").next().unwrap().trim());
            continue;
        }
        if !maps.contains_key(header.unwrap()) {
            maps.insert(header.unwrap(), SingleMap::new());
        }

        let nums = line
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let range = Range {
            min: nums[1],
            max: nums[1] + nums[2] - 1,
        };
        maps.get_mut(header.unwrap())
            .unwrap()
            .insert(range, nums[0] as i64 - nums[1] as i64);
    }

    maps
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Range {
    min: u64,
    max: u64,
}

type SingleMap = HashMap<Range, i64>;
