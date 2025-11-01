use crate::{common::Point2D, DayTask};

pub struct Task;

const TI: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        14
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<i64> {
        vec![12]
    }

    fn get_part2_test_result(&self) -> Vec<i64> {
        vec![1]
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(229868730)
    }

    fn get_part2_result(&self) -> Option<i64> {
        Some(7861)
    }

    fn run_p1(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        let mut bots = parse_bots(lines);
        if is_test {
            get_safety_factor(&mut bots, 11, 7, 100)
        } else {
            get_safety_factor(&mut bots, 101, 103, 100)
        }
    }

    fn run_p2(&self, lines: &Vec<String>, is_test: bool) -> i64 {
        let mut bots = parse_bots(lines);
        if is_test {
            1
        } else {
            find_christmas_tree(&mut bots, 101, 103)
        }
    }
}

fn get_safety_factor(bots: &mut [Bot], width: i64, height: i64, iterations: i32) -> i64 {
    let mut ul_count = 0;
    let mut ur_count = 0;
    let mut ll_count = 0;
    let mut lr_count = 0;

    let mid_x = width / 2;
    let mid_y = height / 2;

    for bot in bots {
        let mut new_x = bot.position.x;
        let mut new_y = bot.position.y;
        for _ in 0..iterations {
            new_x = (new_x + bot.velocity.x) % width;
            if new_x < 0 {
                new_x += width;
            }
            new_y = (new_y + bot.velocity.y) % height;
            if new_y < 0 {
                new_y += height;
            }
        }
        bot.position.x = new_x;
        bot.position.y = new_y;
        if bot.position.x < mid_x && bot.position.y < mid_y {
            ul_count += 1;
        } else if bot.position.x > mid_x && bot.position.y < mid_y {
            ur_count += 1;
        } else if bot.position.x < mid_x && bot.position.y > mid_y {
            ll_count += 1;
        } else if bot.position.x > mid_x && bot.position.y > mid_y {
            lr_count += 1;
        }
    }
    ul_count * ur_count * ll_count * lr_count
}

fn find_christmas_tree(bots: &mut [Bot], width: i64, height: i64) -> i64 {
    let mut iters = 0;
    loop {
        for bot in &mut *bots {
            let mut new_x = bot.position.x;
            let mut new_y = bot.position.y;
            new_x = (new_x + bot.velocity.x) % width;
            if new_x < 0 {
                new_x += width;
            }
            new_y = (new_y + bot.velocity.y) % height;
            if new_y < 0 {
                new_y += height;
            }
            bot.position.x = new_x;
            bot.position.y = new_y;
        }

        for y in 0..height {
            let mut row = ".".repeat(width as usize);
            for bot in bots.iter() {
                if bot.position.y == y {
                    row.replace_range(bot.position.x as usize..bot.position.x as usize + 1, "#");
                }
            }
            if row.contains("####################") {
                print_tree(bots, width, height);
                return iters + 1;
            }
        }

        iters += 1;
    }
}

fn print_tree(bots: &[Bot], width: i64, height: i64) {
    for y in 0..height {
        let mut row = ".".repeat(width as usize);
        for bot in bots.iter() {
            if bot.position.y == y {
                row.replace_range(bot.position.x as usize..bot.position.x as usize + 1, "#");
            }
        }
        println!("{}", row);
    }
}

struct Bot {
    position: Point2D<i64>,
    velocity: Point2D<i64>,
}

fn parse_bots(lines: &[String]) -> Vec<Bot> {
    lines
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let pos = parts[0]
                .trim_start_matches("p=")
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let vel = parts[1]
                .trim_start_matches("v=")
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            Bot {
                position: Point2D::new(pos[0], pos[1]),
                velocity: Point2D::new(vel[0], vel[1]),
            }
        })
        .collect()
}
