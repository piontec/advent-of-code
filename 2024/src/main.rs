pub mod common;
mod d1;
// mod d10;
// mod d11;
// mod d12;
// mod d13;
// mod d14;
// mod d15;
// mod d16;
// mod d17;
// mod d18;
// mod d19;
mod d2;
// mod d20;
// mod d21;
// mod d22;
// mod d23;
// mod d24;
// mod d25;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::time::Instant;

pub trait DayTask<T: Debug + Display + std::cmp::Eq> {
    fn day_no(&self) -> u8;

    fn main(&self) {
        let day = self.day_no();
        let lines = self.read_lines(format!("d{day}.txt").as_str());

        println!("[[Day {day} - part 1]]");
        let now = Instant::now();
        for i in 0..self.get_part1_test_input().len() {
            assert_eq!(
                self.run_p1(&self.get_test_data(self.get_part1_test_input()[i]), true),
                self.get_part1_test_result()[i]
            );
        }
        println!("[test: {}ms]", now.elapsed().as_millis());
        let now = Instant::now();
        let result1 = self.run_p1(&lines, false);
        println!("{result1}");
        println!("[main: {}ms]", now.elapsed().as_millis());
        if let Some(res) = self.get_part1_result() {
            assert_eq!(result1, res);
        }

        println!("[[Day {day} - part 2]]");
        let now = Instant::now();
        for i in 0..self.get_part2_test_input().len() {
            assert_eq!(
                self.run_p2(&self.get_test_data(self.get_part2_test_input()[i]), true),
                self.get_part2_test_result()[i]
            );
        }
        println!("[test: {}ms]", now.elapsed().as_millis());
        let now = Instant::now();
        let result2 = self.run_p2(&lines, false);
        println!("{result2}");
        println!("[main: {}ms]", now.elapsed().as_millis());
        if let Some(res) = self.get_part2_result() {
            assert_eq!(result2, res);
        }
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

    fn get_part1_test_input(&self) -> Vec<&'static str>;

    fn get_part2_test_input(&self) -> Vec<&'static str>;

    fn get_part1_test_result(&self) -> Vec<T>;

    fn get_part2_test_result(&self) -> Vec<T>;

    fn run_p1(&self, lines: &Vec<String>, is_test: bool) -> T;

    fn run_p2(&self, lines: &Vec<String>, is_test: bool) -> T;

    fn get_part1_result(&self) -> Option<T>;

    fn get_part2_result(&self) -> Option<T>;
}

fn main() {
    // d1::Task.main();
    // d2::Task.main();
    // d3::Task.main();
    // d4::Task.main();
    // d5::Task.main();
    d6::Task.main();
    // d7::Task.main();
    // d8::Task.main();
    // d9::Task.main();
    // d10::Task.main();
    // d11::Task.main();
    // d12::Task.main();
    // d13::Task.main();
    // d14::Task.main();
    // d15::Task.main();
    // d16::Task.main();
    // d17::Task.main();
    // d18::Task.main();
    // d19::Task.main();
    // d20::Task.main();
    // d21::Task.main();
    // d22::Task.main();
    // d23::Task.main();
    // d24::Task.main();
    // d25::Task.main();
}
