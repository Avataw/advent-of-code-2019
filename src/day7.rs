use std::{collections::HashMap, fs};

use itertools::Itertools;

const PATH: &str = "src/day7.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).unwrap();

    println!(
        "Day 7: \n a) {} \n b) {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn parse(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn extract_param(memory: &[i64], index: usize, immediate_mode: bool) -> i64 {
    match immediate_mode {
        true => memory[index],
        false => memory[memory[index] as usize],
    }
}

fn add(memory: &[i64], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: first_param + second_param,
        steps: index + 4,
    }
}

fn multiply(memory: &[i64], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: first_param * second_param,
        steps: index + 4,
    }
}

fn jump_if_true(memory: &[i64], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');

    let index = if first_param != 0 {
        second_param as usize
    } else {
        index + 3
    };

    OperationResult {
        target_index: -1,
        target_value: -1,
        steps: index,
    }
}

fn jump_if_false(memory: &[i64], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');

    let index = if first_param == 0 {
        second_param as usize
    } else {
        index + 3
    };

    OperationResult {
        target_index: -1,
        target_value: -1,
        steps: index,
    }
}

fn less_than(memory: &[i64], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: i64::from(first_param < second_param),
        steps: index + 4,
    }
}

fn equal_to(memory: &[i64], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: i64::from(first_param == second_param),
        steps: index + 4,
    }
}

fn set(memory: &[i64], index: usize, input: i64) -> OperationResult {
    let first_param = extract_param(memory, index + 1, true);

    OperationResult {
        target_index: first_param,
        target_value: input,
        steps: index + 2,
    }
}

fn get(memory: &[i64], index: usize, modes: &[char], outputs: &mut Vec<i64>) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');

    outputs.push(first_param);

    OperationResult {
        target_index: -1,
        target_value: -1,
        steps: index + 2,
    }
}

struct IntcodeComputer {
    memory: Vec<i64>,
    running: bool,
    index: usize,
    initialized: bool,
}

impl IntcodeComputer {
    fn run_operations(
        &mut self,
        phase_setting: i64,
        previous_amplifier_result: i64,
        looping: bool,
    ) -> Option<i64> {
        let mut outputs: Vec<i64> = vec![];

        loop {
            let mut current: Vec<char> = self.memory[self.index].to_string().chars().collect();

            while current.len() < 5 {
                current.splice(0..0, ['0']);
            }

            let op_code: String = current.iter().rev().take(2).rev().collect();

            let op_result = match op_code.as_str() {
                "99" => {
                    self.running = false;
                    break;
                }
                "01" => add(&self.memory, self.index, &current),
                "02" => multiply(&self.memory, self.index, &current),
                "03" => {
                    if !self.initialized {
                        self.initialized = true;
                        set(&self.memory, self.index, phase_setting)
                    } else {
                        set(&self.memory, self.index, previous_amplifier_result)
                    }
                }
                "04" => {
                    let op_result = get(&self.memory, self.index, &current, &mut outputs);

                    if looping {
                        self.index = op_result.steps;
                        break;
                    } else {
                        op_result
                    }
                }
                "05" => jump_if_true(&self.memory, self.index, &current),
                "06" => jump_if_false(&self.memory, self.index, &current),
                "07" => less_than(&self.memory, self.index, &current),
                "08" => equal_to(&self.memory, self.index, &current),
                _ => panic!("Operation code {} could not be run!", op_code),
            };

            if op_result.target_index != -1 {
                self.memory[op_result.target_index as usize] = op_result.target_value;
            }

            self.index = op_result.steps;
        }

        outputs.last().copied()
    }
}

