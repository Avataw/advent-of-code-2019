use std::fs;

const PATH: &str = "src/day2.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).unwrap();

    println!(
        "Day 2: \n a) {} \n b) {}",
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

fn initialize(input: &mut [i32], first: i32, second: i32) {
    input[1] = first;
    input[2] = second;
}

fn run_operations(input: &mut [i32]) {
    let mut i = 0;
    loop {
        let value = match input[i] {
            99 => break,
            1 => input[input[i + 1] as usize] + input[input[i + 2] as usize],
            2 => input[input[i + 1] as usize] * input[input[i + 2] as usize],
            _ => panic!("No operation matches"),
        };
        let target = input[i + 3] as usize;
        input[target] = value;
        i += 4;
    }
}

fn solve_part_one(input: &str) -> i32 {
    let mut intcode_program = parse(input);

    initialize(&mut intcode_program, 12, 2);
    run_operations(&mut intcode_program);

    *intcode_program.first().unwrap()
}

fn solve_part_two(input: &str) -> i32 {

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode_program = parse(input);

            initialize(&mut intcode_program, noun, verb);
            run_operations(&mut intcode_program);
            if intcode_program[0] == 19690720 {
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
        let mut input = vec![1, 0, 0, 3, 99];
        run_operations(&mut input);

        assert_eq!(input, vec![1, 0, 0, 2, 99]);
    }

    #[test]
    fn should_multiply() {
        let mut input = vec![2, 0, 0, 3, 99];
        run_operations(&mut input);

        assert_eq!(input, vec![2, 0, 0, 4, 99]);
    }

    #[test]
    fn should_initialize() {
        let mut input = vec![1, 1, 1, 1, 99];
        initialize(&mut input, 12, 2);

        assert_eq!(input, vec![1, 12, 2, 1, 99]);
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).unwrap();
        assert_eq!(solve_part_one(&input), 9706670);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).unwrap();
        assert_eq!(solve_part_two(&input), 2552);
    }
}
