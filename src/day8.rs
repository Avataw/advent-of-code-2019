use std::fs;

const PATH: &str = "src/day8.txt";

pub fn solve() {
    let input: String = fs::read_to_string(PATH).unwrap();
    println!(
        "Day 8: \n a) {} \n b) \n{}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}

fn solve_part_one(input: &str) -> usize {
    let layers = parse(input, 25, 6);

    let target_layer = find_fewest_zeroes(&layers);

    target_layer.count('1') * target_layer.count('2')
}

fn solve_part_two(input: &str) -> String {
    let layers = parse(input, 25, 6);

    let decoded = decode(&layers, 25 * 6);

    let digits: Vec<char> = decoded.chars().collect();

    let message: Vec<String> = digits.chunks(25).map(|d| d.iter().collect()).collect();

    message.join("\n")
}

fn parse(input: &str, width: usize, height: usize) -> Vec<Layer> {
    let chunk_size: usize = width * height;

    let digits: Vec<char> = input.chars().collect();

    digits
        .chunks(chunk_size)
        .map(|pixels| Layer {
            pixels: pixels.to_vec(),
        })
        .collect()
}

fn find_fewest_zeroes(layers: &[Layer]) -> &Layer {
    layers
        .iter()
        .min_by(|a, b| a.count('0').cmp(&b.count('0')))
        .unwrap()
}

fn decode(layers: &[Layer], length: usize) -> String {
    let mut first_non_transparent_pixels: String = String::new();

    for i in 0..length {
        for layer in layers {
            match layer.pixels[i] {
                '2' => continue,
                '1' => first_non_transparent_pixels.push('⬜'),
                '0' => first_non_transparent_pixels.push('⬛'),
                _ => panic!("Pixel contains invalid color!"),
            }
            break;
        }
    }

    first_non_transparent_pixels
}

#[derive(PartialEq, Debug)]
struct Layer {
    pixels: Vec<char>,
}

impl Layer {
    fn count(&self, digit: char) -> usize {
        self.pixels.iter().filter(|p| **p == digit).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_to_layers() {
        let input = "123456789012";

        let image_width = 3;
        let image_height = 2;

        assert_eq!(parse(input, image_width, image_height).len(), 2);
    }

    #[test]
    fn should_count_digits() {
        let layers = parse("111156789012", 3, 2);

        assert_eq!(layers[0].count('0'), 0);
        assert_eq!(layers[1].count('0'), 1);
        assert_eq!(layers[0].count('1'), 4);
    }

    #[test]
    fn should_find_layer_with_fewest_zeroes() {
        let layers = parse("123456789012", 3, 2);

        assert_eq!(find_fewest_zeroes(&layers), &layers[0]);
    }

    #[test]
    fn should_first_non_transparent_pixels() {
        let layers = parse("0222112222120000", 2, 2);

        assert_eq!(decode(&layers, 4), "⬛⬜⬜⬛");
    }

    #[test]
    fn should_solve_part_one() {
        let input: String = fs::read_to_string(PATH).unwrap();

        assert_eq!(solve_part_one(&input), 1330);
    }

    #[test]
    fn should_solve_part_two() {
        let input: String = fs::read_to_string(PATH).unwrap();

        let result: String = "
        ⬜⬜⬜⬜⬛⬛⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬜⬜⬛⬜⬜⬜⬜⬛
        ⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛
        ⬜⬜⬜⬛⬛⬜⬛⬛⬜⬛⬜⬜⬜⬜⬛⬜⬜⬜⬛⬛⬜⬜⬜⬛⬛
        ⬜⬛⬛⬛⬛⬜⬜⬜⬜⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛
        ⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬛⬛⬛⬛⬜⬛⬛⬛⬛
        ⬜⬛⬛⬛⬛⬜⬛⬛⬜⬛⬜⬛⬛⬜⬛⬜⬜⬜⬜⬛⬜⬛⬛⬛⬛"
            .split_whitespace()
            .collect();

        assert_eq!(solve_part_two(&input).replace('\n', ""), result);
    }
}
