use std::{
    collections::{HashMap, HashSet},
    fs,
};

const PATH: &str = "src/day3.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).expect("");
    println!(
        "Day 3: \n a) {} \n b) {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn solve_part_one(input: &str) -> i32 {
    let mut wires: Vec<Wire> = vec![];

    for line in input.lines() {
        let mut wire = Wire::new();

        let movements = line.split(',');

        for movement in movements {
            let value = movement[1..movement.len()].parse::<i32>().unwrap();

            for _ in 0..value {
                if movement.starts_with('R') {
                    wire.move_right();
                } else if movement.starts_with('L') {
                    wire.move_left();
                } else if movement.starts_with('U') {
                    wire.move_up()
                } else if movement.starts_with('D') {
                    wire.move_down()
                }
            }
        }

        wires.push(wire)
    }

    let first_visited: HashSet<Position> = wires[0].visited_at.keys().cloned().collect();
    let second_visited: HashSet<Position> = wires[1].visited_at.keys().cloned().collect();

    let center_point = Position { x: 0, y: 0 };

    let mut intersections_distances: Vec<i32> = first_visited
        .intersection(&second_visited)
        .map(|i| i.distance_from(&center_point))
        .collect();

    intersections_distances.sort();

    intersections_distances[0]
}

fn solve_part_two(input: &str) -> i32 {
    let mut wires: Vec<Wire> = vec![];

    for line in input.lines() {
        let mut wire = Wire::new();

        let movements = line.split(',');

        for movement in movements {
            let value = movement[1..movement.len()].parse::<i32>().unwrap();

            for _ in 0..value {
                if movement.starts_with('R') {
                    wire.move_right();
                } else if movement.starts_with('L') {
                    wire.move_left();
                } else if movement.starts_with('U') {
                    wire.move_up()
                } else if movement.starts_with('D') {
                    wire.move_down()
                }
            }
        }

        wires.push(wire)
    }

    let first_visited: HashSet<Position> = wires[0].visited_at.keys().cloned().collect();
    let second_visited: HashSet<Position> = wires[1].visited_at.keys().cloned().collect();

    let mut intersections_distances: Vec<i32> = first_visited.intersection(&second_visited)
        .map(|i| wires[0].visited_at.get(i).unwrap() + wires[1].visited_at.get(i).unwrap())
        .collect();

    intersections_distances.sort();

    intersections_distances[0]
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance_from(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Wire {
    position: Position,
    steps: i32,
    visited_at: HashMap<Position, i32>,
}

impl Wire {
    fn new() -> Wire {
        Wire {
            position: Position { x: 0, y: 0 },
            visited_at: HashMap::new(),
            steps: 0,
        }
    }

    fn move_right(&mut self) {
        self.position.x += 1;
        self.steps += 1;
        self.visited_at.insert(self.position, self.steps);
    }

    fn move_left(&mut self) {
        self.position.x -= 1;
        self.steps += 1;
        self.visited_at.insert(self.position, self.steps);
    }

    fn move_up(&mut self) {
        self.position.y += 1;
        self.steps += 1;
        self.visited_at.insert(self.position, self.steps);
    }

    fn move_down(&mut self) {
        self.position.y -= 1;
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

        assert_eq!(first.distance_from(&second), 200);
    }

    //19031 too high
    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).expect("");

        assert_eq!(solve_part_one(&input), 1983);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).expect("");

        assert_eq!(solve_part_two(&input), 107754);
    }
}
