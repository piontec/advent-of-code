mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::time::Instant;

pub trait DayTask<T: Debug + Display + std::cmp::Eq> {
    fn day_no(&self) -> u8;

    fn main(&self) {
        let day = self.day_no();
        println!("[[Day {day} - part 1]]");
        let now = Instant::now();
        assert_eq!(
            self.run_p1(self.get_test_data(self.get_part1_test_input())),
            self.get_part1_test_result()
        );
        println!("[test: {}ms]", now.elapsed().as_millis());
        let lines = self.read_lines(format!("d{day}.txt").as_str());
        let now = Instant::now();
        println!("{}", self.run_p1(lines));
        println!("[main: {}ms]", now.elapsed().as_millis());

        println!("[[Day {day} - part 2]]");
        let now = Instant::now();
        assert_eq!(
            self.run_p2(self.get_test_data(self.get_part2_test_input())),
            self.get_part2_test_result()
        );
        println!("[test: {}ms]", now.elapsed().as_millis());
        let lines = self.read_lines(format!("d{day}.txt").as_str());
        let now = Instant::now();
        println!("{}", self.run_p2(lines));
        println!("[main: {}ms]", now.elapsed().as_millis());
    }

    fn read_lines(&self, filename: &str) -> Vec<String> {
        read_to_string(filename)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect() // gather them together into a vector
    }

    fn get_test_data(&self, input: &str) -> Vec<String> {
        input.lines().map(String::from).collect()
    }

    fn get_part1_test_input(&self) -> &'static str;

    fn get_part2_test_input(&self) -> &'static str;

    fn get_part1_test_result(&self) -> T;

    fn get_part2_test_result(&self) -> T;

    fn run_p1(&self, lines: Vec<String>) -> T;

    fn run_p2(&self, lines: Vec<String>) -> T;
}

fn main() {
    // d1::Task.main();
    // d2::Task.main();
    // d3::Task.main();
    // d4::Task.main();
    d5::Task.main();
}
