use std::{fs, ops::Range};

const PATH: &str = "src/day4.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).expect("");
    println!(
        "Day 4: \n a) {} \n b) {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn solve_part_one(input: &str) -> i32 {
    parse_range(input)
        .filter(|r| is_increasing(r) && matches_adjacent(r))
        .count() as i32
}

fn solve_part_two(input: &str) -> i32 {
    parse_range(input)
        .filter(|r| is_increasing(r) && matches_only_adjacent(r))
        .count() as i32
}

fn parse_range(input: &str) -> Range<i32> {
    let input: Vec<i32> = input
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    input[0]..input[1]
}

fn to_digits(input: &i32) -> Vec<u32> {
    input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn count_digit(digits: &[u32], digit: u32) -> u32 {
    digits
        .iter()
        .filter(|d| **d == digit)
        .count() as u32
}

fn is_increasing(input: &i32) -> bool {
    to_digits(input).windows(2).all(|w| w[0] <= w[1])
}

fn matches_adjacent(input: &i32) -> bool {
    to_digits(input).windows(2).any(|w| w[0] == w[1])
}

fn matches_only_adjacent(input: &i32) -> bool {
    let digits = to_digits(input);
    digits
        .windows(2)
        .any(|w| w[0] == w[1] && count_digit(&digits, w[0]) == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_digit_in_digits() {
        assert_eq!(count_digit(&[1, 2, 3, 4, 5, 6], 1), 1);
        assert_eq!(count_digit(&[3, 3, 3, 3, 3, 3], 3), 6);
        assert_eq!(count_digit(&[1, 2, 3, 4, 5, 0], 9), 0);
    }

    #[test]
    fn should_split_into_digits() {
        assert_eq!(to_digits(&123456), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(to_digits(&333333), vec![3, 3, 3, 3, 3, 3]);
        assert_eq!(to_digits(&123450), vec![1, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn should_check_for_increasing_digits() {
        assert!(is_increasing(&123456));
        assert!(is_increasing(&333333));
        assert!(!is_increasing(&123450));
        assert!(!is_increasing(&555725));
    }

    #[test]
    fn should_check_for_matching_adjacent_digits() {
        assert!(!matches_adjacent(&123456));
        assert!(matches_adjacent(&333333));
        assert!(matches_adjacent(&123455));
    }

    #[test]
    fn should_check_for_exactly_matching_adjacent_digits() {
        assert!(!matches_only_adjacent(&123456));
        assert!(!matches_only_adjacent(&333333));
        assert!(matches_only_adjacent(&123455));
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).expect("");

        assert_eq!(solve_part_one(&input), 2779);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).expect("");

        assert_eq!(solve_part_two(&input), 1972);
    }
}
