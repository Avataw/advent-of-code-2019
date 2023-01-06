use std::{
    collections::{HashMap, HashSet},
    fs,
};

const PATH: &str = "src/day6.txt";

pub fn solve() -> () {
    let input: String = fs::read_to_string(PATH).expect("");
    println!(
        "Day 6 a) : {} b) : {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn solve_part_one(input: &str) -> i32 {
    let objects = parse(input);
    calc_distances(&objects).values().sum()
}

fn solve_part_two(input: &str) -> i32 {
    let objects = parse(input);
    let distances = calc_distances(&objects);

    let path_to_you = find_path(&objects, "YOU");
    let path_to_santa = find_path(&objects, "SAN");

    let intersection = find_closest_intersection(path_to_you, path_to_santa, &distances);

    let you_orbit_to_intersection = distances.get("YOU").unwrap() - intersection - 1;
    let santa_orbit_to_intersection = distances.get("SAN").unwrap() - intersection - 1;

    you_orbit_to_intersection + santa_orbit_to_intersection
}

fn parse(input: &str) -> Vec<Object> {
    input
        .lines()
        .map(|l| {
            let mut orbiting_object = l.split(")").map(|s| String::from(s));
            Object {
                orbits: orbiting_object.next().unwrap(),
                name: orbiting_object.next().unwrap(),
            }
        })
        .collect()
}

fn calc_distances(objects: &Vec<Object>) -> HashMap<String, i32> {
    let mut distances = HashMap::from([(String::from("COM"), 0)]);
    while distances.len() <= objects.len() {
        for object in objects {
            if distances.contains_key(&object.name) {
                continue;
            }

            match distances.get(&object.orbits) {
                Some(distance) => distances.insert(object.name.clone(), distance + 1),
                None => continue,
            };
        }
    }
    distances
}

fn find_path(objects: &Vec<Object>, target: &str) -> HashSet<String> {
    let mut path: HashSet<String> = HashSet::new();
    let mut curr = objects.iter().find(|o| o.name == target).unwrap();

    loop {
        path.insert(curr.name.clone());

        if curr.orbits == "COM" {
            break;
        };

        curr = objects.iter().find(|o| o.name == curr.orbits).unwrap();
    }

    path
}

fn find_closest_intersection(
    first_path: HashSet<String>,
    second_path: HashSet<String>,
    distances: &HashMap<String, i32>,
) -> &i32 {
    first_path
        .intersection(&second_path)
        .map(|i| distances.get(i).unwrap())
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Object {
    name: String,
    orbits: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_correctly() {
        let input = "COM)B\nB)C";

        let objects = parse(input);

        assert_eq!(objects[0].name, "B");
        assert_eq!(objects[0].orbits, "COM");
        assert_eq!(objects.len(), 2);
    }

    #[test]
    fn should_all_distances_to_com() {
        let input = "COM)B\nB)C";

        let objects = parse(input);
        let distances = calc_distances(&objects);

        assert_eq!(
            distances,
            HashMap::from([
                (String::from("COM"), 0),
                (String::from("B"), 1),
                (String::from("C"), 2)
            ])
        );
    }

    #[test]
    fn should_find_all_objects_from_com_to_target() {
        let input = "COM)B\nB)C";

        let objects = parse(input);
        let path = find_path(&objects, "C");

        assert_eq!(path, HashSet::from([String::from("C"), String::from("B")]));
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).expect("");

        assert_eq!(solve_part_one(&input), 253104);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).expect("");

        assert_eq!(solve_part_two(&input), 499);
    }
}
