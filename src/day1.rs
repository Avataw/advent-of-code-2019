use std::fs;

pub fn solve() {
    let input: String = fs::read_to_string("src/day1.txt").expect("");
    println!(
        "Day 1: \n a) {} \n b) {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn solve_part_one(input: &str) -> i32 {
    parse(input).into_iter().map(calc_fuel).sum()
}

fn solve_part_two(input: &str) -> i32 {
    parse(input).into_iter().map(calc_fuel_repeating).sum()
}

fn calc_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calc_fuel_repeating(mass: i32) -> i32 {
    let mut fuel = 0;

    let mut current_mass = mass;

    loop {
        current_mass = calc_fuel(current_mass);

        if current_mass <= 0 {
            break;
        }
        fuel += current_mass;
    }

    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_fuel_from_mass() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn should_calculate_fuel_from_mass_repeating() {
        assert_eq!(calc_fuel_repeating(14), 2);
        assert_eq!(calc_fuel_repeating(1969), 966);
        assert_eq!(calc_fuel_repeating(100756), 50346);
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string("src/day1.txt").expect("");

        assert_eq!(solve_part_one(&input), 3432671);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string("src/day1.txt").expect("");

        assert_eq!(solve_part_two(&input), 5146132);
    }
}
