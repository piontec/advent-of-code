use std::fs::read_to_string;
use std::time::Instant;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn get_test_data() -> Vec<String> {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    input.lines().map(String::from).collect()
}

fn find_numbers(line: &String) -> i32 {
    let digits: Vec<i32> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    digits[0] * 10 + digits.last().unwrap()
}

fn run(lines: Vec<String>) -> i32 {
    lines.iter().map(find_numbers).fold(0, |sum, num| sum + num)
}

pub fn main() {
	let now = Instant::now();
	assert_eq!(run(get_test_data()), 142);
	println!("[test: {}ms]", now.elapsed().as_millis());
    let lines = read_lines("d1.txt");
	let now = Instant::now();
	println!("[main: {}ms]", now.elapsed().as_millis());
    println!("{}", run(lines));
}
