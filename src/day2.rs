use std::fs;

const PATH: &str = "src/day2.txt";

pub fn solve() -> () {
    let input: String = fs::read_to_string(PATH).expect("");

    println!(
        "Day 2 a) : {} b) : {}",
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

fn initialize(input: &Vec<i32>, first: i32, second: i32) -> Vec<i32> {
    let mut result = input.to_vec();
    result[1] = first;
    result[2] = second;
    result
}

fn run_operations(input: &Vec<i32>) -> Vec<i32> {
    let mut result = input.to_vec();
    let mut i = 0;
    loop {
        let value = match result[i] {
            99 => break,
            1 => result[result[i + 1] as usize] + result[result[i + 2] as usize],
            2 => result[result[i + 1] as usize] * result[result[i + 2] as usize],
            _ => panic!("No operation matches")
        };
        let target = result[i + 3] as usize;
        result[target] = value;
        i += 4;
    }
    result
}

fn solve_part_one(input: &str) -> i32 {
    let intcode_program = parse(input);
    let restored_program = initialize(&intcode_program, 12, 2);
    let result = run_operations(&restored_program);
    result[0]
}

fn solve_part_two(input: &str) -> i32 {
    let intcode_program = parse(input);

    for noun in 0..100 {
        for verb in 0..100 {
            let restored_program = initialize(&intcode_program, noun, verb);
            let result = run_operations(&restored_program);
            if result[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No noun and verb combination worked!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        assert_eq!(parse("1,0,0,3"), vec![1, 0, 0, 3]);
    }

    #[test]
    fn should_add() {
        let input = vec![1, 0, 0, 3, 99];
        assert_eq!(run_operations(&input), vec![1, 0, 0, 2, 99]);
    }

    #[test]
    fn should_multiply() {
        let input = vec![2, 0, 0, 3, 99];
        assert_eq!(run_operations(&input), vec![2, 0, 0, 4, 99]);
    }

    #[test]
    fn should_initialize() {
        let input = vec![1, 1, 1, 1, 99];
        assert_eq!(initialize(&input, 12, 2), vec![1, 12, 2, 1, 99]);
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).expect("");
        assert_eq!(solve_part_one(&input), 9706670);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).expect("");
        assert_eq!(solve_part_two(&input), 2552);
    }
}
