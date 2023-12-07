use crate::DayTask;

pub struct Day1;

impl DayTask for Day1 {
    fn get_part1_test_input(&self) -> &'static str {
        "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"
    }

    fn get_part2_test_input(&self) -> &'static str {
        "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"
    }

    fn get_part1_test_result(&self) -> i32 {
        142
    }

    fn get_part2_test_result(&self) -> i32 {
        281
    }


    fn run_p1(&self, lines: Vec<String>) -> i32 {
        lines.iter().map(|l| self.find_numbers(l)).fold(0, |sum, num| sum + num)
    }

    fn run_p2(&self, lines: Vec<String>) -> i32 {
        self.run_p1(
            lines.iter().map(|l| l
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
            ).collect()
        )
    }

}

impl Day1 {
    fn find_numbers(&self, line: &String) -> i32 {
        let digits: Vec<i32> = line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        digits[0] * 10 + digits.last().unwrap()
    }
}