fn run_operations(
    memory: &[i64],
    phase_setting: i64,
    previous_amplifier_result: i64,
    looping: bool,
) -> Vec<i64> {
    let mut memory = memory.to_vec();
    let mut outputs: Vec<i64> = vec![];
    let mut index = 0;
    let mut initialized = false;

    loop {
        let mut current: Vec<char> = memory[index].to_string().chars().collect();

        while current.len() < 5 {
            current.splice(0..0, ['0']);
        }

        let op_code: String = current.iter().rev().take(2).rev().collect();

        let op_result = match op_code.as_str() {
            "99" => break,
            "01" => add(&memory, index, &current),
            "02" => multiply(&memory, index, &current),
            "03" => {
                if !initialized {
                    initialized = true;
                    set(&memory, index, phase_setting)
                } else {
                    set(&memory, index, previous_amplifier_result)
                }
            }
            "04" => {
                let op_result = get(&memory, index, &current, &mut outputs);

                if looping {
                    break;
                } else {
                    op_result
                }
            }
            "05" => jump_if_true(&memory, index, &current),
            "06" => jump_if_false(&memory, index, &current),
            "07" => less_than(&memory, index, &current),
            "08" => equal_to(&memory, index, &current),
            _ => panic!("Operation code {} could not be run!", op_code),
        };

        if op_result.target_index != -1 {
            memory[op_result.target_index as usize] = op_result.target_value;
        }

        index = op_result.steps;
    }

    outputs
}

struct OperationResult {
    target_index: i64,
    target_value: i64,
    steps: usize,
}

fn solve_part_one(input: &str) -> i64 {
    let input = parse(input);

    let mut a_results: HashMap<Vec<i64>, i64> = HashMap::new();
    let mut b_results: HashMap<Vec<i64>, i64> = HashMap::new();
    let mut c_results: HashMap<Vec<i64>, i64> = HashMap::new();
    let mut d_results: HashMap<Vec<i64>, i64> = HashMap::new();
    let mut e_results: HashMap<Vec<i64>, i64> = HashMap::new();

    for i in 0..=4 {
        let a_result = *run_operations(&input, i, 0, false).last().unwrap();
        a_results.insert(vec![i], a_result);
    }

    for i in 0..=4 {
        for a_result in a_results.iter().filter(|a| !a.0.contains(&i)) {
            let b_result = *run_operations(&input, i, *a_result.1, false)
                .last()
                .unwrap();

            let mut key = a_result.0.clone();
            key.extend(vec![i]);

            b_results.insert(key, b_result);
        }
    }

    for i in 0..=4 {
        for b_result in b_results.iter().filter(|b| !b.0.contains(&i)) {
            let c_result = *run_operations(&input, i, *b_result.1, false)
                .last()
                .unwrap();

            let mut key = b_result.0.clone();
            key.extend(vec![i]);

            c_results.insert(key, c_result);
        }
    }

    for i in 0..=4 {
        for c_result in c_results.iter().filter(|c| !c.0.contains(&i)) {
            let d_result = *run_operations(&input, i, *c_result.1, false)
                .last()
                .unwrap();

            let mut key = c_result.0.clone();
            key.extend(vec![i]);

            d_results.insert(key, d_result);
        }
    }

    for i in 0..=4 {
        for d_result in d_results.iter().filter(|d| !d.0.contains(&i)) {
            let e_result = *run_operations(&input, i, *d_result.1, false)
                .last()
                .unwrap();

            let mut key = d_result.0.clone();
            key.extend(vec![i]);

            e_results.insert(key, e_result);
        }
    }

    *e_results.values().max().unwrap()
}

fn solve_part_two(input: &str) -> i64 {
    let input = parse(input);

    let phase_settings = vec![5, 6, 7, 8, 9];
    let permutations: Vec<Vec<i64>> = phase_settings.into_iter().permutations(5).collect();

    let mut max_result = 0;

    for permutation in permutations {
        let mut amps: Vec<IntcodeComputer> = (1..=5)
            .map(|_| IntcodeComputer {
                memory: input.clone(),
                running: true,
                initialized: false,
                index: 0,
            })
            .collect();

        let mut previous_amplifier_result = 0;

        while amps.iter().all(|a| a.running) {
            for (i, amp) in amps.iter_mut().enumerate() {
                let phase_setting = permutation[i];

                match amp.run_operations(phase_setting, previous_amplifier_result, true) {
                    Some(result) => previous_amplifier_result = result,
                    None => break,
                }
            }
        }

        if previous_amplifier_result > max_result {
            max_result = previous_amplifier_result
        }
    }

    max_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        assert_eq!(parse("1,0,0,3"), vec![1, 0, 0, 3]);
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).unwrap();
        assert_eq!(solve_part_one(&input), 17406);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).unwrap();
        assert_eq!(solve_part_two(&input), 1047153);
    }
}
