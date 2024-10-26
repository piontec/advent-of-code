use crate::DayTask;

pub struct Task;

const TI: &str = "Time:      7  15   30
Distance:  9  40  200";

impl DayTask<i32> for Task {
    fn day_no(&self) -> u8 {
        6
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i32 {
        288
    }

    fn get_part2_test_result(&self) -> i32 {
        71503
    }

    fn run_p1(&self, lines: &Vec<String>, _: bool) -> i32 {
        let times = lines[0]
            .split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let distances = lines[1]
            .split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut margin = 1;
        for race_idx in 0..times.len() {
            let t = times[race_idx];
            let d = distances[race_idx];

            let counter = make_race(t, d);
            margin *= counter;
        }

        margin
    }

    fn run_p2(&self, lines: &Vec<String>, _: bool) -> i32 {
        let time = lines[0].replace(" ", "").split(":").nth(1).unwrap().parse::<i64>().unwrap();
        let dist = lines[1].replace(" ", "").split(":").nth(1).unwrap().parse::<i64>().unwrap();

        make_race(time, dist)
    }

    fn get_part1_result(&self) -> Option<i32> {
        Some(1108800)
    }

    fn get_part2_result(&self) -> Option<i32> {
        Some(36919753)
    }
}

fn make_race(t: i64, d: i64) -> i32 {
    let delta = t * t - 4 * d;
    assert!(delta > 0);
    let x1 = (-t as f64 + (delta as f64).sqrt()) / -2.0;
    let x2 = (-t as f64 - (delta as f64).sqrt()) / -2.0;
    let x1_rounded = x1.ceil() as i32;
    let x2_rounded = x2.floor() as i32;
    let mut counter = (x1_rounded - x2_rounded).abs() + 1;
    if x1_rounded as f64 == x1 {
        counter -= 1;
    }
    if x2_rounded as f64 == x2 {
        counter -= 1;
    }
    counter
}
