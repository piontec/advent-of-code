use crate::DayTask;
use std::{collections::HashMap, vec};

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

    fn run_p1(&self, lines: Vec<String>) -> i64 {
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

    fn run_p2(&self, lines: Vec<String>) -> i64 {
        let maps = parse_maps(&lines[1..]);
        let seeds = lines[0]
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
            .map(|range| find_loc_range(range, &maps))
            .min()
            .unwrap() as i64;
        seeds
    }
}

fn find_loc_range(orig_range: Range, maps: &HashMap<&str, HashMap<Range, i64>>) -> u64 {
    let mut ranges_to_check = vec![orig_range];
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
        let mut new_ranges = vec![];
        while ! ranges_to_check.is_empty() {
            let range = ranges_to_check.pop().unwrap();
            for (map_range, offset) in map.iter() {
                // we have to check 5 cases:
                // - range entirely within map_range,
                if range.min >= map_range.min && range.max <= map_range.max {
                    new_ranges.push(Range {
                        min: (range.min as i64 + offset) as u64,
                        max: (range.max as i64 + offset) as u64,
                    });
                }
                // - map_range entirely within range,
                else if range.min < map_range.min && range.max > map_range.max {
                    ranges_to_check.push(Range {
                        min: range.min,
                        max: map_range.min,
                    });
                    new_ranges.push(Range {
                        min: (map_range.min as i64 + offset) as u64,
                        max: (map_range.max as i64 + offset) as u64,
                    });
                    ranges_to_check.push(Range {
                        min: map_range.max,
                        max: range.max,
                    });
                }
                // - right side of range overlaps with map_range,
                else if range.min < map_range.min && range.max > map_range.min {
                    ranges_to_check.push(Range {
                        min: range.min,
                        max: map_range.min,
                    });
                    new_ranges.push(Range {
                        min: (map_range.min as i64 + offset) as u64,
                        max: (range.max as i64 + offset) as u64,
                    });
                }
                // - left side of range overlaps with map_range,
                else if range.min < map_range.max && range.max > map_range.max {
                    new_ranges.push(Range {
                        min: (range.min as i64 + offset) as u64,
                        max: (map_range.max as i64 + offset) as u64,
                    });
                    ranges_to_check.push(Range {
                        min: map_range.max,
                        max: range.max,
                    });
                }
            }
            // range is outside of map_range
            if new_ranges.is_empty() {
                new_ranges.push(Range { ..range });
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

#[derive(Eq, Hash, PartialEq)]
struct Range {
    min: u64,
    max: u64,
}

type SingleMap = HashMap<Range, i64>;
