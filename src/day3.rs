use std::{
    collections::{HashMap, HashSet},
    fs,
};

const PATH: &str = "src/day3.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).unwrap();
    println!(
        "Day 3: \n a) {} \n b) {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn solve_part_one(input: &str) -> i32 {
    let wires: Vec<Wire> = input
        .lines()
        .map(|line| Wire::new(line.split(',').collect()))
        .collect();

    let first_visited: HashSet<Position> = wires[0].visited_at.keys().cloned().collect();
    let second_visited: HashSet<Position> = wires[1].visited_at.keys().cloned().collect();

    let center_point = Position { x: 0, y: 0 };

    first_visited
        .intersection(&second_visited)
        .map(|i| i.manhattan_distance_from(&center_point))
        .min()
        .unwrap()
}

fn solve_part_two(input: &str) -> i32 {
    let wires: Vec<Wire> = input
        .lines()
        .map(|line| Wire::new(line.split(',').collect()))
        .collect();

    let first_visited: HashSet<Position> = wires[0].visited_at.keys().cloned().collect();
    let second_visited: HashSet<Position> = wires[1].visited_at.keys().cloned().collect();

    first_visited
        .intersection(&second_visited)
        .map(|i| wires[0].visited_at.get(i).unwrap() + wires[1].visited_at.get(i).unwrap())
        .min()
        .unwrap()
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn manhattan_distance_from(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Default)]
struct Wire {
    position: Position,
    steps: i32,
    visited_at: HashMap<Position, i32>,
}

impl Wire {
    fn new(movements: Vec<&str>) -> Wire {
        let mut wire = Wire {
            ..Default::default()
        };

        for movement in movements {
            let value = movement[1..movement.len()].parse::<i32>().unwrap();

            let (first_letter, _) = movement.split_at(1);

            for _ in 0..value {
                match first_letter {
                    "U" => wire.move_by(0, 1),
                    "R" => wire.move_by(1, 0),
                    "D" => wire.move_by(0, -1),
                    "L" => wire.move_by(-1, 0),
                    _ => panic!("Movement {} not recognized!", first_letter),
                }
            }
        }

        wire
    }

    fn move_by(&mut self, x: i32, y: i32) {
        self.position.x += x;
        self.position.y += y;
        self.steps += 1;
        self.visited_at.insert(self.position, self.steps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_distance() {
        let first = Position { x: 100, y: 100 };
        let second = Position { x: 0, y: 0 };

        assert_eq!(first.manhattan_distance_from(&second), 200);
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).unwrap();

        assert_eq!(solve_part_one(&input), 1983);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).unwrap();

        assert_eq!(solve_part_two(&input), 107754);
    }
}
