use crate::DayTask;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

pub struct Task;

const TI: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}
trait Module {
    fn process_pulse(&mut self, input: &str, pulse: Pulse) -> Option<Pulse>;
    fn init_inputs(&mut self, inputs: &Vec<&str>);
}

struct FlipFlop {
    state_on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { state_on: false }
    }
}

impl Module for FlipFlop {
    fn process_pulse(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::Low => {
                self.state_on = !self.state_on;
                if self.state_on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
            Pulse::High => None,
        }
    }

    fn init_inputs(&mut self, _: &Vec<&str>) {}
}

struct Conjuction {
    inputs: HashMap<String, Pulse>,
}

impl Module for Conjuction {
    fn process_pulse(&mut self, input: &str, pulse: Pulse) -> Option<Pulse> {
        self.inputs.insert(input.to_string(), pulse);
        if self.inputs.values().all(|&p| p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn init_inputs(&mut self, inputs: &Vec<&str>) {
        for input in inputs {
            self.inputs.insert(input.to_string(), Pulse::Low);
        }
    }
}

struct Broadcaster {}

impl Module for Broadcaster {
    fn process_pulse(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }

    fn init_inputs(&mut self, _: &Vec<&str>) {}
}

impl DayTask<i64> for Task {
    fn day_no(&self) -> u8 {
        20
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i64 {
        11687500
    }

    fn get_part2_test_result(&self) -> i64 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i64 {
        let mut links: HashMap<String, Vec<&str>> = HashMap::new();
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        for line in lines {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let targets: Vec<&str> = parts[1].split(", ").collect();
            let full_module_name = parts[0];
            let module_name = if full_module_name == "broadcaster" {parts[0].to_string()} else {parts[0].chars().skip(1).collect::<String>()};
            links.insert(module_name.clone(), targets);
            let module: Box<dyn Module> = match full_module_name.chars().next().unwrap() {
                '%' => Box::new(FlipFlop::new()),
                '&' => Box::new(Conjuction {
                    inputs: HashMap::new(),
                }),
                'b' => Box::new(Broadcaster {}),
                _ => panic!("Unknown module type"),
            };
            modules.insert(module_name, module);
        }
        for (module_name, _) in links.iter() {
            let module = modules.get_mut(module_name).unwrap();
            let sources = links
                .iter()
                .filter(|(_, destinations)| destinations.contains(&module_name.as_str()))
                .map(|(s, _)| s.as_str())
                .collect();
            module.init_inputs(&sources);
        }

        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let mut pulses_on_inputs = VecDeque::from([("BUTTON", "broadcaster", Pulse::Low)]);
            low_count += 1;
            while !pulses_on_inputs.is_empty() {
                let (src_module, module_name, pulse) = pulses_on_inputs.pop_front().unwrap();
                let module = modules.get_mut(module_name);
                if module.is_none(){
                    continue;
                }
                let module = module.unwrap();
                let new_pulse = module.process_pulse(&src_module, pulse);
                if new_pulse == None {
                    continue;
                }
                for target in links.get(module_name).unwrap() {
                    pulses_on_inputs.push_back((module_name, target, new_pulse.unwrap()));
                    if new_pulse.unwrap() == Pulse::Low {
                        low_count += 1;
                    } else {
                        high_count += 1;
                    }
                }
            }
        }

        low_count * high_count
    }

    fn run_p2(&self, lines: &Vec<String>) -> i64 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i64> {
        Some(869395600)
    }

    fn get_part2_result(&self) -> Option<i64> {
        None
    }
}
