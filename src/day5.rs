use std::fs;

const PATH: &str = "src/day5.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).expect("");

    println!(
        "Day 5: \n a) {} \n b) {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn parse(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn extract_param(memory: &[i32], index: usize, immediate_mode: bool) -> i32 {
    match immediate_mode {
        true => memory[index],
        false => memory[memory[index] as usize],
    }
}

fn add(memory: &[i32], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: first_param + second_param,
        steps: index + 4,
    }
}

fn multiply(memory: &[i32], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: first_param * second_param,
        steps: index + 4,
    }
}

fn jump_if_true(memory: &[i32], index: usize, modes: &[char]) -> OperationResult {
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

fn jump_if_false(memory: &[i32], index: usize, modes: &[char]) -> OperationResult {
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

fn less_than(memory: &[i32], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: i32::from(first_param < second_param),
        steps: index + 4,
    }
}

fn equal_to(memory: &[i32], index: usize, modes: &[char]) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');
    let second_param = extract_param(memory, index + 2, modes[1] == '1');
    let third_param = extract_param(memory, index + 3, true);

    OperationResult {
        target_index: third_param,
        target_value: i32::from(first_param == second_param),
        steps: index + 4,
    }
}

fn set(memory: &[i32], index: usize, input: i32) -> OperationResult {
    let first_param = extract_param(memory, index + 1, true);

    OperationResult {
        target_index: first_param,
        target_value: input,
        steps: index + 2,
    }
}

fn get(memory: &[i32], index: usize, modes: &[char], outputs: &mut Vec<i32>) -> OperationResult {
    let first_param = extract_param(memory, index + 1, modes[2] == '1');

    outputs.push(first_param);

    OperationResult {
        target_index: -1,
        target_value: -1,
        steps: index + 2,
    }
}

fn run_operations(memory: &[i32], input: i32) -> Vec<i32> {
    let mut memory = memory.to_vec();
    let mut outputs: Vec<i32> = vec![];
    let mut index = 0;

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
            "03" => set(&memory, index, input),
            "04" => get(&memory, index, &current, &mut outputs),
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
    target_index: i32,
    target_value: i32,
    steps: usize,
}

fn solve_part_one(input: &str) -> i32 {
    let intcode_program = parse(input);
    *run_operations(&intcode_program, 1).last().unwrap()
}

fn solve_part_two(input: &str) -> i32 {
    let intcode_program = parse(input);
    *run_operations(&intcode_program, 5).last().unwrap()
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
        let input: String = fs::read_to_string(PATH).expect("");
        assert_eq!(solve_part_one(&input), 15386262);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).expect("");
        assert_eq!(solve_part_two(&input), 10376124);
    }
}
