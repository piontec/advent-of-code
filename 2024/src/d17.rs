use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

const TI: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

struct CPU {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    program: Vec<usize>,
    stdout: String,
}

impl CPU {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();

        // Parse register A
        let a = lines[0]
            .strip_prefix("Register A: ")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        // Parse register B
        let b = lines[1]
            .strip_prefix("Register B: ")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        // Parse register C
        let c = lines[2]
            .strip_prefix("Register C: ")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        // Parse program (skip blank line at index 3)
        let program_line = lines[4].strip_prefix("Program: ").unwrap();
        let program: Vec<usize> = program_line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        CPU {
            a,
            b,
            c,
            ip: 0, // Initialize instruction pointer to 0
            program,
            stdout: String::new(),
        }
    }

    fn _get_combo(&self) -> i64 {
        match self.program[self.ip + 1] {
            op if op <= 3 => op as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            v @ _ => panic!("Invalid operand: {}", v),
        }
    }

    fn run(&mut self) -> String {
        while self.ip < self.program.len() {
            let instr = self.program[self.ip];
            let literal = self.program[self.ip + 1] as i64;
            match instr {
                // adv
                0 => {
                    self.a = self.a / 2i64.pow(self._get_combo() as u32);
                }
                // bxl
                1 => {
                    self.b = self.b ^ literal;
                }
                // bst
                2 => {
                    self.b = self._get_combo() % 8;
                }
                // jnz
                3 => {
                    if self.a != 0 {
                        self.ip = literal as usize;
                        continue;
                    }
                }
                // bxc
                4 => {
                    self.b = self.b ^ self.c;
                }
                // out
                5 => {
                    self.stdout.push_str(&format!("{},", self._get_combo() % 8));
                }
                // bdv
                6 => {
                    self.b = self.a / 2i64.pow(self._get_combo() as u32);
                }
                // cdv
                7 => {
                    self.c = self.a / 2i64.pow(self._get_combo() as u32);
                }
                _ => {
                    panic!("Unknown instruction: {}", instr);
                }
            }
            self.ip += 2;
        }
        self.stdout.clone()
    }
}

impl DayTask<String> for Task {
    fn day_no(&self) -> u8 {
        17
    }

    fn get_part1_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part2_test_input(&self) -> Vec<&'static str> {
        vec![TI]
    }

    fn get_part1_test_result(&self) -> Vec<String> {
        vec![String::from("4,6,3,5,6,3,5,2,1,0,")]
    }

    fn get_part2_test_result(&self) -> Vec<String> {
        todo!()
    }

    fn get_part1_result(&self) -> Option<String> {
        None
    }

    fn get_part2_result(&self) -> Option<String> {
        None
    }

    fn run_p1(&self, lines: &Vec<String>, _is_test: bool) -> String {
        let mut cpu = CPU::parse(lines.join("\n").as_str());
        cpu.run();
        print!("{}", cpu.stdout);
        cpu.stdout
    }

    fn run_p2(&self, lines: &Vec<String>, _is_test: bool) -> String {
        todo!()
    }
}
