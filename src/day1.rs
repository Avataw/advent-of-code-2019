use std::fs;

pub fn solve() -> () {
    let input: String = fs::read_to_string("src/day1.txt").expect("");
    println!(
        "Day 1 a) : {} b) : {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn parse(input: &str) -> Vec<f32> {
    input.lines().map(|s| s.parse::<f32>().unwrap()).collect()
}

fn solve_part_one(input: &str) -> f32 {
    parse(input).into_iter().map(calc_fuel).sum()
}

fn solve_part_two(input: &str) -> f32 {
    parse(input).into_iter().map(calc_fuel_repeating).sum()
}

fn calc_fuel(mass: f32) -> f32 {
    (mass / 3.0).floor() - 2.0
}

fn calc_fuel_repeating(mass: f32) -> f32 {
    let mut fuel = 0.0;

    let mut current_mass = mass;

    loop {
        current_mass = calc_fuel(current_mass);

        if current_mass <= 0.0 {
            break;
        }
        fuel += current_mass;
    }

    return fuel;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_fuel_from_mass() {
        assert_eq!(calc_fuel(12.0), 2.0);
        assert_eq!(calc_fuel(14.0), 2.0);
        assert_eq!(calc_fuel(1969.0), 654.0);
        assert_eq!(calc_fuel(100756.0), 33583.0);
    }

    #[test]
    fn should_calculate_fuel_from_mass_repeating() {
        assert_eq!(calc_fuel_repeating(14.0), 2.0);
        assert_eq!(calc_fuel_repeating(1969.0), 966.0);
        assert_eq!(calc_fuel_repeating(100756.0), 50346.0);
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string("src/day1.txt").expect("");

        assert_eq!(solve_part_one(&input), 3432671.0);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string("src/day1.txt").expect("");

        assert_eq!(solve_part_two(&input), 5146132.0);
    }
}
