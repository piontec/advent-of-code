use crate::DayTask;

pub struct Day2;
const TI: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

impl DayTask<i32> for Day2 {
    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i32 {
        8
    }

    fn get_part2_test_result(&self) -> i32 {
        2286
    }


    fn run_p1(&self, lines: Vec<String>) -> i32 {
        let games = self.parse(lines);
        let target = Subset {
            green: 13,
            blue: 14,
            red: 12,
        };
        let result: i32 = games.iter().fold(0, |sum, g|
            sum + if g.subsets.iter().all(|s| s.smaller_equal(&target)) {g.id as i32} else {0}
        );
        result
    }

    fn run_p2(&self, lines: Vec<String>) -> i32 {
        let games = self.parse(lines);
        let powers = games.iter().map(|g| {
            let mut max_green = 0;
            let mut max_blue = 0;
            let mut max_red = 0;
            for subset in &g.subsets {
                if subset.green > max_green {
                    max_green = subset.green;
                }
                if subset.blue > max_blue {
                    max_blue = subset.blue;
                }
                if subset.red > max_red {
                    max_red = subset.red;
                }
            }
            let power: i32 = max_blue as i32 * max_green as i32 * max_red as i32;
            power
        }).collect::<Vec<i32>>();
        powers.iter().sum()
    }

    fn day_no(&self) -> u8 {
        2
    }

}

struct Subset {
    green: u8,
    blue: u8,
    red: u8,
}

impl Subset {
    fn smaller_equal(&self, other: &Subset) -> bool {
        self.green <= other.green && self.blue <= other.blue && self.red <= other.red
    }
}

struct Game {
    id: u8,
    subsets: Vec<Subset>,
}

impl Day2 {
    fn parse(&self, lines: Vec<String>) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        for line in lines {
            let main_parts = line.split(":").collect::<Vec<&str>>();
            let game_no = main_parts[0][5..].parse::<u8>().unwrap();
            let mut game = Game {
                id: game_no,
                subsets: Vec::new(),
            };
            for subset in main_parts[1].split(";") {
                let subset_parts = subset.split(",").collect::<Vec<&str>>();
                let mut green: u8 = 0;
                let mut blue: u8 = 0;
                let mut red: u8 = 0;

                for part in subset_parts {
                    let parts = part.trim().split(" ").collect::<Vec<&str>>();
                    let count = parts[0].parse::<u8>().unwrap();
                    match parts[1] {
                        "green" => green = count,
                        "blue" => blue = count,
                        "red" => red = count,
                        _ => panic!("Unknown color"),
                    }
                }

                let subset = Subset {
                    green,
                    blue,
                    red,
                };
                game.subsets.push(subset);
            }
            games.push(game);
        }
        games
    }
}
