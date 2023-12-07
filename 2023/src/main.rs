mod d1;
use std::fs::read_to_string;
use std::time::Instant;

pub trait DayTask {
    fn main(&self, day: u8) {
        println!("[[Day {day} - part 1]]");
    	let now = Instant::now();
    	assert_eq!(self.run_p1(self.get_test_data(self.get_part1_test_input())), self.get_part1_test_result());
    	println!("[test: {}ms]", now.elapsed().as_millis());
        let lines = self.read_lines(format!("d{day}.txt").as_str());
    	let now = Instant::now();
        println!("{}", self.run_p1(lines));
    	println!("[main: {}ms]", now.elapsed().as_millis());

        println!("[[Day {day} - part 2]]");
    	let now = Instant::now();
    	assert_eq!(self.run_p2(self.get_test_data(self.get_part2_test_input())), self.get_part2_test_result());
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

    fn get_part1_test_result(&self) -> i32;

    fn get_part2_test_result(&self) -> i32;

    fn run_p1(&self, lines: Vec<String>) -> i32;

    fn run_p2(&self, lines: Vec<String>) -> i32;
}

fn main() {
    d1::Day1.main(1);
}
