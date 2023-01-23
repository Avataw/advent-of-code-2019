use std::{collections::HashMap, fs};

const PATH: &str = "src/day9.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).unwrap();

    println!(
        "Day 9: \n a) {} \n b) {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn parse(input: &str) -> HashMap<usize, i64> {
    input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect()
}

struct IntcodeComputer {
    memory: HashMap<usize, i64>,
    running: bool,
    index: usize,
    relative_base: i64,
}

impl IntcodeComputer {
    fn extract_param(&self, index: i64, mode: char, literal: bool) -> i64 {
        let length = self.memory.len();

        if literal {
            match mode {
                '0' => *self.memory.get(&(index as usize)).unwrap_or(&0),
                '1' => *self.memory.get(&(index as usize)).unwrap_or(&0),
                '2' => *self.memory.get(&(index as usize)).unwrap_or(&0) + self.relative_base,
                _ => panic!("Literal Param could not be extracted"),
            }
        } else {
            match mode {
                '0' => {
                    if index >= length as i64 {
                        return 0;
                    };

                    let inner_index = *self.memory.get(&(index as usize)).unwrap();

                    *self.memory.get(&(inner_index as usize)).unwrap_or(&0)
                }
                '1' => *self.memory.get(&(index as usize)).unwrap_or(&0),
                '2' => {
                    let inner_index = *self.memory.get(&(index as usize)).unwrap();

                    *self
                        .memory
                        .get(&((self.relative_base + inner_index) as usize))
                        .unwrap_or(&0)
                }
                _ => panic!("Param could not be extracted!"),
            }
        }
    }

    fn add(&self, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);
        let second_param = self.extract_param(self.index as i64 + 2, modes[1], false);
        let third_param = self.extract_param(self.index as i64 + 3, modes[0], true);

        OperationResult {
            target_index: third_param,
            target_value: first_param + second_param,
            steps: self.index + 4,
        }
    }

    fn multiply(&self, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);
        let second_param = self.extract_param(self.index as i64 + 2, modes[1], false);
        let third_param = self.extract_param(self.index as i64 + 3, modes[0], true);

        OperationResult {
            target_index: third_param,
            target_value: first_param * second_param,
            steps: self.index + 4,
        }
    }

    fn jump_if_true(&self, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);
        let second_param = self.extract_param(self.index as i64 + 2, modes[1], false);

        let index = if first_param != 0 {
            second_param as usize
        } else {
            self.index + 3
        };

        OperationResult {
            target_index: -1,
            target_value: -1,
            steps: index,
        }
    }

    fn jump_if_false(&self, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);
        let second_param = self.extract_param(self.index as i64 + 2, modes[1], false);

        let index = if first_param == 0 {
            second_param as usize
        } else {
            self.index + 3
        };

        OperationResult {
            target_index: -1,
            target_value: -1,
            steps: index,
        }
    }

    fn less_than(&self, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);
        let second_param = self.extract_param(self.index as i64 + 2, modes[1], false);
        let third_param = self.extract_param(self.index as i64 + 3, modes[0], true);

        OperationResult {
            target_index: third_param,
            target_value: i64::from(first_param < second_param),
            steps: self.index + 4,
        }
    }

    fn equal_to(&self, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);
        let second_param = self.extract_param(self.index as i64 + 2, modes[1], false);
        let third_param = self.extract_param(self.index as i64 + 3, modes[0], true);

        OperationResult {
            target_index: third_param,
            target_value: i64::from(first_param == second_param),
            steps: self.index + 4,
        }
    }

    fn set(&self, input: i64, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], true);

        OperationResult {
            target_index: first_param,
            target_value: input,
            steps: self.index + 2,
        }
    }

    fn get(&self, modes: &[char], outputs: &mut Vec<i64>) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);

        outputs.push(first_param);

        OperationResult {
            target_index: -1,
            target_value: -1,
            steps: self.index + 2,
        }
    }

    fn adjust_base(&mut self, modes: &[char]) -> OperationResult {
        let first_param = self.extract_param(self.index as i64 + 1, modes[2], false);

        self.relative_base = self.relative_base + first_param;

        OperationResult {
            target_index: -1,
            target_value: -1,
            steps: self.index + 2,
        }
    }

    fn run_operations(&mut self, input_value: i64) -> Vec<i64> {
        let mut outputs: Vec<i64> = vec![];

        loop {
            let mut current: Vec<char> = self
                .memory
                .get(&(self.index as usize))
                .unwrap()
                .to_string()
                .chars()
                .collect();

            while current.len() < 5 {
                current.splice(0..0, ['0']);
            }

            let op_code: String = current.iter().rev().take(2).rev().collect();

            let op_result = match op_code.as_str() {
                "99" => {
                    self.running = false;
                    break;
                }
                "01" => self.add(&current),
                "02" => self.multiply(&current),
                "03" => self.set(input_value, &current),
                "04" => self.get(&current, &mut outputs),
                "05" => self.jump_if_true(&current),
                "06" => self.jump_if_false(&current),
                "07" => self.less_than(&current),
                "08" => self.equal_to(&current),
                "09" => self.adjust_base(&current),
                _ => panic!("Operation code {} could not be run!", op_code),
            };

            if op_result.target_index != -1 {
                self.memory
                    .insert(op_result.target_index as usize, op_result.target_value);
            }

            self.index = op_result.steps;
        }

        outputs.clone()
    }
}

struct OperationResult {
    target_index: i64,
    target_value: i64,
    steps: usize,
}

fn solve_part_one(input: &str) -> i64 {
    let input = parse(input);

    let mut intcode_computer: IntcodeComputer = IntcodeComputer {
        memory: input,
        running: true,
        index: 0,
        relative_base: 0,
    };

    let result = intcode_computer.run_operations(1);

    *result.last().unwrap()
}

fn solve_part_two(input: &str) -> i64 {
    let input = parse(input);

    let mut intcode_computer: IntcodeComputer = IntcodeComputer {
        memory: input,
        running: true,
        index: 0,
        relative_base: 0,
    };

    let result = intcode_computer.run_operations(2);

    *result.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_tests() {
        assert_eq!(
            solve_part_one("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
            99
        );
        assert_eq!(
            solve_part_one("1102,34915192,34915192,7,4,7,99,0"),
            1219070632396864
        );
        assert_eq!(solve_part_one("104,1125899906842624,99"), 1125899906842624);

        assert_eq!(solve_part_one("109,-1,4,1,99"), -1);
        assert_eq!(solve_part_one("109,-1,104,1,99"), 1);
        assert_eq!(solve_part_one("109,-1,204,1,99"), 109);
        assert_eq!(solve_part_one("109,1,9,2,204,-6,99"), 204);
        assert_eq!(solve_part_one("109,1,109,9,204,-6,99"), 204);
        assert_eq!(solve_part_one("109,1,209,-1,204,-106,99"), 204);
        assert_eq!(solve_part_one("109,1,3,3,204,2,99"), 1);
        assert_eq!(solve_part_one("109,1,203,2,204,2,99"), 1);
        assert_eq!(solve_part_one("109,1,203,11,209,8,204,1,99,10,0,42,0"), 1);
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).unwrap();
        assert_eq!(solve_part_one(&input), 2518058886);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).unwrap();
        assert_eq!(solve_part_two(&input), 44292);
    }
}
