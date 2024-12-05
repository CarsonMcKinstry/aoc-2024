use std::f32::consts::E;

use regex::{Captures, Match, Regex};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY THREE =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let line_length = matrix.first().map(|f| f.len()).unwrap();
    let column_length = matrix.len();

    let mut total: u32 = 0;

    for y in 0..column_length {
        for x in 0..line_length {
            let current_character = matrix[y][x].clone();
            let can_north = y >= 3;
            let can_west = x >= 3;
            let can_south = y < column_length - 3;
            let can_east = x < line_length - 3;

            let mut strings: Vec<String> = vec![];

            // nw
            if can_west && can_north {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y - 1][x - 1].clone(),
                        matrix[y - 2][x - 2].clone(),
                        matrix[y - 3][x - 3].clone(),
                    ]
                    .join(""),
                );
            }
            // n
            if can_north {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y - 1][x].clone(),
                        matrix[y - 2][x].clone(),
                        matrix[y - 3][x].clone(),
                    ]
                    .join(""),
                );
            }
            // ne
            if can_north && can_east {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y - 1][x + 1].clone(),
                        matrix[y - 2][x + 2].clone(),
                        matrix[y - 3][x + 3].clone(),
                    ]
                    .join(""),
                );
            }
            // e
            if can_east {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y][x + 1].clone(),
                        matrix[y][x + 2].clone(),
                        matrix[y][x + 3].clone(),
                    ]
                    .join(""),
                );
            }
            // se
            if can_south && can_east {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y + 1][x + 1].clone(),
                        matrix[y + 2][x + 2].clone(),
                        matrix[y + 3][x + 3].clone(),
                    ]
                    .join(""),
                );
            }

            // s
            if can_south {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y + 1][x].clone(),
                        matrix[y + 2][x].clone(),
                        matrix[y + 3][x].clone(),
                    ]
                    .join(""),
                );
            }
            // sw
            if can_south && can_west {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y + 1][x - 1].clone(),
                        matrix[y + 2][x - 2].clone(),
                        matrix[y + 3][x - 3].clone(),
                    ]
                    .join(""),
                );
            }
            // w
            if can_west {
                strings.push(
                    vec![
                        current_character.clone(),
                        matrix[y][x - 1].clone(),
                        matrix[y][x - 2].clone(),
                        matrix[y][x - 3].clone(),
                    ]
                    .join(""),
                );
            }

            total += strings
                .iter()
                .filter(|chars| **chars == String::from("XMAS"))
                .count() as u32;
        }
    }

    total
}

fn part_two(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let line_length = matrix.first().map(|f| f.len()).unwrap();
    let column_length = matrix.len();

    let mut total: u32 = 0;

    for y in 0..column_length - 2 {
        for x in 0..line_length - 2 {
            let tl = matrix[y][x].as_str();
            let tr = matrix[y][x + 2].as_str();
            let c = matrix[y + 1][x + 1].as_str();
            let bl = matrix[y + 2][x].as_str();
            let br = matrix[y + 2][x + 2].as_str();

            let forward_slash = format!("{tl}{c}{br}");
            let back_slash = format!("{tr}{c}{bl}");

            let mas = String::from("MAS");
            let sam = String::from("SAM");

            if (forward_slash == mas || forward_slash == sam)
                && (back_slash == mas || back_slash == sam)
            {
                total += 1
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_example_returns_the_correct_answer() {
        let expected = 18;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 9;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
