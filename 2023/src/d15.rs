use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

const TI: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

impl DayTask<i64> for Task {

    fn day_no(&self) -> u8 {
        15
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        1320
    }

    fn get_part2_test_result(&self) -> i64 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        todo!()
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        None
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}
